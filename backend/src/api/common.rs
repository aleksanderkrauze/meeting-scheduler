use serde::{Deserialize, Serialize};

use crate::database::models;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Vote {
    No,
    Maybe,
    Yes,
}

impl From<models::Vote> for Vote {
    fn from(value: models::Vote) -> Self {
        match value {
            models::Vote::No => Self::No,
            models::Vote::Maybe => Self::Maybe,
            models::Vote::Ok => Self::Yes,
        }
    }
}
