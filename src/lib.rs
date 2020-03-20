#![warn(missing_docs)]

use bvh::nalgebra::{Point2, Point3, Vector3};

pub type Point2D = Point2<f32>;
pub type Point = Point3<f32>;
pub type Vector = Vector3<f32>;

pub mod core;
pub mod light;
pub mod material;
pub mod render;
pub mod serialize;
pub mod shape;
pub mod texture;
