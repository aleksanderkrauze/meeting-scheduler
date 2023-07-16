use std::env::var;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use anyhow::{Context, Result};
use tracing::info;

#[derive(Debug, Clone)]
pub struct Config {
    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_url: String,
    pub postgres_port: u16,
    pub postgres_db: String,
    pub server_addr: IpAddr,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Arc<Self>> {
        info!("Creating configuration from env");

        let postgres_user = var("POSTGRES_USER").context("missing env variable POSTGRES_USER")?;
        let postgres_password =
            var("POSTGRES_PASSWORD").context("missing env variable POSTGRES_PASSWORD")?;
        let postgres_url = var("POSTGRES_URL").context("missing env variable POSTGRES_URL")?;
        let postgres_port = var("POSTGRES_PORT")
            .context("missing env variable POSTGRES_PORT")?
            .parse()
            .context("failed to parse POSTGRES_PORT as u16")?;
        let postgres_db = var("POSTGRES_DB").context("missing env variable POSTGRES_DB")?;
        let server_addr = var("SERVER_ADDR")
            .context("missing env variable SERVER_ADDR")?
            .parse()
            .context("failed to parse SERVER_ADDR as IP address")?;
        let server_port = var("SERVER_PORT")
            .context("missing env variable SERVER_PORT")?
            .parse()
            .context("failed to parse SERVER_PORT as u16")?;

        let config = Config {
            postgres_user,
            postgres_password,
            postgres_url,
            postgres_port,
            postgres_db,
            server_addr,
            server_port,
        };

        Ok(Arc::new(config))
    }

    pub fn postgres_uri(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_url,
            self.postgres_port,
            self.postgres_db
        )
    }

    pub fn server_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.server_addr, self.server_port)
    }
}
