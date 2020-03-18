use super::film::Film;
use crate::{Point, Vector};
use serde::{Deserialize, Deserializer};

/// Represent an abstract camera to observe the scene.
#[derive(Debug, PartialEq)]
pub struct Camera {
    /// Where the camera is set in the scene (i.e: its focal point).
    origin: Point,
    /// The film to represent each pixel in the scene.
    film: Film,
}

impl Camera {
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

    pub fn film(&self) -> &Film {
        &self.film
    }

    pub fn origin(&self) -> &Point {
        &self.origin
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

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let cam: SerializedCamera = Deserialize::deserialize(deserializer)?;
        Ok(cam.into())
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
