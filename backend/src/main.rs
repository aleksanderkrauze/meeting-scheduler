mod config;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{routing::get, Router, Server};
use dotenvy::dotenv;

use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();

    let config = Config::from_env()?;

    let app = Router::new().route("/", get(|| async { "Hello world!" }));

    Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        config.server_port,
    ))
    .serve(app.into_make_service())
    .await?;

    Ok(())
}
