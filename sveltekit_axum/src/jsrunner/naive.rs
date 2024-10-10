use std::time::Instant;

use axum::body::Body;
use axum::response::Response;
use deno_core::ascii_str;
use deno_core::v8;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::task::LocalSet;
use tracing::info;

use super::{
    runtime::{load_module_get_export, setup_runtime},
    JsRenderTask, ResponseRep,
};

/// This wrapper for JsRuntime achieves better latencies, but can't process multiple
/// request in parallel.
#[derive(Debug, Clone)]
pub struct JsRenderSpawnerNaive {
    send: mpsc::UnboundedSender<JsRenderTask>,
}

impl JsRenderSpawnerNaive {
    pub fn new() -> Self {
        let (send, recv) = mpsc::unbounded_channel();

        std::thread::spawn(move || {
            Self::start_js_thread(recv);
        });

        Self { send }
    }

    fn start_js_thread(mut recv: UnboundedReceiver<JsRenderTask>) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let local = LocalSet::new();

        local.spawn_local(async move {
            let start = Instant::now();

            let mut runtime = setup_runtime(include_dir::Dir::new("", &[]));

            // load server module
            let handler = load_module_get_export(
                &mut runtime,
                "file:///home/konsti/programs/webframework/testapp/build/index.js",
                ascii_str!("handler").into(),
            )
            .await
            .unwrap();

            info!("JsRuntime init took: {}us", start.elapsed().as_micros());

            while let Some((req, res_sender)) = recv.recv().await {
                let req_rep = {
                    let mut scope = runtime.handle_scope();

                    let local = serde_v8::to_v8(
                        &mut scope,
                        RequestRep {
                            uri: req.uri().to_string(),
                            method: req.method().to_string(),
                            // TODO: this is broken
                            body_stream_rid: 0,
                        },
                    )
                    .unwrap();
                    v8::Global::new(&mut scope, local)
                };

                let call_fut = runtime.call_with_args(&handler, &[req_rep]);

                let render_res = runtime
                    .with_event_loop_future(call_fut, Default::default())
                    .await
                    .unwrap();

                let mut scope = runtime.handle_scope();
                let local = v8::Local::new(&mut scope, render_res);

                let res_rep = serde_v8::from_v8::<ResponseRep>(&mut scope, local).unwrap();

                let body = Body::from(res_rep.body.to_vec());
                let res = Response::new(body);

                // the client requesting the page might already be gone, so ignore send errors
                // TODO: possibly detect this and abort rendering
                let _ = res_sender.send(res);
            }
        });

        rt.block_on(local);
    }

    pub fn spawn(&self, task: JsRenderTask) {
        self.send
            .send(task)
            .expect("Thread with LocalSet has shut down.");
    }
}
