use crate::api::repo::Repository;
use crate::config::postgres::Config;
use crate::model::{domain::Record, error::AppError};
use anyhow::Result;
use async_trait::async_trait;
use tokio_postgres::{Client, NoTls, Row};

pub struct Postgres {
    client: Client,
}

impl Postgres {
    pub async fn new(config: &Config) -> Result<Self, AppError> {
        let (client, connection) = tokio_postgres::connect(config.url().as_str(), NoTls)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("error creating client and connection {}", e))
            })?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("connection error {}", e);
            }
        });

        Ok(Postgres { client })
    }
}

#[async_trait]
impl Repository for Postgres {
    async fn read_records(&self) -> Result<Vec<Record>, AppError> {
        let rows = self
            .client
            .query("SELECT id, name FROM records", &[])
            .await
            .map_err(|e| AppError::DatabaseError(format!("error reading all records {}", e)))?;

        let records = row_to_record(rows)?;

        Ok(records)
    }

    async fn read_record(&self, id: i32) -> Result<Record, AppError> {
        let rows = self
            .client
            .query("SELECT id, name FROM records WHERE id = $1", &[&id])
            .await
            .map_err(|e| AppError::DatabaseError(format!("error reading record by id {}", e)))?;

        let records = row_to_record(rows)?;

        if records.is_empty() {
            return Err(AppError::RecordNotFound(format!("id not found {}", id)));
        }

        let record = records
            .get(0)
            .ok_or::<AppError>(AppError::ApplicationError(
                "error reading record".to_string(),
            ))?;

        Ok(record.clone())
    }

    async fn read_record_by_name(&self, name: String) -> Result<Record, AppError> {
        let rows = self
            .client
            .query("SELECT id, name FROM records WHERE name = $1", &[&name])
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("error reading all record by name {}", e))
            })?;

        let records = row_to_record(rows)?;

        if records.is_empty() {
            return Err(AppError::RecordNotFound(format!("name not found {}", name)));
        }

        let record = records
            .get(0)
            .ok_or::<AppError>(AppError::ApplicationError(
                "error reading record".to_string(),
            ))?;

        Ok(record.clone())
    }

    async fn save_record(&self, record: &Record) -> Result<(), AppError> {
        let results = self
            .client
            .execute("INSERT INTO records (name) VALUES ($1)", &[&record.name])
            .await
            .map_err(|e| AppError::DatabaseError(format!("error saving record {}", e)))?;

        if results != 1 {
            return Err(AppError::ApplicationError(format!(
                "error reading records created {}",
                results
            )));
        }

        Ok(())
    }

    async fn update_record(&self, record: &Record) -> Result<(), AppError> {
        self.client
            .execute(
                "UPDATE records SET name = $1 WHERE id = $2",
                &[&record.name, &record.id],
            )
            .await
            .map_err(|e| AppError::DatabaseError(format!("error updating record {}", e)))?;

        Ok(())
    }

    async fn delete_record(&self, id: i32) -> Result<(), AppError> {
        self.client
            .execute("DELETE FROM records WHERE id = $1", &[&id])
            .await
            .map_err(|e| AppError::DatabaseError(format!("error deleting record {}", e)))?;

        Ok(())
    }
}

fn row_to_record(rows: Vec<Row>) -> Result<Vec<Record>, AppError> {
    let mut records: Vec<Record> = Vec::new();

    for row in rows.iter() {
        let id = row
            .try_get("id")
            .map_err(|e| AppError::DatabaseError(format!("error reading attribute id {}", e)))?;
        let name = row
            .try_get("name")
            .map_err(|e| AppError::DatabaseError(format!("error reading attribute name {}", e)))?;

        let record = Record { id, name };

        records.push(record);
    }

    Ok(records)
}
