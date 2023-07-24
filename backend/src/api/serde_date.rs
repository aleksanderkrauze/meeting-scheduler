use std::fmt;

use serde::ser::Error as _;
use serde::{de, Deserializer, Serialize, Serializer};
use time::format_description::well_known::iso8601::{
    Config, EncodedConfig, FormattedComponents, Iso8601,
};
use time::Date;

const CONFIG: EncodedConfig = Config::DEFAULT
    .set_formatted_components(FormattedComponents::Date)
    .encode();
const DATE_FORMATTER: Iso8601<CONFIG> = Iso8601::<CONFIG>;

struct Iso8601DateVisitor;

impl<'a> de::Visitor<'a> for Iso8601DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("as ISO 8601 formatted Date")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Date::parse(value, &DATE_FORMATTER).map_err(E::custom)
    }
}

pub fn serialize<S: Serializer>(date: &Date, serializer: S) -> Result<S::Ok, S::Error> {
    date.format(&DATE_FORMATTER)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Date, D::Error> {
    deserializer.deserialize_str(Iso8601DateVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use time::macros::date;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Foo(#[serde(with = "super")] Date);

    macro_rules! value {
        ($date:literal) => {
            concat!("\"", $date, "\"")
        };
    }

    #[test]
    fn serialize() {
        #[rustfmt::skip]
        let foo = Foo(date!(2023-07-24));

        let s = serde_json::to_string(&foo).unwrap();
        assert_eq!(s, value!("2023-07-24"));
    }

    #[test]
    fn deserialize() {
        #[rustfmt::skip]
        let expected = Foo(date!(2023-07-24));

        assert_eq!(expected, serde_json::from_str(value!("20230724")).unwrap());
        assert_eq!(
            expected,
            serde_json::from_str(value!("2023-07-24")).unwrap()
        );
        assert_eq!(expected, serde_json::from_str(value!("2023-205")).unwrap());
        assert_eq!(expected, serde_json::from_str(value!("2023205")).unwrap());
    }

    #[test]
    #[ignore = "not implemented yet"]
    fn cannot_deserialize_datetime_as_only_date() {
        assert!(serde_json::from_str::<Foo>(value!("2007-03-01T13:00:00Z")).is_err());
    }
}
