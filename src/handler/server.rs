use crate::api::repo::Repository;
use crate::model::domain::Record;
use crate::model::error::AppError;
use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

pub async fn all_records(
    State(repo): State<Arc<dyn Repository>>,
) -> Result<impl IntoResponse, AppError> {
    let records = repo.read_records().await?;

    Ok((StatusCode::OK, Json(records)))
}

pub async fn record_by_id(
    State(repo): State<Arc<dyn Repository>>,
    Path(params): Path<RecordParams>,
) -> Result<impl IntoResponse, AppError> {
    let record = repo.read_record(params.id).await?;

    Ok((StatusCode::OK, Json(record)))
}

pub async fn create_record(
    State(repo): State<Arc<dyn Repository>>,
    Json(request): Json<RecordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let name = request.name;

    match repo.read_record_by_name(name.clone()).await {
        Ok(r) => {
            return Err(AppError::RecordFound(format!(
                "record with name {} already exist",
                r.name
            )))
        }
        Err(e) => match e {
            AppError::RecordNotFound(_) => {}
            _ => return Err(e),
        },
    };

    let mut record = Record { id: 0, name };

    repo.save_record(&record).await?;

    record = repo.read_record_by_name(record.name).await?;

    Ok((StatusCode::CREATED, Json(record)))
}

pub async fn update_record(
    State(repo): State<Arc<dyn Repository>>,
    Path(params): Path<RecordParams>,
    Json(request): Json<RecordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut record = repo.read_record(params.id).await?;

    record.name = request.name;

    repo.update_record(&record).await?;

    Ok((StatusCode::OK, Json(record)))
}

pub async fn delete_record(
    State(repo): State<Arc<dyn Repository>>,
    Path(params): Path<RecordParams>,
) -> Result<impl IntoResponse, AppError> {
    let record = repo.read_record(params.id).await?;

    repo.delete_record(record.id).await?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RecordParams {
    id: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RecordRequest {
    name: String,
}
