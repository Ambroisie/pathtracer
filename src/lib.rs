#![warn(missing_docs)]

//! A pathtracing crate

use bvh::nalgebra::{Point2, Point3, Vector3};

/// A 2D point coordinate
pub type Point2D = Point2<f32>;
/// A 3D point coordinate
pub type Point = Point3<f32>;
/// A 3D vector
pub type Vector = Vector3<f32>;

pub mod core;
pub mod light;
pub mod material;
pub mod render;
pub mod serialize;
pub mod shape;
pub mod texture;
