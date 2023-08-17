use anyhow::{self, Result};
use sqlx::PgPool;
use tracing::debug;
use uuid::Uuid;

use crate::database;

/// Error indicating that user validation failed
#[derive(Debug, thiserror::Error)]
pub(crate) enum CredentialValidationError {
    /// Error variant returned when user with provided id does not exist in the database.
    #[error("user with provided id does not exist")]
    NonexistentUser,
    /// Error variant returned when user's secret token doesn't match the one given.
    #[error("invalid secret token")]
    InvalidSecretToken,
    /// Database error
    #[error(transparent)]
    DatabaseError(#[from] anyhow::Error),
}

// TODO: make this proper tower middleware

/// Validates passed user credentials.
#[tracing::instrument(skip(pool))]
pub(crate) async fn validate_user_credentials(
    user_id: Uuid,
    user_secret_token: Uuid,
    pool: &PgPool,
) -> Result<(), CredentialValidationError> {
    debug!(?user_id, ?user_secret_token, "Validating user credentials");

    match database::get_user_secret_token(user_id, pool).await? {
        Some(token) => {
            if user_secret_token == token {
                debug!("User credentials are valid");
                Ok(())
            } else {
                debug!(?user_secret_token, "Invalid user secret token");
                Err(CredentialValidationError::InvalidSecretToken)
            }
        }
        None => {
            debug!(?user_id, "User with given id does not exist");
            Err(CredentialValidationError::NonexistentUser)
        }
    }
}
