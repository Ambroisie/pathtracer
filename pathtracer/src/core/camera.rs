//! Camera related logic

use super::film::Film;
use crate::{Point, Vector};
use serde::Deserialize;

/// Represent an abstract camera to observe the scene.
#[serde(from = "SerializedCamera")]
#[derive(Debug, PartialEq, Deserialize)]
pub struct Camera {
    /// Where the camera is set in the scene (i.e: its focal point).
    origin: Point,
    /// The film to represent each pixel in the scene.
    film: Film,
}

impl Camera {
    /// Creates a new `Camera`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::Camera;
    /// use pathtracer::{Point, Vector};
    ///
    /// let cam = Camera::new(
    ///     Point::new(-1., 0., 0.),
    ///     Vector::new(1., 0., 0.),
    ///     Vector::new(0., 1., 0.),
    ///     2. * f32::atan(1.), /* 90° in radian */
    ///     1.,
    ///     1080,
    ///     1080,
    /// );
    /// ```
    pub fn new(
        origin: Point,
        forward: Vector,
        up: Vector,
        fov: f32,
        dist_to_image: f32,
        x: u32,
        y: u32,
    ) -> Self {
        let right = forward.cross(&up);
        let center = origin + forward.normalize() * dist_to_image;
        let screen_size = 2. * f32::tan(fov / 2.) * dist_to_image;
        let film = Film::new(x, y, screen_size, center, up, right);
        Camera { origin, film }
    }

    /// Get the `Camera`'s [`Film`].
    ///
    /// [`Film`]: ../film/struct.Film.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::{Camera, Film};
    /// #
    /// let cam = Camera::default();
    /// let film: &Film = cam.film();
    /// ```
    pub fn film(&self) -> &Film {
        &self.film
    }

    /// Get the `Camera`'s `Point` of origin.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::Camera;
    /// # use pathtracer::Point;
    /// #
    /// let cam = Camera::default();
    /// let origin: &Point = cam.origin();
    /// ```
    pub fn origin(&self) -> &Point {
        &self.origin
    }
}

impl Default for Camera {
    /// Returns a `Camera` with a 1080x1080 `Film`
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::core::Camera;
    /// use pathtracer::{Point, Vector};
    ///
    /// let default = Camera::default();
    /// let new = Camera::new(
    ///     Point::new(0., 0., 0.),
    ///     Vector::new(1., 0., 0.),
    ///     Vector::new(0., 1., 0.),
    ///     2. * f32::atan(1.), /* 90° in radian */
    ///     1.,
    ///     1080,
    ///     1080,
    /// );
    ///
    /// assert_eq!(default, new);
    /// ```
    fn default() -> Self {
        Self::new(
            Point::origin(),
            Vector::new(1., 0., 0.),
            Vector::new(0., 1., 0.),
            2. * f32::atan(1.), /* 90° in radian */
            1.,
            1080,
            1080,
        )
    }
}

#[derive(Debug, Deserialize)]
struct SerializedCamera {
    origin: Point,
    forward: Vector,
    up: Vector,
    fov: f32,
    distance_to_image: f32,
    x: u32,
    y: u32,
}

impl From<SerializedCamera> for Camera {
    fn from(cam: SerializedCamera) -> Self {
        Camera::new(
            cam.origin,
            cam.forward,
            cam.up,
            std::f32::consts::PI * cam.fov / 180.,
            cam.distance_to_image,
            cam.x,
            cam.y,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works() {
        let cam = Camera::new(
            Point::new(-1., 0., 0.),
            Vector::new(1., 0., 0.),
            Vector::new(0., 1., 0.),
            2. * f32::atan(1.), /* 90° in radian */
            1.,
            1080,
            1080,
        );
        assert_eq!(
            cam,
            Camera {
                origin: Point::new(-1., 0., 0.),
                film: Film::new(
                    1080,
                    1080,
                    2.,
                    Point::origin(),
                    Vector::new(0., 1., 0.),
                    Vector::new(0., 0., 1.),
                )
            }
        )
    }

    #[test]
    fn deserialization_works() {
        let yaml = r#"
            origin: [-1.0, 0.0, 0.0]
            forward: [ 1.0, 0.0, 0.0]
            up: [0.0, 1.0, 0.0]
            fov: 90.0
            distance_to_image: 1.0
            x: 1080
            y: 1080
        "#;
        let cam: Camera = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            cam,
            Camera {
                origin: Point::new(-1., 0., 0.),
                film: Film::new(
                    1080,
                    1080,
                    2.,
                    Point::origin(),
                    Vector::new(0., 1., 0.),
                    Vector::new(0., 0., 1.),
                )
            }
        )
    }
}
