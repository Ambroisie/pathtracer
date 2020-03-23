use super::Object;

/// Represent a mesh of objects.
pub struct Mesh {
    /// The shapes composing the mesh
    #[allow(unused)] // FIXME: remove when used
    shapes: Vec<Object>,
}
