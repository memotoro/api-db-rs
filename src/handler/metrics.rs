use crate::model::error::AppError;
use axum::{http::StatusCode, response::IntoResponse};
use lazy_static::lazy_static;
use prometheus::{register_int_counter, Encoder, IntCounter, TextEncoder};
use tracing::error;

lazy_static! {
    static ref REQUEST_COUNTER: IntCounter =
        register_int_counter!("request_counter", "Number of request").unwrap();
}

pub async fn process_metrics() -> Result<impl IntoResponse, AppError> {
    let encoder = TextEncoder::new();

    let mut buffer = vec![];

    let _ = encoder
        .encode(&prometheus::gather(), &mut buffer)
        .map_err(|e| {
            error!("{}", e);
            AppError::ApplicationError("failed to encode metrics".to_string())
        });

    let response = String::from_utf8(buffer.clone()).map_err(|e| {
        error!("{}", e);
        AppError::ApplicationError("failed to convert bytes to string".to_string())
    });

    buffer.clear();

    Ok((StatusCode::OK, response))
}

pub fn request_counter() {
    REQUEST_COUNTER.inc();
}
