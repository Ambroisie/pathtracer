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
}

/// Represent an abstract shape inside the scene.
#[enum_dispatch::enum_dispatch(ShapeEnum)]
pub trait Shape: std::fmt::Debug {
    /// Return the distance at which the object intersects with the ray, or None if it does not.
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    /// Return the unit vector corresponding to the normal at this point of the shape.
    fn normal(&self, point: &Point) -> Unit<Vector>;
    /// Project the point from the shape's surface to its texel coordinates.
    fn project_texel(&self, point: &Point) -> Point2D;
    /// Enclose the `Shape` in an axi-aligned bounding-box.
    fn aabb(&self) -> AABB;
    /// Return the centroid of the shape.
    fn centroid(&self) -> Point;
}

impl Bounded for dyn Shape {
    fn aabb(&self) -> AABB {
        self.aabb()
    }

    fn centroid(&self) -> Point {
        self.centroid()
    }
}

impl Intersected for dyn Shape {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        self.intersect(ray)
    }
}

mod sphere;
pub use sphere::*;

mod triangle;
pub use triangle::*;
