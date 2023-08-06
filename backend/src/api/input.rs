use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CreateMeetingData {
    pub(crate) meeting_name: String,
    pub(crate) meeting_description: Option<String>,
    pub(crate) user_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct JoinMeetingData {
    pub(crate) name: String,
}
