//! Glue code between Axum (tower) and Js

use deno_core::{
    error::AnyError,
    v8::{Global, Local, Object},
    JsRuntime,
};

pub mod request;
pub mod response;

deno_core::extension!(
  axum_compat,
  esm_entry_point = "ext:axum_compat/axum_compat.js",
  esm = [dir "src/axum_compat", "axum_compat.js", "response.js", "request.js"]
);

pub(crate) fn get_axum_compat(runtime: &mut JsRuntime) -> Result<Global<Object>, AnyError> {
    let axum_compat_global = runtime.execute_script("<anon>", "globalThis.AxumCompat")?;

    let mut scope = runtime.handle_scope();
    let local = Local::new(&mut scope, axum_compat_global);
    Ok(Global::new(&mut scope, local.cast()))
}
