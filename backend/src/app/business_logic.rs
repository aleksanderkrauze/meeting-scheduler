use anyhow::{anyhow, Result};
use time::{ext::NumericalDuration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub(crate) id: Uuid,
    pub(crate) secret_token: Uuid,
    pub(crate) name: String,
}

impl User {
    pub fn new(name: String) -> Result<Self> {
        if name.is_empty() {
            return Err(anyhow!("name is empty").context("failed to validate name"));
        }

        let id = Uuid::new_v4();
        let secret_token = Uuid::new_v4();

        Ok(Self {
            id,
            secret_token,
            name,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Meeting {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) created_at: OffsetDateTime,
    pub(crate) expires_at: OffsetDateTime,
    pub(crate) user_id: Uuid,
}

impl Meeting {
    pub fn new(name: String, description: Option<String>, user_id: Uuid) -> Result<Self> {
        if name.is_empty() {
            return Err(anyhow!("name is empty").context("failed to validate name"));
        }
        if let Some(ref description) = description {
            if description.is_empty() {
                return Err(anyhow!("description is set to empty string")
                    .context("failed to validate description"));
            }
        }

        let id = Uuid::new_v4();
        let created_at = OffsetDateTime::now_utc();
        let offset = 14.days();
        let expires_at = created_at + offset;

        Ok(Self {
            id,
            name,
            description,
            created_at,
            expires_at,
            user_id,
        })
    }
}
