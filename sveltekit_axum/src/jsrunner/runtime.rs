use std::rc::Rc;
use std::time::Duration;

use deno_core::{
    error::{generic_error, AnyError},
    op2, v8, FastString, JsRuntime, ModuleCodeBytes, ModuleLoadResponse, ModuleSource,
    ModuleSpecifier, ModuleType, RuntimeOptions,
};
use tokio::time::sleep;

use crate::axum_compat::axum_compat;
use crate::StaticFiles;

pub fn setup_runtime(server_js: StaticFiles) -> JsRuntime {
    let mut options = RuntimeOptions::default();

    options.module_loader = Some(Rc::new(StaticModuleLoader { files: server_js }));
    // options.module_loader = Some(Rc::new(deno_core::FsModuleLoader));

    // the order is important for initialisation
    options.extensions = vec![
        axum_compat::init_ops_and_esm(),
        runtime_bootstrap::init_ops_and_esm(),
        deno_console::deno_console::init_ops_and_esm(),
        deno_webidl::deno_webidl::init_ops_and_esm(),
        deno_url::deno_url::init_ops_and_esm(),
        deno_web::deno_web::init_ops_and_esm::<AllowHrtime>(Default::default(), None),
        deno_net::deno_net::init_ops_and_esm::<DisallowNet>(None, None),
        deno_fetch::deno_fetch::init_ops_and_esm::<DisallowFetch>(Default::default()),
        deno_crypto::deno_crypto::init_ops_and_esm(None),
    ];

    JsRuntime::new(options)
}

#[derive(Debug)]
struct AllowHrtime;
impl deno_web::TimersPermission for AllowHrtime {
    fn allow_hrtime(&mut self) -> bool {
        true
    }
}

#[derive(Debug)]
struct DisallowNet;
impl deno_net::NetPermissions for DisallowNet {
    fn check_net<T: AsRef<str>>(
        &mut self,
        _host: &(T, Option<u16>),
        _api_name: &str,
    ) -> Result<(), AnyError> {
        Err(deno_core::error::not_supported())
    }
    fn check_read(&mut self, _p: &str, _api_name: &str) -> Result<std::path::PathBuf, AnyError> {
        Err(deno_core::error::not_supported())
    }
    fn check_write(&mut self, _p: &str, _api_name: &str) -> Result<std::path::PathBuf, AnyError> {
        Err(deno_core::error::not_supported())
    }
    fn check_write_path<'a>(
        &mut self,
        _p: &'a std::path::Path,
        _api_name: &str,
    ) -> Result<std::borrow::Cow<'a, std::path::Path>, AnyError> {
        Err(deno_core::error::not_supported())
    }
}

struct DisallowFetch;
impl deno_fetch::FetchPermissions for DisallowFetch {
    fn check_net_url(
        &mut self,
        _url: &deno_core::url::Url,
        _api_name: &str,
    ) -> Result<(), deno_core::error::AnyError> {
        Err(deno_core::error::not_supported())
    }

    fn check_read<'a>(
        &mut self,
        _p: &'a std::path::Path,
        _api_name: &str,
    ) -> Result<std::borrow::Cow<'a, std::path::Path>, AnyError> {
        Err(deno_core::error::not_supported())
    }
}

#[op2(fast)]
fn op_log(#[string] message: &str, #[number] level: i64) {
    use tracing::{debug, error, info, warn};

    let message = message.trim_end();

    match level {
        0 => debug!("{}", message),
        1 => info!("{}", message),
        2 => warn!("{}", message),
        3 => error!("{}", message),
        _ => warn!("Invalid log level from js!"),
    }
}

#[op2(async)]
async fn op_sleep(#[number] millis: u64) {
    sleep(Duration::from_millis(millis)).await
}

deno_core::extension!(
  runtime_bootstrap,
  ops = [op_log, op_sleep],
  esm_entry_point = "ext:runtime_bootstrap/bootstrap.js",
  esm = [dir "src/jsrunner", "bootstrap.js"]
);

pub async fn load_module_get_export(
    runtime: &mut JsRuntime,
    module: &str,
    export: FastString,
) -> anyhow::Result<v8::Global<v8::Function>> {
    let module_id = runtime
        .load_main_es_module(&ModuleSpecifier::parse(module)?)
        .await?;

    // run initialisation code
    runtime.mod_evaluate(module_id).await?;

    let namespace = runtime.get_module_namespace(module_id)?;

    // get local handle to module
    let mut scope = runtime.handle_scope();
    let namespace = v8::Local::new(&mut scope, namespace);

    let key = export.v8_string(&mut scope);

    // get reference to handler function
    let handler: v8::Local<'_, v8::Function> = namespace
        .get(&mut scope, key.cast())
        .ok_or(generic_error("module missing requested export"))?
        .try_cast()?;
    Ok(v8::Global::new(&mut scope, handler))
}

#[derive(Debug, Clone)]
pub(crate) struct StaticModuleLoader {
    files: StaticFiles,
}

impl deno_core::ModuleLoader for StaticModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<ModuleSpecifier, anyhow::Error> {
        Ok(deno_core::resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        let path = module_specifier.path().trim_start_matches("/");
        let content = self.files.get_file(path);

        let Some(content) = content else {
            return ModuleLoadResponse::Sync(Err(anyhow::anyhow!(
                "Failed to find requested JS module '{}'",
                module_specifier
            )));
        };

        ModuleLoadResponse::Sync(Ok(ModuleSource::new(
            ModuleType::JavaScript,
            deno_core::ModuleSourceCode::Bytes(ModuleCodeBytes::Static(content.contents())),
            module_specifier,
            None,
        )))
    }
}
