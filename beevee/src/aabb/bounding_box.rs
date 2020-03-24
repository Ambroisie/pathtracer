//! An Axis-Alighned Bounding Box.

use crate::{Axis, Point, Vector};
use std::fmt::{Display, Formatter, Result};

/// An Axis-Aligned Bounding Box.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AABB {
    /// The corner with the lowest (x, y, z) coordinates.
    pub low: Point,
    /// The corner with the highest (x, y, z) coordinates.
    pub high: Point,
}

impl AABB {
    /// Create a new empty [`AABB`]
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    ///
    /// ```
    /// use beevee::Point;
    /// use beevee::aabb::AABB;
    ///
    /// let aabb = AABB::empty();
    ///
    /// // Here for teh origin, but also true for any other point
    /// let point = Point::origin();
    ///
    /// assert!(!aabb.contains(&point));
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        let lowest = std::f32::NEG_INFINITY;
        let highest = std::f32::INFINITY;
        AABB {
            low: Point::new(highest, highest, highest),
            high: Point::new(lowest, lowest, lowest),
        }
    }

    /// Create a new empty [`AABB`]
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// assert_eq!(aabb, AABB::with_bounds(low, high));
    /// ```
    #[must_use]
    pub fn with_bounds(low: Point, high: Point) -> Self {
        debug_assert!(low.x <= high.x);
        debug_assert!(low.y <= high.y);
        debug_assert!(low.z <= high.z);
        AABB { low, high }
    }

    /// Return a new bounding box containing both `self` and the new [`Point`]
    ///
    /// [`Point`]: ../type.Point.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let aabb = AABB::empty();
    /// let new_aabb = aabb.grow(&Point::origin());
    ///
    /// assert_eq!(new_aabb, AABB::with_bounds(Point::origin(), Point::origin()));
    /// ```
    #[must_use]
    pub fn grow(&self, point: &Point) -> Self {
        let mut ans = *self;
        ans.grow_mut(point);
        ans
    }

    /// Grow the bounding box to accomodate a new  [`Point`].
    ///
    /// [`Point`]: ../type.Point.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let mut aabb = AABB::empty();
    /// aabb.grow_mut(&Point::origin());
    ///
    /// assert_eq!(aabb, AABB::with_bounds(Point::origin(), Point::origin()));
    /// ```
    pub fn grow_mut(&mut self, point: &Point) -> &mut Self {
        // Update lowest bound
        self.low.x = self.low.x.min(point.x);
        self.low.y = self.low.y.min(point.y);
        self.low.z = self.low.z.min(point.z);
        // Update higher bound
        self.high.x = self.high.x.max(point.x);
        self.high.y = self.high.y.max(point.y);
        self.high.z = self.high.z.max(point.z);
        // Return self for method chaining
        self
    }

    /// Return true if the bounding box contains the [`Point`], false otherwise
    ///
    /// [`Point`]: ../type.Point.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// // It contains the whole box from low to high
    /// assert!(aabb.contains(&low));
    /// assert!(aabb.contains(&high));
    /// assert!(aabb.contains(&Point::new(0.5, 0.5, 0.5)));
    ///
    /// // And doesn't contain anything else
    /// assert!(!aabb.contains(&Point::new(-1., -1., -1.)));
    /// assert!(!aabb.contains(&Point::new(1.1, 0., 0.)));
    /// assert!(!aabb.contains(&Point::new(2., -2., 0.)));
    /// ```
    pub fn contains(&self, point: &Point) -> bool {
        (self.low.x..=self.high.x).contains(&point.x)
            && (self.low.y..=self.high.y).contains(&point.y)
            && (self.low.z..=self.high.z).contains(&point.z)
    }

    /// Return a new `AABB` which encloses `self` and the other [`AABB`].
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::empty();
    /// let other = AABB::with_bounds(low, high);
    ///
    ///  // Grow the AABB to enclose the other one.
    ///  let union = aabb.union(&other);
    ///
    /// // The result is now the union of an empty bounding box and the other one.
    ///  assert_eq!(union, other);
    /// ```
    pub fn union(&self, other: &Self) -> Self {
        // Clone the first bounding box.
        let mut ans = *self;
        // Update the new bounding box.
        ans.union_mut(other);
        // Return the new bounding box.
        ans
    }

    /// Grow `self` to enclose the other [`AABB`].
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let mut aabb = AABB::empty();
    /// let other = AABB::with_bounds(low, high);
    ///
    ///  // Grow the AABB to enclose the other one.
    ///  aabb.union_mut(&other);
    ///
    /// // The bounding box has grown to become equal to other one.
    ///  assert_eq!(aabb, other);
    /// ```
    pub fn union_mut(&mut self, other: &Self) -> &mut Self {
        self.grow_mut(&other.low);
        self.grow_mut(&other.high);
        self
    }

    /// Return a vector correspondin to the diagonal from `low` to `high` for the [`AABB`].
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// assert_eq!(aabb.diagonal(), high - low);
    /// ```
    pub fn diagonal(&self) -> Vector {
        self.high - self.low
    }

    /// Return the center of the [`AABB`].
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// assert_eq!(aabb.centroid(), low + (high - low) / 2.);
    /// ```
    pub fn centroid(&self) -> Point {
        self.low + self.diagonal() / 2.
    }

    /// Return true if the [`AABB`] is empty, false otherwise.
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let not_empty = AABB::with_bounds(low, high);
    /// let empty = AABB::empty();
    ///
    /// assert!(!not_empty.is_empty());
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.low.x > self.high.x || self.low.y > self.high.y || self.low.z > self.high.z
    }

    /// Return the total surface area of the [`AABB`].
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// assert!((aabb.surface() - 6.).abs() < std::f32::EPSILON);
    /// ```
    pub fn surface(&self) -> f32 {
        let diagonal = self.diagonal();
        2. * (diagonal.x * diagonal.y + diagonal.x * diagonal.z + diagonal.y * diagonal.z)
    }

    /// Return the total volume of the [`AABB`].
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// assert!((aabb.volume() - 1.).abs() < std::f32::EPSILON);
    /// ```
    pub fn volume(&self) -> f32 {
        let diagonal = self.diagonal();
        diagonal.x * diagonal.y * diagonal.z
    }

    /// Return the axis along which the [`AABB`] is the largest.
    ///
    /// [`AABB`]: struct.AABB.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// use beevee::Axis;
    ///
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(3., 2., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// assert_eq!(aabb.largest_axis(), Axis::X);
    /// ```
    ///
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// use beevee::Axis;
    ///
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// // Prefers the X axis in case of a three-way tie, then the Y axis in a tie with Z
    /// assert_eq!(aabb.largest_axis(), Axis::X);
    /// ```
    pub fn largest_axis(&self) -> Axis {
        let diagonal = self.diagonal();
        if diagonal.x >= diagonal.y && diagonal.x >= diagonal.z {
            Axis::X
        } else if diagonal.y >= diagonal.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    /// Return the shortest distance from an [`AABB`] to a [`Point`].
    ///
    /// [`AABB`]: struct.AABB.html
    /// [`AABB`]: ../type.Point.html
    ///
    /// # Examples
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// assert!((aabb.distance_to_point(Point::new(-1., 0., 0.)) - 1.).abs() < std::f32::EPSILON);
    /// ```
    ///
    /// ```
    /// # use beevee::Point;
    /// # use beevee::aabb::AABB;
    /// #
    /// let low = Point::new(0., 0., 0.);
    /// let high = Point::new(1., 1., 1.);
    /// let aabb = AABB::with_bounds(low, high);
    ///
    /// // Returns 0. when the point is contained by the AABB
    /// assert!(aabb.distance_to_point(Point::new(0.5, 0.5, 0.5)).abs() < std::f32::EPSILON);
    /// ```
    pub fn distance_to_point(&self, point: Point) -> f32 {
        let dx = (self.low.x - point.x).max(0.).max(point.x - self.high.x);
        let dy = (self.low.y - point.y).max(0.).max(point.y - self.high.y);
        let dz = (self.low.z - point.z).max(0.).max(point.z - self.high.z);
        f32::sqrt(dx * dx + dy * dy + dz * dz)
    }
}

/// Display implementation for [`AABB`].
///
/// [`AABB`]: struct.AABB.html
///
/// # Examples
/// ```
/// # use beevee::Point;
/// # use beevee::aabb::AABB;
/// let low = Point::new(0., 0., 0.);
/// let high = Point::new(1., 1., 1.);
/// let aabb = AABB::with_bounds(low, high);
///
/// assert_eq!(format!("{}", aabb), "low: {0, 0, 0}, high: {1, 1, 1}");
/// ```
impl Display for AABB {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "low: {}, high: {}", self.low, self.high)
    }
}

/// Return an empty [`AABB`].
///
/// [`AABB`]: struct.AABB.html
///
/// # Examples
///
/// ```
/// # use beevee::aabb::AABB;
/// let default = <AABB as Default>::default();
/// let empty = AABB::empty();
///
/// assert_eq!(default, empty);
/// ```
impl Default for AABB {
    fn default() -> Self {
        AABB::empty()
    }
}
