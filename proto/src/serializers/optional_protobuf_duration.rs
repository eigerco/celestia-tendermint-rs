//! [`serde`] serializer for optional [`Duration`].

use crate::google::protobuf::Duration;
use crate::serializers::cow_str::CowStr;
use alloc::string::ToString;
use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Deserialize [`Option<Duration>`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let Some(s) = Option::<CowStr>::deserialize(deserializer)? else {
        return Ok(None);
    };

    let ns = s
        .parse::<u64>()
        .map_err(|e| D::Error::custom(e.to_string()))?;

    let dur = core::time::Duration::from_nanos(ns);

    let seconds: i64 = dur
        .as_secs()
        .try_into()
        .map_err(|_| D::Error::custom("Duration.seconds overflow".to_string()))?;
    let nanos: i32 = dur
        .subsec_nanos()
        .try_into()
        .map_err(|_| D::Error::custom("Duration.nanos overflow".to_string()))?;

    Ok(Some(Duration { seconds, nanos }))
}

/// Serialize [`Option<Duration>`].
pub fn serialize<S>(value: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(value) => {
            let seconds: u64 = value
                .seconds
                .try_into()
                .map_err(|_| S::Error::custom("negative Duration.seconds".to_string()))?;
            let nanos: u32 = value
                .nanos
                .try_into()
                .map_err(|_| S::Error::custom("negative Duration.nanos".to_string()))?;

            let dur = core::time::Duration::new(seconds, nanos);

            dur.as_nanos().to_string().serialize(serializer)
        },
        None => serializer.serialize_none(),
    }
}
