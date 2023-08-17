use anyhow::{anyhow, Context};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tracing::info;
use uuid::Uuid;

use super::{business_logic, AppState};
use crate::api::input::{CreateMeetingData, JoinMeetingData, PostCommentData};
use crate::api::output::{CreatedMeeting, JoinMeetingResponse, Meeting};
use crate::app::middleware;
use crate::database;

#[axum_macros::debug_handler]
#[tracing::instrument(skip(app_state))]
pub(crate) async fn get_meeting_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Meeting>, StatusCode> {
    info!(meeting_id=?id, "Getting meeting info");

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
    info!(meeting_data=?data, "Creating new meeting");

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

#[axum_macros::debug_handler]
#[tracing::instrument(skip(app_state))]
pub(crate) async fn join_meeting(
    State(app_state): State<AppState>,
    Path(meeting_id): Path<Uuid>,
    Json(data): Json<JoinMeetingData>,
) -> Result<(StatusCode, Json<JoinMeetingResponse>), StatusCode> {
    info!(?meeting_id, join_meeting_data=?data, "Creating new meeting participant");

    let user = business_logic::User::new(data.name).map_err(bad_request)?;

    if let Err(error) = database::join_meeting(&user, meeting_id, &app_state.database_pool).await {
        match error {
            database::JoinMeetingError::NonexistentMeeting(_) => {
                return Err(bad_request(error.into()))
            }
            database::JoinMeetingError::Database(err) => return Err(internal_error(err)),
        }
    }

    let response = JoinMeetingResponse {
        id: user.id,
        secret_token: user.secret_token,
    };
    info!(?response, "Created new meeting participant");
    Ok((StatusCode::CREATED, Json(response)))
}

#[axum_macros::debug_handler]
#[tracing::instrument(skip(app_state))]
pub(crate) async fn post_comment(
    State(app_state): State<AppState>,
    Path(meeting_id): Path<Uuid>,
    Json(data): Json<PostCommentData>,
) -> Result<StatusCode, StatusCode> {
    info!(?meeting_id, comment_data=?data, "Posting new comment to meeting");

    let PostCommentData {
        user_id,
        user_token,
        message,
    } = data;

    if !database::meeting_exists(meeting_id, &app_state.database_pool)
        .await
        .map_err(internal_error)?
    {
        info!(?meeting_id, "Meeting with given id does not exist");
        return Err(StatusCode::NOT_FOUND);
    }

    if let Err(error) =
        middleware::validate_user_credentials(user_id, user_token, &app_state.database_pool).await
    {
        match error {
            middleware::CredentialValidationError::NonexistentUser => {
                info!(?user_id, "Unauthorized");
                return Err(StatusCode::UNAUTHORIZED);
            }
            middleware::CredentialValidationError::InvalidSecretToken => {
                info!(?user_id, "Forbidden");
                return Err(StatusCode::FORBIDDEN);
            }
            middleware::CredentialValidationError::DatabaseError(err) => {
                return Err(internal_error(err));
            }
        }
    }

    let meeting_comment =
        business_logic::MeetingComment::new(user_id, meeting_id, message).map_err(bad_request)?;
    database::post_comment(&meeting_comment, &app_state.database_pool)
        .await
        .map_err(internal_error)?;

    info!(?meeting_comment, "Meeting comment was added to database");
    Ok(StatusCode::CREATED)
}

fn internal_error(err: anyhow::Error) -> StatusCode {
    info!(error = ?err, "Internal error");
    StatusCode::INTERNAL_SERVER_ERROR
}

fn not_found_error(err: anyhow::Error) -> StatusCode {
    info!(error = ?err, "Not found");
    StatusCode::NOT_FOUND
}

fn bad_request(err: anyhow::Error) -> StatusCode {
    info!(error = ?err, "Bad request");
    StatusCode::BAD_REQUEST
}
