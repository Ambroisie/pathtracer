#![warn(missing_docs)]

//! A pathtracing crate

/// 3D points and vectors
pub use beevee::{Point, Vector};

/// A 2D point coordinate
pub type Point2D = nalgebra::Point2<f32>;

pub mod core;
pub mod light;
pub mod material;
pub mod render;
pub mod scene;
pub mod serialize;
pub mod shape;
pub mod texture;
