pub(crate) mod business_logic;
pub(crate) mod handlers;

use std::{sync::Arc, time::Duration};

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router, Server,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::config::Config;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    #[allow(dead_code)]
    pub(crate) config: Arc<Config>,
    pub(crate) database_pool: PgPool,
}

pub async fn run_server(
    config: Arc<Config>,
    cancellation_token: CancellationToken,
) -> Result<(), anyhow::Error> {
    let database_pool = match db_pool_connect(Arc::clone(&config), cancellation_token.clone()).await
    {
        Some(pool) => pool,
        None => return Ok(()),
    };

    let app_state = AppState {
        config: Arc::clone(&config),
        database_pool,
    };

    let app = Router::new()
        .route("/meeting", post(handlers::create_meeting))
        .route("/meeting/:uuid", get(handlers::get_meeting_by_id))
        .route("/meeting/:uuid/join", post(handlers::join_meeting))
        .with_state(app_state);

    let address = config.server_socket_addr();
    info!(?address, "Starting server");
    Server::try_bind(&address)
        .with_context(|| format!("failed to bind server to address {:?}", address))?
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move {
            cancellation_token.cancelled().await;
            info!("Server received cancellation signal. Starting gracefull shutdown");
        })
        .await
        .context("Failed to start server")?;

    Ok(())
}

async fn db_pool_connect(
    config: Arc<Config>,
    cancellation_token: CancellationToken,
) -> Option<PgPool> {
    let uri = config.postgres_uri();
    let timeout = Duration::from_secs(5);
    info!("Connecting to database");
    loop {
        if cancellation_token.is_cancelled() {
            info!("Received cancellation request. Abandoning database pool connection");
            return None;
        }

        let pool = PgPoolOptions::new()
            .max_connections(64)
            .acquire_timeout(timeout)
            .connect(&uri)
            .await;
        match pool {
            Ok(pool) => return Some(pool),
            Err(e) => warn!("Connecting to database failed: `{:?}`. Trying again...", e),
        }
    }
}
