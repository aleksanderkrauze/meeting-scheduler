mod config;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Result};
use axum::{extract::State, http::StatusCode, response::Html, routing::get, Router, Server};
use dotenvy::dotenv;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Row,
};

use config::Config;

#[derive(Debug, Clone)]
struct AppState {
    config: Arc<Config>,
    database_pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();

    let config = Config::from_env()?;

    let database_pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(2))
        .connect(&config.postgres_uri())
        .await
        .context("failed to create db connection pool")?;

    let app_state = AppState {
        config: Arc::clone(&config),
        database_pool,
    };

    let app = Router::new()
        .route("/", get(homepage))
        .with_state(app_state);

    Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        config.server_port,
    ))
    .serve(app.into_make_service())
    .await?;

    Ok(())
}

#[axum_macros::debug_handler]
async fn homepage(State(app_state): State<AppState>) -> Result<Html<String>, (StatusCode, String)> {
    let results = sqlx::query("SELECT * FROM meeting")
        .fetch_all(&app_state.database_pool)
        .await
        .context("fetching from database failed")
        .map_err(internal_error)?
        .into_iter()
        .map::<String, _>(|row| row.get("name"));

    let mut body = String::from("<h1>Meetings:</h1><ul>");
    for meeting in results {
        body.push_str("<li>");
        body.push_str(&meeting);
        body.push_str("</li>");
    }
    body.push_str("</ul>");

    Ok(Html(body))
}

fn internal_error(err: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#}", err))
}
