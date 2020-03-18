use crate::Vector;
use serde::de::{Deserialize, Deserializer};

pub fn vector_normalizer<'de, D>(deserializer: D) -> Result<Vector, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vector = Deserialize::deserialize(deserializer)?;
    Ok(v.normalize())
}
