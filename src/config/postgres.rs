use crate::model::error::AppError;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_host: String,
    pub db_port: u32,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
}

impl Config {
    pub fn new_from_env() -> Result<Self> {
        envy::from_env()
            .map_err(|e| AppError::ApplicationError(format!("error reading config {:?}", e)).into())
    }

    pub fn url(&self) -> String {
        format!(
            "host={} port={} dbname={} user={} password={}",
            self.db_host, self.db_port, self.db_name, self.db_user, self.db_password
        )
    }
}
