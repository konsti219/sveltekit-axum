use std::future::ready;
use std::rc::Rc;
use std::task::Poll;

use axum::{
    body::{Body, Bytes},
    http::{HeaderName, HeaderValue, StatusCode},
    response::Response,
};
use deno_core::{
    ascii_str,
    error::AnyError,
    futures::Stream,
    v8::{self, Function, Global, Local, Value},
    BufView, JsRuntime,
};
use serde::{Deserialize, Serialize};
use serde_v8::ByteString;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use super::get_axum_compat;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResponseRep(
    // status code
    u16,
    // headers:
    Vec<(ByteString, ByteString)>,
);

pub fn response_from_js(res: Global<Value>, runtime: &mut JsRuntime) -> Result<Response, AnyError> {
    let (sender, receiver): (_, UnboundedReceiver<Bytes>) = mpsc::unbounded_channel();

    let res_body_rid = runtime
        .op_state()
        .borrow_mut()
        .resource_table
        .add_rc(Rc::new(ResponseBodyResource(sender)));

    let axum_compat = get_axum_compat(runtime)?;

    let mut scope = runtime.handle_scope();
    let axum_comapt = axum_compat.open(&mut scope);

    let key = ascii_str!("responseToRep").v8_string(&mut scope);
    let response_from_rep: Local<'_, Function> =
        axum_comapt.get(&mut scope, key.cast()).unwrap().cast();
    let this = v8::undefined(&mut scope).into();

    let res = Local::new(&mut scope, res);
    let res_body_rid = serde_v8::to_v8(&mut scope, res_body_rid).unwrap();

    let response_rep = response_from_rep
        .call(&mut scope, this, &[res, res_body_rid])
        .unwrap();

    let response_rep: ResponseRep = serde_v8::from_v8(&mut scope, response_rep).unwrap();

    let body = Body::from_stream(ReceiverStream(receiver));

    let mut response = Response::new(body);
    *response.status_mut() = StatusCode::from_u16(response_rep.0).unwrap();
    for (header_name, header_value) in response_rep.1 {
        response.headers_mut().append(
            HeaderName::from_bytes(&header_name).unwrap(),
            HeaderValue::from_bytes(&header_value).unwrap(),
        );
    }

    Ok(response)
}

#[derive(Debug)]
struct ReceiverStream<T>(UnboundedReceiver<T>);
impl<T> Stream for ReceiverStream<T> {
    type Item = Result<T, std::convert::Infallible>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.0.poll_recv(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(value) => Poll::Ready(value.map(|v| Ok(v))),
        }
    }
}

#[derive(Debug)]
struct ResponseBodyResource(UnboundedSender<Bytes>);
impl deno_core::Resource for ResponseBodyResource {
    fn name(&self) -> std::borrow::Cow<str> {
        "axum::Body".into()
    }

    fn write(self: Rc<Self>, buf: BufView) -> deno_core::AsyncResult<deno_core::WriteOutcome> {
        // TODO: optimze by implementing a custom http_body::Body which wraps an UnboundedReceiver<BufView>
        let raw_buf: Vec<u8> = buf.to_owned();
        let len = raw_buf.len();

        if let Err(_err) = self.0.send(Bytes::from(raw_buf)) {
            return Box::pin(ready(Err(anyhow::anyhow!(
                "Failed to send response stream buffer."
            ))));
        }
        Box::pin(ready(Ok(deno_core::WriteOutcome::Full { nwritten: len })))
    }

    fn write_sync(self: Rc<Self>, data: &[u8]) -> Result<usize, anyhow::Error> {
        self.0.send(Bytes::from(data.to_owned()))?;
        Ok(data.len())
    }
}
