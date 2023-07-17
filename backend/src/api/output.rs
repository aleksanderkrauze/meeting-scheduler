use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Meeting {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub(crate) created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub(crate) expires_at: OffsetDateTime,
    pub(crate) created_by: Uuid,
}

// Temporary impl for quick testing
impl From<crate::database::models::Meeting> for Meeting {
    fn from(value: crate::database::models::Meeting) -> Self {
        Meeting {
            id: value.id,
            name: value.name,
            description: value.description,
            created_at: value.created_at,
            expires_at: value.expires_at,
            created_by: value.user_id,
        }
    }
}
