mod parallel;
mod runtime;

pub use parallel::JsRenderSpawnerParallel;

type JsRenderTask = (
    axum::extract::Request,
    tokio::sync::oneshot::Sender<axum::response::Response>,
);
