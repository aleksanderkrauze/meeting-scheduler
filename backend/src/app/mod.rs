mod handlers;

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::{routing::get, Router, Server};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::{info, warn};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) config: Arc<Config>,
    pub(crate) database_pool: PgPool,
}

pub async fn run_server(config: Arc<Config>) -> Result<(), anyhow::Error> {
    let database_pool = db_pool_connect(Arc::clone(&config)).await;

    let app_state = AppState {
        config: Arc::clone(&config),
        database_pool,
    };

    let app = Router::new()
        .route("/meeting/:uuid", get(handlers::get_meeting_by_id))
        .with_state(app_state);

    info!("Starting server");
    let address = config.server_socket_addr();
    Server::try_bind(&address)
        .with_context(|| format!("failed to bind server to address {:?}", address))?
        .serve(app.into_make_service())
        .await
        .context("Failed to start server")?;

    Ok(())
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
