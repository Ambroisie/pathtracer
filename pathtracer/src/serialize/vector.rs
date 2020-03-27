//! Helper functions to deserialize `Vector` values.

use crate::Vector;
use nalgebra::Unit;
use serde::de::{Deserialize, Deserializer};

/// Deserialize a vector.
///
/// Needs a custom implementation to make sur the vector is normalized when deserialized.
pub fn vector_normalizer<'de, D>(deserializer: D) -> Result<Unit<Vector>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vector = Deserialize::deserialize(deserializer)?;
    Ok(Unit::new_normalize(v))
}
