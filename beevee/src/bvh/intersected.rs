use crate::aabb::Bounded;
use crate::ray::Ray;

/// The trait for any object to be used in the [`BVH`]. Its derivation for [`Accelerated`] is
/// automatically derived to return a reference to itself. If this not the intended semantics, see
/// [`Accelerated`].
///
/// [`Accelerated`]: struct.Accelerated.html
/// [`BVH`]: struct.BVH.html
pub trait Intersected: Bounded {
    /// Return None if there is no intersection, or the distance along the ray to the closest
    /// intersection
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}
