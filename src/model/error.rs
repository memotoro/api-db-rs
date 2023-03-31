use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(thiserror::Error, Debug, Clone)]
pub enum AppError {
    #[error("record not found {0}")]
    RecordNotFound(String),
    #[error("record already exist {0}")]
    RecordFound(String),
    #[error("error with repository {0}")]
    DatabaseError(String),
    #[error("error with application {0}")]
    ApplicationError(String),
}

#[derive(Serialize, Clone, Debug)]
pub struct ErrorResponse {
    code: ErrorCode,
    message: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    DataError,
    RepositoryError,
    ApplicationError,
}

impl From<AppError> for ErrorCode {
    fn from(error: AppError) -> ErrorCode {
        match error {
            AppError::RecordNotFound(_) => ErrorCode::DataError,
            AppError::RecordFound(_) => ErrorCode::DataError,
            AppError::DatabaseError(_) => ErrorCode::RepositoryError,
            AppError::ApplicationError(_) => ErrorCode::ApplicationError,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::RecordNotFound(_) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    code: self.clone().into(),
                    message: self.to_string(),
                }),
            )
                .into_response(),
            AppError::RecordFound(_) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    code: self.clone().into(),
                    message: self.to_string(),
                }),
            )
                .into_response(),

            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: self.clone().into(),
                    message: self.to_string(),
                }),
            )
                .into_response(),
            AppError::ApplicationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: self.clone().into(),
                    message: self.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
