pub mod models;

use anyhow::{Context, Result};
use sqlx::PgPool;
use tracing::debug;
use uuid::Uuid;

pub async fn get_meeting_info(id: Uuid, pool: &PgPool) -> Result<Option<models::MeetingInfo>> {
    let query = r#"
SELECT
    meeting.name, meeting.description, users.id AS created_by, meeting.created_at
FROM
    meeting
INNER JOIN users
    ON meeting.user_id = users.id
WHERE
    meeting.id = $1
"#;

    debug!(?id, "Queering meeting from database");
    let meeting = sqlx::query_as(query)
        .bind(id)
        .fetch_optional(pool)
        .await
        .context("Failed to query meeting from database")?;
    debug!(?meeting, "Received meeting from database");
    Ok(meeting)
}

pub async fn get_meeting_comments(id: Uuid, pool: &PgPool) -> Result<Vec<models::MeetingComment>> {
    let query = r#"
SELECT
    users.id AS written_by, meeting_comment.message, meeting_comment.posted_at
FROM
    meeting_comment
INNER JOIN users
    ON meeting_comment.user_id = users.id
WHERE
    meeting_comment.meeting_id = $1
ORDER BY
    meeting_comment.posted_at DESC
"#;

    debug!(?id, "Queering meeting comments from database");
    let comments = sqlx::query_as(query)
        .bind(id)
        .fetch_all(pool)
        .await
        .context("Failed to query meeting comments from database")?;
    debug!(?comments, "Received comments from database");
    Ok(comments)
}

pub async fn get_meeting_participants_proposed_dates_votes(
    id: Uuid,
    pool: &PgPool,
) -> Result<Vec<models::ParticipantsProposedDatesVotes>> {
    let query = r#"
SELECT
    users.id AS user_id,
    users.name,
    proposed_date.id AS date_id,
    proposed_date.date,
    proposed_date_user_votes.vote,
    proposed_date_user_votes.comment
FROM
    meeting_participants
INNER JOIN users
    ON meeting_participants.user_id = users.id
FULL OUTER JOIN proposed_date_user_votes
    ON meeting_participants.user_id = proposed_date_user_votes.user_id
FULL OUTER JOIN proposed_date
    ON proposed_date_user_votes.proposed_date_id = proposed_date.id
WHERE
    meeting_participants.meeting_id = $1 OR
    proposed_date.meeting_id = $1
"#;

    debug!(
        ?id,
        "Queering meeting participants, proposed dates and their votes"
    );

    sqlx::query_as(query)
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
}
