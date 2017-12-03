use serde::{self, Deserialize, Deserializer};
use serde_json::Value;
use serde::de::Error;
use chrono::prelude::*;

pub fn deserialize_rfc2822<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
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
