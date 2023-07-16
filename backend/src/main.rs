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
use tracing::{debug, error, info, warn};

use config::Config;

#[derive(Debug, Clone)]
struct AppState {
    config: Arc<Config>,
    database_pool: PgPool,
}

async fn db_pool_connect(config: Arc<Config>) -> PgPool {
    let uri = config.postgres_uri();
    let timeout = Duration::from_secs(5);
    info!("Connecting to database");
    loop {
        let pool = PgPoolOptions::new()
            .max_connections(64)
            .acquire_timeout(timeout)
            .connect(&uri)
            .await;
        match pool {
            Ok(pool) => return pool,
            Err(e) => warn!("Connecting to database failed: `{:?}`. Trying again...", e),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    info!("Loading .env file");
    let _ = dotenv();

    info!("Creating configuration from env");
    let config = Config::from_env()?;

    let database_pool = db_pool_connect(Arc::clone(&config)).await;

    let app_state = AppState {
        config: Arc::clone(&config),
        database_pool,
    };

    let app = Router::new()
        .route("/", get(homepage))
        .with_state(app_state);

    info!("Starting server");
    Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        config.server_port,
    ))
    .serve(app.into_make_service())
    .await?;

    Ok(())
}

#[axum_macros::debug_handler]
#[tracing::instrument(skip(app_state))]
async fn homepage(State(app_state): State<AppState>) -> Result<Html<String>, StatusCode> {
    info!("Running homepage handler");

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

fn internal_error(err: anyhow::Error) -> StatusCode {
    error!("Error: {:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR
}
