use axum::{body::BoxBody, http::Request, response::Response};
use std::time::Duration;
use tracing::{info, Span};

pub fn request_processor(request: &Request<hyper::Body>, _span: &Span) {
    let path = request.uri().path();
    let method = request.method().to_string();

    info!(method, path, "incoming request");
}
pub fn response_processor(response: &Response<BoxBody>, latency: Duration, _span: &Span) {
    let status = response.status().to_string();

    info!(status, duration = latency.as_micros(), "outgoing response");
}
