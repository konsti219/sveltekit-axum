use axum::{
    routing::{any, post},
    Extension, Json, Router,
};
use tower_http::trace;
use tracing::{info, Level};

use sveltekit_axum::jsrunner::JsRenderSpawnerParallel as JsRunner;
use sveltekit_axum::{release_include_dir, sveltekit_handler, SveltekitData};

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let core_router = core_router();

    let sveltekit_data = SveltekitData {
        js_runner: JsRunner::new(
            release_include_dir!("$SVELTEKIT_BUILD/server"),
            core_router.clone(),
        ),

        client_js: release_include_dir!("$SVELTEKIT_BUILD/client"),
        prerendered: release_include_dir!("$SVELTEKIT_BUILD/prerendered"),

        static_dir: "./static",
    };

    // Do not add more routers here. Extend core_router instead.
    let app = Router::new()
        .merge(core_router)
        .fallback(any(sveltekit_handler))
        .layer(Extension(sveltekit_data))
        .layer(
            trace::TraceLayer::new_for_http()
                .on_request(trace::DefaultOnRequest::new().level(Level::TRACE))
                .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG))
                .on_failure(trace::DefaultOnFailure::new().level(Level::WARN)),
        );

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

fn core_router() -> Router {
    // more routers can be added here
    Router::new().nest("/api", api_router())
}

fn api_router() -> Router {
    Router::new().route("/hello", post(hello).get(|| async { "hello" }))
}

async fn hello(Json(data): Json<TestData>) -> Json<TestData> {
    let mut value = data.value;
    value.push_str(" is cool");
    println!("got '{}'", value);

    Json(TestData { value })
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TestData {
    value: String,
}
