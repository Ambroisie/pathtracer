#![warn(missing_docs)]

//! A Bounding Volume Hierarchy crate for use with ray-tracing.

/// The point to describe the [`AABB`]'s corners.
///
/// [`AABB`]: aabb/struct.AABB.html
pub type Point = nalgebra::Point3<f32>;

/// The Vector to describe the [`Ray`]'s direction.
///
/// [`Ray`]: ray/struct.Ray.html
pub type Vector = nalgebra::Vector3<f32>;

/// The module relating to Axis-Aligned Bouding Boxes.
pub mod aabb;

/// The module relating to Bouding Volume Hiearchy
pub mod bvh;

/// Module defining a [`Ray`] structure to intersect with the [`BVH`]
///
/// [`BVH`]: ../bvh/struct.BVH.html
/// [`Ray`]: struct.Ray.html
pub mod ray;
