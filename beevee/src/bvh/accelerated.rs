use super::Intersected;
use crate::aabb::Bounded;
use crate::ray::Ray;

/// The trait for any mesh-like object to be used in the [`BVH`]. If your object is not an
/// aggregate, you should instead implement [`Intersected`] which derives this trait automatically.
///
/// This trait is there to accomodate for aggregate objects inside the [`BVH`]: you can implement a
/// faster look-up of information using a [`BVH`] in a mesh for example, returning directly the
/// reference to a hit triangle. This enables us to return this triangle instead of returning a
/// reference to the whole mesh.
///
/// [`BVH`]: struct.BVH.html
/// [`Intersected`]: struct.Intersected.html
pub trait Accelerated: Bounded {
    /// The type contained in your [`Accelerated`] structure
    ///
    /// [`Accelerated`]: struct.Accelerated.html
    type Output;

    /// Return None if no intersection happens with the ray, or a tuple of distance along the ray
    /// and a reference to the object that was hit.
    fn intersect(&self, ray: &Ray) -> Option<(f32, &Self::Output)>;
}

/// The automatic implementation for any [`Intersected`] object to be used in the [`BVH`].
///
/// [`BVH`]: struct.BVH.html
impl<T> Accelerated for T
where
    T: Intersected,
{
    type Output = Self;

    /// Return a reference to `self` when a distance was found.
    fn intersect(&self, ray: &Ray) -> Option<(f32, &Self::Output)> {
        self.intersect(ray).map(|t| (t, self))
    }
}
