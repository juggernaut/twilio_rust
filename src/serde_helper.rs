use serde::{self, Deserialize, Deserializer, Serializer};
use serde_json::Value;
use serde::de::Error;
use chrono::prelude::*;

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.to_rfc2822());
    serializer.serialize_str(&s)
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<D>(D) -> Result<T, D::Error> where D: Deserializer
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc2822(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}

pub fn opt_deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        Value::String(s) => DateTime::parse_from_rfc2822(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
            .map(|dt| Some(dt)),
        _ => Err(Error::custom(String::from("Expected string or null"))),
        }
}

pub fn deserialize_str_to_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where D: Deserializer<'de> {
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        Value::String(s) => s.parse::<u32>()
            .map(|x| Some(x))
            .map_err(serde::de::Error::custom),
        _ => Err(Error::custom(String::from("Expected string or null"))),
    }
}
