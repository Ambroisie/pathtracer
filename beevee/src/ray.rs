use crate::aabb::AABB;
use crate::{Point, Vector};
use nalgebra::Unit;
use std::fmt::{Display, Formatter, Result};

/// The [`Ray`] to intersect with the [`BVH`].
///
/// [`BVH`]: ../bvh/struct.BVH.html
/// [`Ray`]: struct.Ray.html
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    /// The point of origin of the ray.
    origin: Point,
    /// A unit vector representing the direction of the ray.
    direction: Unit<Vector>,
    /// The inverse of each coefficient of the ray's direction.
    inv_direction: Vector,
}

impl Ray {
    /// Create a new [`Ray`] with the given origin and direction
    ///
    /// [`Ray`]: struct.Ray.html
    ///
    /// # Examples
    /// ```
    /// use beevee::{Point, Vector};
    /// use beevee::ray::Ray;
    ///
    /// let ray = Ray::new(Point::origin(), Vector::x_axis());
    /// ```
    #[must_use]
    pub fn new(origin: Point, direction: Unit<Vector>) -> Self {
        let inv_direction = Vector::new(1. / direction.x, 1. / direction.y, 1. / direction.z);
        Ray {
            origin,
            direction,
            inv_direction,
        }
    }

    /// Change the [`Point`] of origin a the [`Ray`] and return the new value.
    ///
    /// [`Point`]: ../type.Point.html
    /// [`Ray`]: struct.Ray.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::{Point, Vector};
    /// # use beevee::ray::Ray;
    ///
    /// let ray = Ray::new(Point::origin(), Vector::x_axis());
    /// let new_origin = Point::new(0., 1., 2.);
    /// let new_ray = ray.with_origin(new_origin);
    ///
    /// assert_eq!(new_ray, Ray::new(new_origin, Vector::x_axis()));
    /// ```
    #[must_use]
    pub fn with_origin(&self, origin: Point) -> Self {
        let mut ans = *self;
        ans.with_origin_mut(origin);
        ans
    }

    /// Mutably change the [`Point`] of origin a the [`Ray`].
    ///
    /// [`Point`]: ../type.Point.html
    /// [`Ray`]: struct.Ray.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::{Point, Vector};
    /// # use beevee::ray::Ray;
    ///
    /// let mut ray = Ray::new(Point::origin(), Vector::x_axis());
    /// let new_origin = Point::new(0., 1., 2.);
    ///
    /// ray.with_origin_mut(new_origin);
    ///
    /// assert_eq!(ray, Ray::new(new_origin, Vector::x_axis()));
    /// ```
    pub fn with_origin_mut(&mut self, origin: Point) -> &mut Self {
        self.origin = origin;
        self
    }

    /// Change the [`Vector`] of direction a the [`Ray`] and return the new value.
    ///
    /// [`Vector`]: ../type.Vector.html
    /// [`Ray`]: struct.Ray.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::{Point, Vector};
    /// # use beevee::ray::Ray;
    ///
    /// let ray = Ray::new(Point::origin(), Vector::x_axis());
    /// let new_direction = Vector::y_axis();
    /// let new_ray = ray.with_direction(new_direction);
    ///
    /// assert_eq!(new_ray, Ray::new(Point::origin(), new_direction));
    /// ```
    #[must_use]
    pub fn with_direction(&self, direction: Unit<Vector>) -> Self {
        let mut ans = *self;
        ans.with_direction_mut(direction);
        ans
    }

    /// Mutable change the [`Vector`] of direction a the [`Ray`].
    ///
    /// [`Vector`]: ../type.Vector.html
    /// [`Ray`]: struct.Ray.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::{Point, Vector};
    /// # use beevee::ray::Ray;
    ///
    /// let mut ray = Ray::new(Point::origin(), Vector::x_axis());
    /// let new_direction = Vector::y_axis();
    /// ray.with_direction_mut(new_direction);
    ///
    /// assert_eq!(ray, Ray::new(Point::origin(), new_direction));
    /// ```
    pub fn with_direction_mut(&mut self, direction: Unit<Vector>) -> &mut Self {
        self.direction = direction;
        self.inv_direction = Vector::new(1. / direction.x, 1. / direction.y, 1. / direction.z);
        self
    }

    /// Return the distance to intersect with an [`AABB`], or [`None`] if there's no intersection.
    ///
    /// [`AABB`]: ../aabb/struct.AABB.html
    /// [`Ray`]: struct.Ray.html
    ///
    /// # Examples
    /// ```
    /// use beevee::{Point, Vector};
    /// use beevee::aabb::AABB;
    /// use beevee::ray::Ray;
    ///
    /// let aabb = AABB::with_bounds(Point::new(1., -1., -1.), Point::new(3., 1., 1.));
    /// let ray = Ray::new(Point::origin(), Vector::x_axis());
    ///
    /// assert_eq!(ray.aabb_intersection(&aabb), Some(1.));
    /// ```
    ///
    /// ```
    /// use beevee::{Point, Vector};
    /// use beevee::aabb::AABB;
    /// use beevee::ray::Ray;
    ///
    /// let aabb = AABB::with_bounds(Point::new(-1., -1., -1.), Point::new(1., 1., 1.));
    /// let ray = Ray::new(Point::origin(), Vector::x_axis());
    ///
    /// // Also works from inside the AABB.
    /// assert_eq!(ray.aabb_intersection(&aabb), Some(1.));
    /// ```
    ///
    /// ```
    /// use beevee::{Point, Vector};
    /// use beevee::aabb::AABB;
    /// use beevee::ray::Ray;
    ///
    /// let aabb = AABB::with_bounds(Point::new(1., -1., -1.), Point::new(3., 1., 1.));
    /// let ray = Ray::new(Point::origin(), Vector::y_axis());
    ///
    /// assert_eq!(ray.aabb_intersection(&aabb), None);
    /// ```
    pub fn aabb_intersection(&self, aabb: &AABB) -> Option<f32> {
        use crate::Axis;
        let min_max = |axis: Axis| {
            let a = (aabb.high[axis] - self.origin[axis]) * self.inv_direction[axis];
            let b = (aabb.low[axis] - self.origin[axis]) * self.inv_direction[axis];
            if self.direction[axis] < 0. {
                (a, b)
            } else {
                (b, a)
            }
        };
        let (mut t_min, mut t_max) = min_max(Axis::X);

        let (y_min, y_max) = min_max(Axis::Y);

        if y_min > t_max || y_max < t_min {
            return None;
        }

        if y_min > t_min {
            t_min = y_min;
        }
        if y_max < t_max {
            t_max = y_max;
        }

        let (z_min, z_max) = min_max(Axis::Z);

        if z_min > t_max || z_max < t_min {
            return None;
        }

        if z_min > t_min {
            t_min = z_min;
        }
        if z_max < t_max {
            t_max = z_max;
        }

        if t_max < 0. {
            return None;
        }

        if t_min < 0. {
            Some(t_max)
        } else {
            Some(t_min)
        }
    }
}

/// Display implementation for [`Ray`].
///
/// [`Ray`]: struct.Ray.html
///
/// # Examples
/// ```
/// # use beevee::{Point, Vector};
/// # use beevee::ray::Ray;
/// let ray = Ray::new(Point::origin(), Vector::x_axis());
///
/// assert_eq!(format!("{}", ray), "origin: {0, 0, 0}, direction: {1, 0, 0}")
/// ```
impl Display for Ray {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "origin: {}, direction: {{{}, {}, {}}}",
            self.origin, self.direction.x, self.direction.y, self.direction.z,
        )
    }
}
