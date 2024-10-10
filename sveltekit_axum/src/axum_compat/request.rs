use std::pin::Pin;
use std::rc::Rc;

use axum::{
    body::{Body, BodyDataStream, HttpBody},
    extract::Request,
    http::{HeaderMap, HeaderValue},
};
use deno_core::{
    futures::{stream::Peekable, StreamExt},
    AsyncRefCell, BufView, JsRuntime, RcRef,
};
use serde_v8::ByteString;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestRep(
    // Body read_stream_rid:
    deno_core::ResourceId,
    // method:
    String,
    // url:
    String,
    // headers:
    Vec<(ByteString, ByteString)>,
);

pub fn request_to_rep(req: Request, runtime: &mut JsRuntime) -> anyhow::Result<RequestRep> {
    let (parts, body) = req.into_parts();

    let localhost = HeaderValue::from_static("localhost");
    let host = parts.headers.get(hyper::header::HOST).unwrap_or(&localhost);

    let req_body_rid = runtime
        .op_state()
        .borrow_mut()
        .resource_table
        .add_rc(Rc::new(RequestBodyResource::new(body)));

    let request_rep = RequestRep(
        req_body_rid,
        parts.method.to_string(),
        // Js Request needs host to be valid, so just kinda guess for now
        format!("http://{}{}", host.to_str()?, parts.uri),
        req_headers(&parts.headers),
    );

    Ok(request_rep)
}

// based on https://github.com/denoland/deno/blob/main/ext/http/request_body.rs
#[derive(Debug)]
struct RequestBodyResource {
    body: AsyncRefCell<Peekable<BodyDataStream>>,
    size_hint: (u64, Option<u64>),
}

impl RequestBodyResource {
    pub fn new(body: Body) -> Self {
        let size_hint = body.size_hint();
        let stream = body.into_data_stream().peekable();
        Self {
            body: AsyncRefCell::new(stream),
            size_hint: (size_hint.lower(), size_hint.upper()),
        }
    }

    async fn read(self: Rc<Self>, limit: usize) -> Result<BufView, deno_core::error::AnyError> {
        let peekable = RcRef::map(self, |this| &this.body);
        let mut peekable = peekable.borrow_mut().await;
        match Pin::new(&mut *peekable).peek_mut().await {
            None => Ok(BufView::empty()),
            Some(Err(_)) => Err(peekable.next().await.unwrap().err().unwrap().into()),
            Some(Ok(bytes)) => {
                if bytes.len() <= limit {
                    // We can safely take the next item since we peeked it
                    return Ok(BufView::from(peekable.next().await.unwrap()?));
                }
                let ret = bytes.split_to(limit);
                Ok(BufView::from(ret))
            }
        }
    }
}

impl deno_core::Resource for RequestBodyResource {
    fn name(&self) -> std::borrow::Cow<str> {
        "axum::BodyDataStream".into()
    }

    fn read(self: Rc<Self>, limit: usize) -> deno_core::AsyncResult<BufView> {
        Box::pin(RequestBodyResource::read(self, limit))
    }

    fn size_hint(&self) -> (u64, Option<u64>) {
        self.size_hint
    }
}

// https://github.com/denoland/deno/blob/779a98cd39b781091427e68b1548d4f3189a8595/ext/http/lib.rs#L625
fn req_headers(header_map: &HeaderMap<HeaderValue>) -> Vec<(ByteString, ByteString)> {
    // We treat cookies specially, because we don't want them to get them
    // mangled by the `Headers` object in JS. What we do is take all cookie
    // headers and concat them into a single cookie header, separated by
    // semicolons.
    let cookie_sep = "; ".as_bytes();
    let mut cookies = vec![];

    let mut headers = Vec::with_capacity(header_map.len());
    for (name, value) in header_map.iter() {
        if name == hyper::header::COOKIE {
            cookies.push(value.as_bytes());
        } else {
            let name: &[u8] = name.as_ref();
            let value = value.as_bytes();
            headers.push((name.into(), value.into()));
        }
    }

    if !cookies.is_empty() {
        headers.push(("cookie".into(), cookies.join(cookie_sep).into()));
    }

    headers
}
