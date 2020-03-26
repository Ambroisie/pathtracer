use super::Shape;
use crate::{Point, Point2D, Vector};
use beevee::aabb::{Bounded, AABB};
use beevee::bvh::Intersected;
use beevee::ray::Ray;
use nalgebra::Unit;
use serde::Deserialize;

/// Represent a sphere shape inside the scene.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Sphere {
    /// The sphere is inverted if it is expected to be seen from the inside.
    #[serde(default)]
    inverted: bool,
    /// The center of the sphere in space.
    center: Point,
    /// The radius of the sphere being rendered.
    radius: f32,
}

impl Sphere {
    /// Return a sphere which should be rendered as seen from the outside.
    pub fn new(center: Point, radius: f32) -> Self {
        Sphere {
            center,
            radius,
            inverted: false,
        }
    }

    /// Return a sphere which should be rendered as seen from the inside.
    pub fn inverted_new(center: Point, radius: f32) -> Self {
        Sphere {
            center,
            radius,
            inverted: true,
        }
    }
}

impl Shape for Sphere {
    fn normal(&self, point: &Point) -> Unit<Vector> {
        let delt = if self.inverted {
            self.center - point
        } else {
            point - self.center
        };
        Unit::new_normalize(delt)
    }

    fn project_texel(&self, point: &Point) -> Point2D {
        // Project the sphere on the XY-plane
        Point2D::new(
            0.5 + (point.x - self.center.x) / (2. * self.radius),
            0.5 + (point.y - self.center.y) / (2. * self.radius),
        )
    }
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let delt = Vector::new(self.radius, self.radius, self.radius);
        let min = self.center - delt;
        let max = self.center + delt;
        AABB::with_bounds(min, max)
    }

    fn centroid(&self) -> Point {
        self.center
    }
}

impl Intersected for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        use std::mem;

        let delt = self.center - ray.origin;
        let tca = ray.direction.dot(&delt);
        let d2 = delt.norm_squared() - tca * tca;
        let r_2 = self.radius * self.radius;

        if d2 > r_2 {
            return None;
        }

        let thc = (r_2 - d2).sqrt();
        let mut t_0 = tca - thc;
        let mut t_1 = tca + thc;

        if t_0 > t_1 {
            mem::swap(&mut t_0, &mut t_1)
        }
        if t_0 < 0. {
            t_0 = t_1
        }

        if t_0 < 0. {
            None
        } else {
            Some(t_0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn simple_sphere() -> Sphere {
        Sphere::new(Point::origin(), 1.)
    }

    #[test]
    fn intersect_along_axis_works() {
        let sphere = simple_sphere();
        let ray = Ray::new(
            Point::new(-2., 0., 0.),
            Unit::new_normalize(Vector::new(1., 0., 0.)),
        );
        assert_eq!(sphere.intersect(&ray), Some(1.))
    }

    #[test]
    fn non_intersect_along_axis_works() {
        let sphere = simple_sphere();
        let ray = Ray::new(
            Point::new(-2., 0., 0.),
            Unit::new_normalize(Vector::new(-1., 0., 0.)),
        );
        assert_eq!(sphere.intersect(&ray), None)
    }

    #[test]
    fn intersect_not_on_axis() {
        let sphere = simple_sphere();
        let ray = Ray::new(
            Point::new(1., 1., 1.),
            Unit::new_normalize(Vector::new(-1., -1., -1.)),
        );
        assert_eq!(sphere.intersect(&ray), Some(f32::sqrt(3.) - 1.))
    }

    #[test]
    fn normal_works() {
        let sphere = simple_sphere();
        assert_eq!(
            sphere.normal(&Point::new(-1., 0., 0.)),
            Unit::new_normalize(Vector::new(-1., 0., 0.))
        )
    }

    #[test]
    fn inverted_normal_works() {
        let sphere = Sphere::inverted_new(Point::origin(), 1.);
        assert_eq!(
            sphere.normal(&Point::new(-1., 0., 0.)),
            Unit::new_normalize(Vector::new(1., 0., 0.))
        )
    }

    #[test]
    fn projection_works_1() {
        let sphere = simple_sphere();
        let projection = sphere.project_texel(&Point::new(-1., -1., 1.));
        assert!(projection.x.abs() < 1e-5);
        assert!(projection.y.abs() < 1e-5)
    }

    #[test]
    fn projection_works_2() {
        let sphere = simple_sphere();
        let projection = sphere.project_texel(&Point::new(1., -1., 1.));
        assert!((projection.x - 1.).abs() < 1e-5);
        assert!(projection.y.abs() < 1e-5)
    }

    #[test]
    fn projection_works_3() {
        let sphere = simple_sphere();
        let projection = sphere.project_texel(&Point::new(1., 0., 1.));
        assert!((projection.x - 1.).abs() < 1e-5);
        assert!((projection.y - 0.5).abs() < 1e-5)
    }

    #[test]
    fn deserialization_works() {
        let yaml = r#"
            inverted: false
            center: [0.5, 1.0, 2.0]
            radius: 2.5
        "#;
        let sphere: Sphere = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(sphere, Sphere::new(Point::new(0.5, 1.0, 2.0), 2.5))
    }
}
