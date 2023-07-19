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

pub mod serde_date {
    use serde::ser::Error as _;
    use serde::{Serialize, Serializer};
    use time::{format_description::FormatItem, macros::format_description, Date};

    static DATE_FORMATTER: &[FormatItem] = format_description!("[year]-[month]-[day]");

    pub fn serialize<S: Serializer>(date: &Date, serializer: S) -> Result<S::Ok, S::Error> {
        date.format(&DATE_FORMATTER)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
}
