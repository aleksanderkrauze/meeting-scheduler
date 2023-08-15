#![allow(dead_code)]

use std::fmt;

use serde::ser::Error as _;
use serde::{de, Deserializer, Serialize, Serializer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const RFC3339_FORMAT: Rfc3339 = Rfc3339;

struct Rfc3339OffsetDateTimeVisitor;

impl<'de> de::Visitor<'de> for Rfc3339OffsetDateTimeVisitor {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a RFC 3339 formatted Datetime")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        OffsetDateTime::parse(v, &RFC3339_FORMAT).map_err(E::custom)
    }
}

pub(crate) fn serialize<S: Serializer>(
    date: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    date.format(&RFC3339_FORMAT)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

pub(crate) fn deserialize<'a, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<OffsetDateTime, D::Error> {
    deserializer.deserialize_str(Rfc3339OffsetDateTimeVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use time::macros::datetime;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Foo(#[serde(with = "super")] OffsetDateTime);

    #[test]
    fn serialize() {
        #[rustfmt::skip]
        let foo = Foo(datetime!(2023-07-24 12:34:56.123456 UTC));

        let s = serde_json::to_string(&foo).unwrap();
        assert_eq!(s, r#""2023-07-24T12:34:56.123456Z""#);
    }

    #[test]
    fn deserialize_utc() {
        let str = [
            r#""2023-07-24T12:34:56.123456Z""#,
            r#""2023-07-24T12:34:56.123456+00:00""#,
            r#""2023-07-24T12:34:56.123456-00:00""#,
        ];
        #[rustfmt::skip]
        let expected = Foo(datetime!(2023-07-24 12:34:56.123456 UTC));

        for time in str {
            let actual: Foo = serde_json::from_str(time).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn deserialize() {
        let str = r#""2023-07-24T12:34:56.123456+07:00""#;
        #[rustfmt::skip]
        let expected = Foo(datetime!(2023-07-24 12:34:56.123456 +7));

        let actual: Foo = serde_json::from_str(str).unwrap();
        assert_eq!(expected, actual);
    }
}
