//! `Option<Hash>` serialization with validation

use serde::{Deserialize, Deserializer, Serializer};

use super::hash;
use crate::hash::{Algorithm, Hash};
use crate::serializers::cow_str::CowStr;

/// Deserialize hexstring into `Option<Hash>`
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Hash>, D::Error>
where
    D: Deserializer<'de>,
{
    let hexstring = Option::<CowStr>::deserialize(deserializer)?.unwrap_or_default();
    Hash::from_hex_upper(Algorithm::Sha256, &hexstring).map_err(serde::de::Error::custom)
}

/// Serialize from `Option<Hash>` into hexstring
pub fn serialize<S>(value: &Option<Hash>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.is_none() {
        serializer.serialize_none()
    } else {
        hash::serialize(&value.unwrap(), serializer)
    }
}
