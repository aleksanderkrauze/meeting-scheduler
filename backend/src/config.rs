use std::env::var;
use std::sync::Arc;

use anyhow::{Context, Result};

pub struct Config {
    postgres_user: String,
    postgres_password: String,
    postgres_db: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Arc<Self>> {
        let postgres_user = var("POSTGRES_USER").context("missing env variable POSTGRES_USER")?;
        let postgres_password =
            var("POSTGRES_PASSWORD").context("missing env variable POSTGRES_PASSWORD")?;
        let postgres_db = var("POSTGRES_DB").context("missing env variable POSTGRES_DB")?;
        let server_port = var("SERVER_PORT")
            .context("missing env variable SERVER_PORT")?
            .parse()
            .context("failed to parse SERVER_PORT as u16")?;

        let config = Config {
            postgres_user,
            postgres_password,
            postgres_db,
            server_port,
        };

        Ok(Arc::new(config))
    }
}
