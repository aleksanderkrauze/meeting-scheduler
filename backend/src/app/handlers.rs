use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tracing::info;
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
    info!("Getting meeting info");

    let meeting_info = database::get_meeting_info(id, &app_state.database_pool)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| anyhow!("No meeting with provided id"))
        .map_err(not_found_error)?;

    let meeting_comments = database::get_meeting_comments(id, &app_state.database_pool)
        .await
        .map_err(internal_error)?;

    let participants_proposed_dates_votes =
        database::get_meeting_participants_proposed_dates_votes(id, &app_state.database_pool)
            .await
            .map_err(internal_error)?;

    Ok(Json(
        Meeting::new(
            meeting_info,
            meeting_comments,
            participants_proposed_dates_votes,
        )
        .map_err(internal_error)?,
    ))
}

fn internal_error(err: anyhow::Error) -> StatusCode {
    info!(error = ?err, "Internal error");
    StatusCode::INTERNAL_SERVER_ERROR
}

fn not_found_error(err: anyhow::Error) -> StatusCode {
    info!(error = ?err, "Not found error");
    StatusCode::NOT_FOUND
}
