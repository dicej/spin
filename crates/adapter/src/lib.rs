#![deny(warnings)]

wit_bindgen_rust::import!("../../wit/ephemeral/spin-http.wit");
wit_bindgen_rust::export!("../../wit/ephemeral/wasi-outbound-http.wit");
wit_bindgen::generate!("spin" in "../../wit/preview2/spin.wit");

use {http_types as ht, spin_http as sh, wasi_outbound_http as woh};

impl From<woh::Method> for ht::Method {
    fn from(method: woh::Method) -> Self {
        use woh::Method::*;

        match method {
            Get => Self::Get,
            Post => Self::Post,
            Put => Self::Put,
            Delete => Self::Delete,
            Patch => Self::Patch,
            Head => Self::Head,
            Options => Self::Options,
        }
    }
}

impl From<ht::Method> for sh::Method {
    fn from(method: ht::Method) -> Self {
        use ht::Method::*;

        match method {
            Get => Self::Get,
            Post => Self::Post,
            Put => Self::Put,
            Delete => Self::Delete,
            Patch => Self::Patch,
            Head => Self::Head,
            Options => Self::Options,
        }
    }
}

impl From<ht::Response> for woh::Response {
    fn from(res: ht::Response) -> Self {
        Self {
            status: res.status,
            headers: res.headers,
            body: res.body,
        }
    }
}

impl From<sh::Response> for ht::Response {
    fn from(res: sh::Response) -> Self {
        Self {
            status: res.status,
            headers: res.headers,
            body: res.body,
        }
    }
}

impl From<ht::HttpError> for woh::HttpError {
    fn from(error: ht::HttpError) -> Self {
        use ht::HttpError::*;

        match error {
            Success => Self::Success,
            DestinationNotAllowed => Self::DestinationNotAllowed,
            InvalidUrl => Self::InvalidUrl,
            RequestError => Self::RequestError,
            RuntimeError => Self::RuntimeError,
            TooManyRequests => Self::TooManyRequests,
        }
    }
}

impl<'a> From<ht::RequestParam<'a>> for sh::Request<'a> {
    fn from(req: ht::RequestParam<'a>) -> Self {
        Self {
            method: req.method.into(),
            uri: req.uri,
            headers: req.headers,
            params: req.params,
            body: req.body,
        }
    }
}

fn into(v: &[(String, String)]) -> Vec<(&str, &str)> {
    v.iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect::<Vec<_>>()
}

struct WasiOutboundHttp;

impl woh::WasiOutboundHttp for WasiOutboundHttp {
    fn request(req: woh::Request) -> Result<woh::Response, woh::HttpError> {
        outbound_http::send_request(ht::RequestParam {
            method: req.method.into(),
            uri: &req.uri,
            headers: &into(&req.headers),
            params: &into(&req.params),
            body: req.body.as_deref(),
        })
        .map(Into::into)
        .map_err(Into::into)
    }
}

struct InboundHttp;

impl inbound_http::InboundHttp for InboundHttp {
    fn handle_request(req: ht::RequestResult) -> ht::Response {
        sh::handle_http_request(sh::Request {
            method: req.method.into(),
            uri: &req.uri,
            headers: &into(&req.headers),
            params: &into(&req.params),
            body: req.body.as_deref(),
        })
        .into()
    }
}

export_spin!(InboundHttp);
