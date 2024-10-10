use std::future::poll_fn;
use std::task::Poll;
use std::time::Instant;

use axum::{response::Response, Router};
use deno_core::{
    ascii_str,
    v8::{self, Global},
    JsRuntime,
};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver},
    oneshot::Sender,
};
use tokio::task::{JoinSet, LocalSet};
use tracing::info;

use crate::{
    axum_compat::{request::request_to_rep, response::response_from_js},
    StaticFiles,
};

use super::{
    runtime::{load_module_get_export, setup_runtime},
    JsRenderTask,
};

#[derive(Debug, Clone)]
pub struct JsRenderSpawnerParallel {
    send: mpsc::UnboundedSender<JsRenderTask>,
}

impl JsRenderSpawnerParallel {
    pub fn new(server_js: StaticFiles, core_router: Router) -> Self {
        let (send, recv) = mpsc::unbounded_channel();

        std::thread::spawn(move || {
            start_js_thread(recv, server_js, core_router);
        });

        Self { send }
    }

    pub fn spawn(&self, task: JsRenderTask) {
        self.send
            .send(task)
            .expect("Thread with LocalSet has shut down.");
    }
}

fn start_js_thread(
    mut recv: UnboundedReceiver<JsRenderTask>,
    server_js: StaticFiles,
    _core_router: Router,
) {
    // Do not init JsRuntime at all in debug mode
    if cfg!(debug_assertions) {
        return;
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let local = LocalSet::new();

    local.spawn_local(async move {
        let start = Instant::now();

        let mut runtime = setup_runtime(server_js);

        // load server module
        let handler = load_module_get_export(
            &mut runtime,
            "file:///entry.js",
            ascii_str!("handler").into(),
        )
        .await
        .unwrap();

        info!("JsRuntime init took: {}us", start.elapsed().as_micros());

        let mut set: JoinSet<(anyhow::Result<v8::Global<v8::Value>>, Sender<Response>)> =
            JoinSet::new();

        poll_fn(|cx| {
            // poll for finished renders
            while let Poll::Ready(Some(t)) = set.poll_join_next(cx) {
                let (render_res, res_sender) = t.unwrap();

                let response = response_from_js(render_res.unwrap(), &mut runtime);

                // the client requesting the page might already be gone, so ignore send errors
                // TODO: possibly detect this and abort rendering
                let _ = res_sender.send(response.unwrap());
            }

            // poll for new renders
            while let Poll::Ready(Some((req, res_sender))) = recv.poll_recv(cx) {
                let request_rep = request_to_rep(req, &mut runtime).unwrap();

                info!("Calling Js handler entrypoint");

                let scope = &mut runtime.handle_scope();
                let request_rep = serde_v8::to_v8(scope, request_rep).unwrap();
                let request_rep = Global::new(scope, request_rep);
                let call_fut = JsRuntime::scoped_call_with_args(scope, &handler, &[request_rep]);

                //? Sometimes this future resolves immediatly because it is only executing synchronus code.
                //? Maybe a shortcut can be added here.

                set.spawn_local(async move { (call_fut.await, res_sender) });
            }

            // poll JsRuntime event loop
            // this needs to be polled after join set and recv, because otherwise this system somehow deadlocks
            if let Poll::Ready(t) = runtime.poll_event_loop(cx, Default::default()) {
                _ = t;
            }

            Poll::<()>::Pending
        })
        .await;
    });

    rt.block_on(local);
}
