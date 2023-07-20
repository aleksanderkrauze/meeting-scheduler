use anyhow::{anyhow, bail, Result};
use serde::Serialize;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use crate::database::models;

use super::common::Vote;

enum ValidatedParticipantsProposedDatesVotes {
    Participant {
        user_id: Uuid,
        name: String,
    },
    ProposedDate {
        date_id: Uuid,
        date: Date,
    },
    Vote {
        user_id: Uuid,
        name: String,
        date_id: Uuid,
        date: Date,
        vote: Vote,
        comment: Option<String>,
    },
}

impl TryFrom<models::ParticipantsProposedDatesVotes> for ValidatedParticipantsProposedDatesVotes {
    type Error = anyhow::Error;

    fn try_from(value: models::ParticipantsProposedDatesVotes) -> Result<Self, Self::Error> {
        use models::ParticipantsProposedDatesVotes as PPDV;

        let create_base_error = |value| anyhow!("failed to validate row: {:?}", value);

        match (value.user_id, value.date_id) {
            (None, None) => Err(create_base_error(value)),

            (Some(user_id), None) => {
                let name = if let Some(name) = value.name {
                    name
                } else {
                    return Err(create_base_error(value));
                };
                Ok(Self::Participant { user_id, name })
            }
            (None, Some(date_id)) => {
                let date = value.date.ok_or_else(|| create_base_error(value))?;
                Ok(Self::ProposedDate { date_id, date })
            }
            (Some(user_id), Some(date_id)) => match (value.name, value.date, value.vote) {
                (Some(name), Some(date), Some(vote)) => Ok(Self::Vote {
                    user_id,
                    name,
                    date_id,
                    date,
                    vote: vote.into(),
                    comment: value.comment,
                }),
                (name, _, vote) => Err(create_base_error(PPDV {
                    name,
                    vote,
                    ..value
                })),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Participant {
    id: Uuid,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ProposedDate {
    id: Uuid,
    #[serde(with = "super::common::serde_date")]
    date: Date,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ParticipantVote {
    participant_id: Uuid,
    date_id: Uuid,
    vote: Vote,
    comment: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MeetingInfo {
    /// Name of the meeting
    pub(crate) name: String,
    /// Description of the meeting
    pub(crate) description: Option<String>,
    /// Id of the user that created the meeting
    pub(crate) created_by: Uuid,
    /// Date and time of meeting creation
    #[serde(with = "time::serde::iso8601")]
    pub(crate) created_at: OffsetDateTime,
}

impl From<models::MeetingInfo> for MeetingInfo {
    fn from(value: models::MeetingInfo) -> Self {
        let models::MeetingInfo {
            name,
            description,
            created_by,
            created_at,
        } = value;
        Self {
            name,
            description,
            created_by,
            created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MeetingComment {
    /// Comment message
    pub(crate) message: String,
    /// Id of the user that posted the comment
    pub(crate) written_by: Uuid,
    /// Date and time of posting comment
    #[serde(with = "time::serde::iso8601")]
    pub(crate) posted_at: OffsetDateTime,
}

impl From<models::MeetingComment> for MeetingComment {
    fn from(value: models::MeetingComment) -> Self {
        let models::MeetingComment {
            message,
            written_by,
            posted_at,
        } = value;
        Self {
            message,
            written_by,
            posted_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Meeting {
    #[serde(flatten)]
    meeting_info: MeetingInfo,
    comments: Vec<MeetingComment>,
    participants: Vec<Participant>,
    proposed_dates: Vec<ProposedDate>,
    votes: Vec<ParticipantVote>,
}

impl Meeting {
    pub fn new(
        meeting_info: models::MeetingInfo,
        comments: Vec<models::MeetingComment>,
        participants_proposed_dates_votes: Vec<models::ParticipantsProposedDatesVotes>,
    ) -> Result<Self> {
        let meeting_info = meeting_info.into();
        let model_comments = comments;
        let mut comments = Vec::with_capacity(model_comments.len());
        comments.extend(model_comments.into_iter().map(Into::into));

        let mut participants = Vec::new();
        let mut proposed_dates = Vec::new();
        let mut votes = Vec::new();

        for row in participants_proposed_dates_votes {
            match row.try_into()? {
                ValidatedParticipantsProposedDatesVotes::Participant { user_id, name } => {
                    let participant = Participant { id: user_id, name };
                    if !participants.contains(&participant) {
                        participants.push(participant);
                    }
                }
                ValidatedParticipantsProposedDatesVotes::ProposedDate { date_id, date } => {
                    let proposed_date = ProposedDate { id: date_id, date };
                    if !proposed_dates.contains(&proposed_date) {
                        proposed_dates.push(proposed_date);
                    }
                }
                ValidatedParticipantsProposedDatesVotes::Vote {
                    user_id,
                    name,
                    date_id,
                    date,
                    vote,
                    comment,
                } => {
                    let participant = Participant { id: user_id, name };
                    let proposed_date = ProposedDate { id: date_id, date };
                    let participant_vote = ParticipantVote {
                        participant_id: user_id,
                        date_id,
                        vote,
                        comment,
                    };

                    if !participants.contains(&participant) {
                        participants.push(participant);
                    }
                    if !proposed_dates.contains(&proposed_date) {
                        proposed_dates.push(proposed_date);
                    }
                    if votes.contains(&participant_vote) {
                        bail!("vote already in list: {:?}", vote);
                    }
                    votes.push(participant_vote);
                }
            }
        }

        Ok(Self {
            meeting_info,
            comments,
            participants,
            proposed_dates,
            votes,
        })
    }
}
