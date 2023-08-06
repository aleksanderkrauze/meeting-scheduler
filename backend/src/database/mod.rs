pub(crate) mod models;

use anyhow::{Context, Result};
use futures::future::TryFutureExt;
use sqlx::PgPool;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::app::business_logic;

pub(crate) async fn get_meeting_info(
    id: Uuid,
    pool: &PgPool,
) -> Result<Option<models::MeetingInfo>> {
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

pub(crate) async fn get_meeting_comments(
    id: Uuid,
    pool: &PgPool,
) -> Result<Vec<models::MeetingComment>> {
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

pub(crate) async fn get_meeting_participants_proposed_dates_votes(
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

pub(crate) async fn create_new_meeting(
    user: &business_logic::User,
    meeting: &business_logic::Meeting,
    pool: &PgPool,
) -> Result<()> {
    let insert_user_query = r#"
INSERT INTO
    users(id, secret_token, name)
VALUES
    ($1, $2, $3)
"#;
    let insert_meeting_query = r#"
INSERT INTO
    meeting(id, name, description, created_at, expires_at, user_id)
VALUES
    ($1, $2, $3, $4, $5, $6)
"#;
    let insert_meeting_participants_query = r#"
INSERT INTO
    meeting_participants(user_id, meeting_id)
VALUES
    ($1, $2)
"#;

    let insert_user = || async {
        sqlx::query(insert_user_query)
            .bind(user.id)
            .bind(user.secret_token)
            .bind(&user.name)
            .execute(pool)
            .await
            .context("failed to insert into users")
    };
    let insert_meeting = || async {
        sqlx::query(insert_meeting_query)
            .bind(meeting.id)
            .bind(&meeting.name)
            .bind(&meeting.description)
            .bind(meeting.created_at)
            .bind(meeting.expires_at)
            .bind(meeting.user_id)
            .execute(pool)
            .await
            .context("failed to insert into meeting")
    };
    let insert_meeting_participants = || async {
        sqlx::query(insert_meeting_participants_query)
            .bind(user.id)
            .bind(meeting.id)
            .execute(pool)
            .await
            .context("failed to insert into meeting_participants")
    };

    info!("starting transaction");
    let transaction = pool.begin().await.context("failed to begin transaction")?;
    if let Err(error) = insert_user()
        .and_then(|_| async { insert_meeting().await })
        .and_then(|_| async { insert_meeting_participants().await })
        .await
    {
        match transaction.rollback().await {
            Ok(_) => warn!(database_error = ?error, "rolled back transaction"),
            Err(rollback_error) => {
                error!(database_error = ?error, ?rollback_error, "failed to rollback transaction");
            }
        }

        Err(error)
    } else {
        transaction
            .commit()
            .await
            .context("failed to commit transaction")?;
        info!("commited transaction");
        Ok(())
    }
}

pub(crate) async fn join_meeting(
    user: &business_logic::User,
    meeting_id: Uuid,
    pool: &PgPool,
) -> Result<()> {
    let insert_user_query = r#"
INSERT INTO
    users(id, secret_token, name)
VALUES
    ($1, $2, $3)
"#;
    let insert_meeting_participants_query = r#"
INSERT INTO
    meeting_participants(user_id, meeting_id)
VALUES
    ($1, $2)
"#;

    let insert_user = || async {
        sqlx::query(insert_user_query)
            .bind(user.id)
            .bind(user.secret_token)
            .bind(&user.name)
            .execute(pool)
            .await
            .context("failed to insert into users")
    };
    let insert_meeting_participants = || async {
        sqlx::query(insert_meeting_participants_query)
            .bind(user.id)
            .bind(meeting_id)
            .execute(pool)
            .await
            .context("failed to insert into meeting_participants")
    };

    info!("starting transaction");
    let transaction = pool.begin().await.context("failed to begin transaction")?;
    if let Err(error) = insert_user()
        .and_then(|_| async { insert_meeting_participants().await })
        .await
    {
        match transaction.rollback().await {
            Ok(_) => warn!(database_error = ?error, "rolled back transaction"),
            Err(rollback_error) => {
                error!(database_error = ?error, ?rollback_error, "failed to rollback transaction");
            }
        }

        Err(error)
    } else {
        transaction
            .commit()
            .await
            .context("failed to commit transaction")?;
        info!("commited transaction");
        Ok(())
    }
}
