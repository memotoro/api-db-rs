use crate::model::error::AppError;
use anyhow::Result;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize, Debug)]
pub struct ApiConfig {
    pub port: u16,
}

impl ApiConfig {
    pub fn new_from_env() -> Result<Self> {
        envy::from_env().map_err(|e| {
            error!("{}", e);
            AppError::ApplicationError("error reading api config".to_string()).into()
        })
    }
}
