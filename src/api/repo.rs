use crate::model::domain::Record;
use crate::model::error::AppError;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn read_records(&self) -> Result<Vec<Record>, AppError>;
    async fn read_record(&self, id: i32) -> Result<Record, AppError>;
    async fn read_record_by_name(&self, name: String) -> Result<Record, AppError>;
    async fn save_record(&self, record: &Record) -> Result<(), AppError>;
    async fn update_record(&self, record: &Record) -> Result<(), AppError>;
    async fn delete_record(&self, id: i32) -> Result<(), AppError>;
}
