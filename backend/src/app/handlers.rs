use anyhow::Context;
use axum::{extract::State, http::StatusCode, response::Html};
use sqlx::Row;
use tracing::{error, info};

use super::AppState;

#[axum_macros::debug_handler]
#[tracing::instrument(skip(app_state))]
pub async fn homepage(State(app_state): State<AppState>) -> Result<Html<String>, StatusCode> {
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
