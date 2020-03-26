use super::Object;
use serde::Deserialize;

/// Represent a mesh of objects.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Mesh {
    /// The shapes composing the mesh
    pub(crate) shapes: Vec<Object>,
}

// FIXME: wavefront mesh deserialized in mesh
