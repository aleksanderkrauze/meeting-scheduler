#![allow(dead_code)]

use std::fmt;

use serde::ser::Error as _;
use serde::{de, Deserializer, Serialize, Serializer};
use time::{format_description::FormatItem, macros::format_description, Date};

const RFC3339_FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

struct Rfc3339DateVisitor;

impl<'de> de::Visitor<'de> for Rfc3339DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a RFC 3339 formatted Date")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Date::parse(v, &RFC3339_FORMAT).map_err(E::custom)
    }
}

pub(crate) fn serialize<S: Serializer>(date: &Date, serializer: S) -> Result<S::Ok, S::Error> {
    date.format(&RFC3339_FORMAT)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

pub(crate) fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Date, D::Error> {
    deserializer.deserialize_str(Rfc3339DateVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use time::macros::date;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Foo(#[serde(with = "super")] Date);

    #[test]
    fn serialize() {
        #[rustfmt::skip]
        let foo = Foo(date!(2023-07-24));

        let s = serde_json::to_string(&foo).unwrap();
        assert_eq!(s, r#""2023-07-24""#);
    }

    #[test]
    fn deserialize() {
        #[rustfmt::skip]
        let expected = Foo(date!(2023-07-24));

        assert_eq!(expected, serde_json::from_str(r#""2023-07-24""#).unwrap());
    }
}
