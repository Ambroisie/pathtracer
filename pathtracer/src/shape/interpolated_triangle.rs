use super::triangle::Triangle;
use super::Shape;
use crate::{Point, Point2D, Vector};
use beevee::aabb::{Bounded, AABB};
use beevee::bvh::Intersected;
use beevee::ray::Ray;
use nalgebra::Unit;
use serde::Deserialize;

/// Represent a triangle with interpolated normals inside the scene.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct InterpolatedTriangle {
    #[serde(flatten)]
    tri: Triangle,
    // FIXME: serialize with unit
    normals: [Unit<Vector>; 3],
}

impl InterpolatedTriangle {
    /// Creates a new `InterpolatedTriangle` from 3 [`Point`]s and 3 [`Vector`]s.
    ///
    /// [`Point`]: ../../type.Point.html
    /// [`Point`]: ../../type.Vector.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::shape::InterpolatedTriangle;
    /// # use pathtracer::{Point, Vector};
    /// #
    /// let t = InterpolatedTriangle::new(
    ///     Point::new(1.0, 0.0, 0.0),
    ///     Point::new(0.0, 1.0, 0.0),
    ///     Point::new(0.0, 0.0, 1.0),
    ///     Vector::x_axis(),
    ///     Vector::y_axis(),
    ///     Vector::z_axis(),
    /// );
    /// ```
    pub fn new(
        c0: Point,
        c1: Point,
        c2: Point,
        n0: Unit<Vector>,
        n1: Unit<Vector>,
        n2: Unit<Vector>,
    ) -> Self {
        InterpolatedTriangle {
            tri: Triangle::new(c0, c1, c2),
            normals: [n0, n1, n2],
        }
    }
}

impl Shape for InterpolatedTriangle {
    fn normal(&self, point: &Point) -> Unit<Vector> {
        let (u, v) = {
            let c = self.tri.barycentric(point);
            (c.x, c.y)
        };
        let interpol = self.normals[0].as_ref() * (1. - u - v)
            + self.normals[1].as_ref() * u
            + self.normals[2].as_ref() * v;
        Unit::new_normalize(interpol)
    }

    fn project_texel(&self, point: &Point) -> Point2D {
        self.tri.project_texel(point)
    }
}

impl Bounded for InterpolatedTriangle {
    fn aabb(&self) -> AABB {
        self.tri.aabb()
    }

    fn centroid(&self) -> Point {
        self.tri.centroid()
    }
}

impl Intersected for InterpolatedTriangle {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        self.tri.intersect(ray)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn simple_triangle() -> InterpolatedTriangle {
        InterpolatedTriangle::new(
            Point::origin(),
            Point::new(0., 1., 1.),
            Point::new(0., 1., 0.),
            Vector::x_axis(),
            Vector::y_axis(),
            Vector::z_axis(),
        )
    }

    #[test]
    fn normal_interpolation_at_c0_works() {
        let triangle = simple_triangle();
        let normal = triangle.normal(&Point::origin());
        assert_eq!(normal, Vector::x_axis());
    }

    #[test]
    fn normal_interpolation_at_c1_works() {
        let triangle = simple_triangle();
        let normal = triangle.normal(&Point::new(0., 1., 1.));
        assert_eq!(normal, Vector::y_axis());
    }

    #[test]
    fn normal_interpolation_at_c2_works() {
        let triangle = simple_triangle();
        let normal = triangle.normal(&Point::new(0., 1., 0.));
        assert_eq!(normal, Vector::z_axis());
    }

    #[test]
    fn normal_interpolation_at_center_works() {
        let triangle = simple_triangle();
        let center = Point::new(0., 2. / 3., 1. / 3.);
        let normal = triangle.normal(&center);
        let expected = Unit::new_normalize(Vector::new(1., 1., 1.));
        assert!((normal.as_ref() - expected.as_ref()).magnitude() < 1e-5)
    }

    #[test]
    fn deserialization_works() {
        let yaml = r#"
            corners:
              - [0.0, 0.0, 0.0]
              - [0.0, 1.0, 1.0]
              - [0.0, 1.0, 0.0]
            normals:
              - [1.0, 0.0, 0.0]
              - [0.0, 1.0, 0.0]
              - [0.0, 0.0, 1.0]
        "#;
        let triangle: InterpolatedTriangle = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            triangle,
            InterpolatedTriangle::new(
                Point::origin(),
                Point::new(0., 1., 1.),
                Point::new(0., 1., 0.),
                Vector::x_axis(),
                Vector::y_axis(),
                Vector::z_axis(),
            )
        )
    }
}
