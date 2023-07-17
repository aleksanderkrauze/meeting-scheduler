use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tracing::{error, info};
use uuid::Uuid;

use super::AppState;
use crate::api::output::Meeting;
use crate::database;

#[axum_macros::debug_handler]
#[tracing::instrument(skip(app_state))]
pub async fn get_meeting_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Meeting>, StatusCode> {
    info!("Running handler for /meeting/:uuid");

    let meeting = database::get_meeting_by_id(id, &app_state.database_pool)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| anyhow!("No meeting with provided id"))
        .map_err(not_found_error)?;

    Ok(Json(meeting.into()))
}

fn internal_error(err: anyhow::Error) -> StatusCode {
    error!("Internal error: {:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR
}

fn not_found_error(err: anyhow::Error) -> StatusCode {
    error!("Not found error: {:?}", err);
    StatusCode::NOT_FOUND
}
