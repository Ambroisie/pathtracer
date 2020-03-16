use super::{Point, Point2D, Vector};
use bvh::aabb::Bounded;
use bvh::ray::Ray;

/// Represent an abstract shape inside the scene.
pub trait Shape: Bounded + std::fmt::Debug {
    /// Return the distance at which the object intersects with the ray, or None if it does not.
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    /// Return the unit vector corresponding to the normal at this point of the shape.
    fn normal(&self, point: &Point) -> Vector;
    /// Project the point from the shape's surface to its texel coordinates.
    fn project_texel(&self, point: &Point) -> Point2D;
}

pub mod sphere;
pub use sphere::*;

pub mod triangle;
pub use triangle::*;
