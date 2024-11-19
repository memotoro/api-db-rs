use crate::handler::metrics::request_counter;
use axum::{body::BoxBody, http::Request, response::Response};
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tracing::{error, info, info_span, Span};

pub fn span_processor(request: &Request<hyper::Body>) -> Span {
    info_span!(
        "http_request",
        method = ?request.method(),
        path = ?request.uri().path()
    )
}

pub fn request_processor(_request: &Request<hyper::Body>, _span: &Span) {
    info!("incoming request");
    request_counter();
}

pub fn response_processor(response: &Response<BoxBody>, latency: Duration, _span: &Span) {
    let status = response.status().to_string();
    let duration = latency.as_micros();

    info!(status, duration, "outgoing response");
}

pub fn failure_processor(error: ServerErrorsFailureClass, latency: Duration, _span: &Span) {
    let duration = latency.as_micros();

    error!(duration, "application error {:?}", error);
}
