use sqlx::FromRow;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub(crate) struct MeetingInfo {
    /// Name of the meeting
    pub(crate) name: String,
    /// Description of the meeting
    pub(crate) description: Option<String>,
    /// Id of the user that created the meeting
    pub(crate) created_by: Uuid,
    /// Date and time of meeting creation
    pub(crate) created_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct MeetingComment {
    /// Comment message
    pub(crate) message: String,
    /// Id of the user that posted the comment
    pub(crate) written_by: Uuid,
    /// Date and time of posting comment
    pub(crate) posted_at: OffsetDateTime,
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "proposed_date_vote")]
#[sqlx(rename_all = "lowercase")]
pub(crate) enum Vote {
    No,
    Maybe,
    Ok,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct ParticipantsProposedDatesVotes {
    /// User id. May be NULL if this row contains date that no one has voted on.
    pub(crate) user_id: Option<Uuid>,
    /// User name. May be NULL <=> user_id is NULL
    pub(crate) name: Option<String>,
    /// Date id. May be NULL if this row contains user that has not voted on any date.
    pub(crate) date_id: Option<Uuid>,
    /// Date. May be NULL <=> date_id is NULL
    pub(crate) date: Option<Date>,
    /// Vote. May be NULL <=> date_id is NULL.
    pub(crate) vote: Option<Vote>,
    /// Optional vote comment. May be NOT NULL <=> date_id is NOT NULL
    pub(crate) comment: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct UserSecretToken {
    /// Secret token of given user
    pub(crate) secret_token: Uuid,
}

impl UserSecretToken {
    pub(crate) fn into_token(self) -> Uuid {
        self.secret_token
    }
}
