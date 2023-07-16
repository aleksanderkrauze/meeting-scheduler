pub mod models;

use anyhow::{Result, Context};
use sqlx::PgPool;
use tracing::debug;
use uuid::Uuid;

pub async fn get_meeting_by_id(id: Uuid, pool: &PgPool) -> Result<Option<models::Meeting>> {
    debug!(id = ?id, "Queering meeting from database");

    let meeting: Option<models::Meeting> = 
        sqlx::query_as("SELECT id, name, description, created_at, expires_at, user_data_id from meeting WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .with_context(|| format!("Failed to query meeting from database"))?;

    debug!(meeting = ?meeting, "Received meeting from database");

    Ok(meeting)
}
