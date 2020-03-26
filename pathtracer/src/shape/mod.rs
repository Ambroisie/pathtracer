//! Various shape implementations

use super::{Point, Point2D, Vector};
use beevee::{
    aabb::{Bounded, AABB},
    bvh::Intersected,
    ray::Ray,
};
use nalgebra::Unit;
use serde::Deserialize;

/// All the existing `Shape` implementation.
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
#[enum_dispatch::enum_dispatch]
#[derive(Debug, PartialEq, Deserialize)]
pub enum ShapeEnum {
    Sphere,
    Triangle,
    InterpolatedTriangle,
}

// FIXME: this has to be written by hand due to a limitation of `enum_dispatch` on super traits
impl Bounded for ShapeEnum {
    fn aabb(&self) -> AABB {
        match self {
            ShapeEnum::Sphere(s) => s.aabb(),
            ShapeEnum::Triangle(s) => s.aabb(),
            ShapeEnum::InterpolatedTriangle(s) => s.aabb(),
        }
    }

    fn centroid(&self) -> Point {
        match self {
            ShapeEnum::Sphere(s) => s.centroid(),
            ShapeEnum::Triangle(s) => s.centroid(),
            ShapeEnum::InterpolatedTriangle(s) => s.centroid(),
        }
    }
}

impl Intersected for ShapeEnum {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        match self {
            ShapeEnum::Sphere(s) => s.intersect(ray),
            ShapeEnum::Triangle(s) => s.intersect(ray),
            ShapeEnum::InterpolatedTriangle(s) => s.intersect(ray),
        }
    }
}

/// Represent an abstract shape inside the scene.
#[enum_dispatch::enum_dispatch(ShapeEnum)]
pub trait Shape: std::fmt::Debug + Intersected {
    /// Return the unit vector corresponding to the normal at this point of the shape.
    fn normal(&self, point: &Point) -> Unit<Vector>;
    /// Project the point from the shape's surface to its texel coordinates.
    fn project_texel(&self, point: &Point) -> Point2D;
}

mod interpolated_triangle;
pub use interpolated_triangle::*;

mod sphere;
pub use sphere::*;

mod triangle;
pub use triangle::*;
