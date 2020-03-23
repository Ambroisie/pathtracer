//! Various shape implementations

use super::{Point, Point2D, Vector};
use bvh::{
    aabb::{Bounded, AABB},
    ray::Ray,
};
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
}

/// Represent an abstract shape inside the scene.
#[enum_dispatch::enum_dispatch(ShapeEnum)]
pub trait Shape: std::fmt::Debug {
    /// Return the distance at which the object intersects with the ray, or None if it does not.
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    /// Return the unit vector corresponding to the normal at this point of the shape.
    fn normal(&self, point: &Point) -> Vector;
    /// Project the point from the shape's surface to its texel coordinates.
    fn project_texel(&self, point: &Point) -> Point2D;
    /// Enclose the `Shape` in an axi-aligned bounding-box.
    fn aabb(&self) -> AABB;
}

impl Bounded for dyn Shape {
    fn aabb(&self) -> AABB {
        self.aabb()
    }
}

mod sphere;
pub use sphere::*;

mod triangle;
pub use triangle::*;
