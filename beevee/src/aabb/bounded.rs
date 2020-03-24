use super::AABB;
use crate::Point;

/// A trait for objects that can be bounded using an [`AABB`]
///
/// [`AABB`]: struct.AABB.html
pub trait Bounded {
    /// Return the [`AABB`] surrounding self.
    ///
    /// [`AABB`]: struct.AABB.html
    fn aabb(&self) -> AABB;
    /// Return the centroid of self.
    fn centroid(&self) -> Point;
}

/// Implementation of [`Bounded`] for [`AABB`]
///
/// [`Bounded`]: trait.Bounded.html
/// [`AABB`]: struct.Point.html
///
/// # Examples
/// ```
/// use beevee::Point;
/// use beevee::aabb::{AABB, Bounded};
///
/// let low = Point::new(0., 0., 0.);
/// let high = Point::new(1., 2., 3.);
/// let aabb = AABB::with_bounds(low, high);
/// assert_eq!(aabb, aabb.aabb());
/// ```
impl Bounded for AABB {
    fn aabb(&self) -> AABB {
        *self
    }

    fn centroid(&self) -> Point {
        self.centroid()
    }
}

/// Implementation of [`Bounded`] for [`Point`]
///
/// [`Bounded`]: trait.Bounded.html
/// [`Point`]: ../type.Point.html
///
/// # Examples
/// ```
/// use beevee::Point;
/// use beevee::aabb::Bounded;
///
/// let point = Point::new(1., 2., 3.);
/// let aabb = point.aabb();
/// assert!(aabb.contains(&point));
/// ```
impl Bounded for Point {
    fn aabb(&self) -> AABB {
        AABB::with_bounds(*self, *self)
    }

    fn centroid(&self) -> Point {
        *self
    }
}
