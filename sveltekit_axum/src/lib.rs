use std::mem;

use axum::{
    body::Body,
    extract::{Extension, Request},
    http::{header, HeaderValue, Method, StatusCode},
    response::Response,
};
use tower::Service;
use tower_http::services::ServeDir;

pub mod axum_compat;
pub mod jsrunner;
use jsrunner::JsRenderSpawnerParallel as JsRunner;

pub type StaticFiles = include_dir::Dir<'static>;

#[derive(Debug, Clone)]
pub struct SveltekitData {
    pub js_runner: JsRunner,

    pub client_js: StaticFiles,
    pub prerendered: StaticFiles,

    pub static_dir: &'static str,
}

/// Handler for any request that did not match any axum routes.
/// First we check if it any static file availble.
/// Then we pass it to SvelteKit to render.
/// If no route is matched then SvelteKit renders a 404 page.
pub async fn sveltekit_handler(
    Extension(mut state): Extension<SveltekitData>,
    mut req: Request,
) -> Response {
    if req.method() == Method::GET || req.method() == Method::HEAD {
        if let Some(res) = serve_static_files(&mut req, &mut state).await {
            return res;
        }
    }

    // This also handles all 404s
    // TODO maybe expose 404 handling to users

    #[cfg(debug_assertions)]
    {
        let mut rev = reverse_proxy_service::builder_http("localhost:5173")
            .unwrap()
            .build(reverse_proxy_service::Identity);
        let res = rev.call(req).await.unwrap().unwrap();
        let (parts, incoming) = res.into_parts();

        Response::from_parts(parts, Body::new(incoming))
    }

    #[cfg(not(debug_assertions))]
    {
        let (send, response) = tokio::sync::oneshot::channel();
        state.js_runner.spawn((req, send));
        response.await.unwrap()
    }
}

async fn serve_static_files(req: &mut Request, state: &mut SveltekitData) -> Option<Response> {
    let stripped_path = req.uri().path().trim_start_matches("/");

    // check if file can be found in /client
    if let Some(file) = state.client_js.get_file(stripped_path) {
        return Some(build_static_file_response(file, req.method()));
    }

    // check if file can be found in /prerendered
    let mut html_path = stripped_path.to_owned();
    if html_path == "" {
        html_path = String::from("index");
    }
    html_path.push_str(".html");
    if let Some(file) = state.prerendered.get_file(&html_path) {
        return Some(Response::new(Body::from(file.contents())));
    }

    // finally try serving from ./static in FS
    let mut static_dir = ServeDir::new(state.static_dir);
    let res = static_dir
        .call(clone_request_without_body(req))
        .await
        .unwrap();
    if res.status() != StatusCode::NOT_FOUND {
        let (parts, body) = res.into_parts();
        let body = Body::new(body);
        return Some(Response::from_parts(parts, body));
    }

    None
}

fn build_static_file_response(
    file: &'static include_dir::File<'static>,
    method: &Method,
) -> Response {
    let mime_header = mime_guess::from_path(file.path())
        .first_raw()
        .map(HeaderValue::from_static)
        .unwrap_or_else(|| HeaderValue::from_str(mime::APPLICATION_OCTET_STREAM.as_ref()).unwrap());

    let builder = Response::builder().header(header::CONTENT_TYPE, mime_header);
    // .header(header::ACCEPT_RANGES, "bytes");

    // if let Some(encoding) = output
    //     .maybe_encoding
    //     .filter(|encoding| *encoding != Encoding::Identity)
    // {
    //     builder = builder.header(header::CONTENT_ENCODING, encoding.into_header_value());
    // }

    // if let Some(last_modified) = output.last_modified {
    //     builder = builder.header(header::LAST_MODIFIED, last_modified.0.to_string());
    // }

    let body = if method == Method::GET {
        Body::from(file.contents())
    } else {
        Body::empty()
    };

    builder
        .header(header::CONTENT_LENGTH, file.contents().len().to_string())
        .body(body)
        .unwrap()
}

fn clone_request_without_body(req: &mut Request) -> Request {
    let (parts, body) = mem::take(req).into_parts();
    let cloned_req = Request::from_parts(parts.clone(), Body::empty());
    *req = Request::from_parts(parts, body);
    cloned_req
}

#[doc(hidden)]
pub use include_dir;

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! release_include_dir {
    ($path: tt) => {
        $crate::include_dir::include_dir!($path)
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! release_include_dir {
    ($_path: tt) => {
        $crate::include_dir::Dir::new("", &[])
    };
}
