use anyhow::{anyhow, Context};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tracing::info;
use uuid::Uuid;

use super::{business_logic, AppState};
use crate::api::input::CreateMeetingData;
use crate::api::output::{CreatedMeeting, Meeting};
use crate::database;

#[axum_macros::debug_handler]
#[tracing::instrument(skip(app_state))]
pub(crate) async fn get_meeting_by_id(
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

#[axum_macros::debug_handler]
#[tracing::instrument(skip(app_state))]
pub(crate) async fn create_meeting(
    State(app_state): State<AppState>,
    Json(data): Json<CreateMeetingData>,
) -> Result<(StatusCode, Json<CreatedMeeting>), StatusCode> {
    info!("Creating new meeting");

    let user = business_logic::User::new(data.user_name)
        .context("failed to create user")
        .map_err(bad_request)?;
    let meeting =
        business_logic::Meeting::new(data.meeting_name, data.meeting_description, user.id)
            .context("failed to create meeting")
            .map_err(bad_request)?;

    database::create_new_meeting(&user, &meeting, &app_state.database_pool)
        .await
        .context("failed to insert new meeting into database")
        .map_err(internal_error)?;

    let response = CreatedMeeting {
        user_id: user.id,
        user_secret_token: user.secret_token,
        meeting_id: meeting.id,
    };
    info!(?response, "Created new meeting");
    Ok((StatusCode::CREATED, Json(response)))
}

fn internal_error(err: anyhow::Error) -> StatusCode {
    info!(error = ?err, "Internal error");
    StatusCode::INTERNAL_SERVER_ERROR
}

fn not_found_error(err: anyhow::Error) -> StatusCode {
    info!(error = ?err, "Not found error");
    StatusCode::NOT_FOUND
}

fn bad_request(err: anyhow::Error) -> StatusCode {
    info!(error = ?err, "Bad request");
    StatusCode::BAD_REQUEST
}
