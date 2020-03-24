use crate::{Point, Vector};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

/// An enum for indexing different spatial structures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Axis {
    /// The X axis.
    X = 0,
    /// The Y axis.
    Y = 1,
    /// The Z axis.
    Z = 2,
}

/// Display implementation for [`Axis`].
///
/// [`Axis`]: enum.Axis.html
///
/// # Examples
/// ```
/// # use beevee::Axis;
/// assert_eq!(format!("{}", Axis::X), "x");
/// assert_eq!(format!("{}", Axis::Y), "y");
/// assert_eq!(format!("{}", Axis::Z), "z");
/// ```
impl Display for Axis {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                Axis::X => "x",
                Axis::Y => "y",
                Axis::Z => "z",
            }
        )
    }
}

/// Slice indexing implementation by [`Axis`]
///
/// [`Axis`]: enum.Axis.html
///
/// # Examples
/// ```
/// # use beevee::Axis;
/// #
/// let slice = &[0., 1., 2.];
/// assert_eq!(slice[Axis::X], 0.);
/// ```
impl<T> Index<Axis> for [T] {
    type Output = T;

    fn index(&self, axis: Axis) -> &Self::Output {
        &self[axis as usize]
    }
}

/// Mutable slice indexing implementation by [`Axis`]
///
/// [`Axis`]: enum.Axis.html
///
/// # Examples
/// ```
/// # use beevee::Axis;
/// #
/// let slice = &mut [0., 1., 2.];
/// slice[Axis::X] = 3.;
/// assert_eq!(slice, &[3., 1., 2.]);
/// ```
impl<T> IndexMut<Axis> for [T] {
    fn index_mut(&mut self, axis: Axis) -> &mut T {
        &mut self[axis as usize]
    }
}

/// [`Point`] indexing implementation by [`Axis`]
///
/// [`Axis`]: enum.Axis.html
/// [`Point`]: type.Point.html
///
/// # Examples
/// ```
/// # use beevee::{Axis, Point};
/// #
/// let point = Point::new(0., 1., 2.);
/// assert_eq!(point[Axis::X], 0.);
/// ```
impl Index<Axis> for Point {
    type Output = f32;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

/// Mutable [`Point`] indexing implementation by [`Axis`]
///
/// [`Axis`]: enum.Axis.html
/// [`Point`]: type.Point.html
///
/// # Examples
/// ```
/// # use beevee::{Axis, Point};
/// #
/// let mut point =  Point::new(0., 1., 2.);
/// point[Axis::X] = 3.;
/// assert_eq!(point, Point::new(3., 1., 2.));
/// ```
impl IndexMut<Axis> for Point {
    fn index_mut(&mut self, axis: Axis) -> &mut f32 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

/// [`Vector`] indexing implementation by [`Axis`]
///
/// [`Axis`]: enum.Axis.html
/// [`Vector`]: type.Vector.html
///
/// # Examples
/// ```
/// # use beevee::{Axis, Vector};
/// #
/// let point = Vector::new(0., 1., 2.);
/// assert_eq!(point[Axis::X], 0.);
/// ```
impl Index<Axis> for Vector {
    type Output = f32;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

/// Mutable [`Vector`] indexing implementation by [`Axis`]
///
/// [`Axis`]: enum.Axis.html
/// [`Vector`]: type.Vector.html
///
/// # Examples
/// ```
/// # use beevee::{Axis, Vector};
/// #
/// let mut point =  Vector::new(0., 1., 2.);
/// point[Axis::X] = 3.;
/// assert_eq!(point, Vector::new(3., 1., 2.));
/// ```
impl IndexMut<Axis> for Vector {
    fn index_mut(&mut self, axis: Axis) -> &mut f32 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}
