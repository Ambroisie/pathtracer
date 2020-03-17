use crate::{Point, Vector};

/// Represent an abstract camera film, to know where each pixel is in space.
#[derive(Debug, PartialEq)]
pub struct Film {
    x: u32,
    y: u32,
    center: Point,
    ratio_up: Vector,
    ratio_right: Vector,
}

impl Film {
    pub fn new(x: u32, y: u32, screen_size: f32, center: Point, up: Vector, right: Vector) -> Self {
        let (x_size, y_size) = if x > y {
            (screen_size, screen_size * y as f32 / x as f32)
        } else {
            (screen_size * x as f32 / y as f32, screen_size)
        };
        Film {
            x,
            y,
            center,
            ratio_up: up.normalize() * x_size,
            ratio_right: right.normalize() * y_size,
        }
    }

    pub fn width(&self) -> u32 {
        self.x
    }

    pub fn height(&self) -> u32 {
        self.y
    }

    pub fn pixel_ratio(&self, x: f32, y: f32) -> (f32, f32) {
        (x / self.x as f32, y / self.y as f32)
    }

    pub fn pixel_at_ratio(&self, x: f32, y: f32) -> Point {
        let delt_x = x - 0.5;
        let delt_y = 0.5 - y;
        self.center + self.ratio_right * delt_x + self.ratio_up * delt_y
    }

    pub fn pixel_at_coord(&self, x: u32, y: u32) -> Point {
        let (x, y) = self.pixel_ratio(x as f32, y as f32);
        self.pixel_at_ratio(x, y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_new_works() {
        let film = Film::new(
            1080,
            1080,
            1.,
            Point::origin(),
            Vector::new(0., 1., 0.),
            Vector::new(0., 0., 1.),
        );
        assert_eq!(
            film,
            Film {
                x: 1080,
                y: 1080,
                center: Point::origin(),
                ratio_up: Vector::new(0., 1., 0.),
                ratio_right: Vector::new(0., 0., 1.),
            }
        )
    }

    #[test]
    fn new_with_smaller_x_works() {
        let film = Film::new(
            1080,
            1440,
            1.,
            Point::origin(),
            Vector::new(0., 1., 0.),
            Vector::new(0., 0., 1.),
        );
        assert_eq!(
            film,
            Film {
                x: 1080,
                y: 1440,
                center: Point::origin(),
                ratio_up: Vector::new(0., 0.75, 0.),
                ratio_right: Vector::new(0., 0., 1.),
            }
        )
    }
    #[test]
    fn new_with_smaller_y_works() {
        let film = Film::new(
            1080,
            540,
            1.,
            Point::origin(),
            Vector::new(0., 1., 0.),
            Vector::new(0., 0., 1.),
        );
        assert_eq!(
            film,
            Film {
                x: 1080,
                y: 540,
                center: Point::origin(),
                ratio_up: Vector::new(0., 1., 0.),
                ratio_right: Vector::new(0., 0., 0.5),
            }
        )
    }

    fn simple_film() -> Film {
        Film::new(
            1080,
            1080,
            1.,
            Point::origin(),
            Vector::new(0., 1., 0.),
            Vector::new(0., 0., 1.),
        )
    }

    #[test]
    fn pixel_ratio_works() {
        let film = simple_film();
        assert_eq!(film.pixel_ratio(0., 0.), (0., 0.));
        assert_eq!(film.pixel_ratio(1080., 1080.), (1., 1.));
        assert_eq!(film.pixel_ratio(1080., 540.), (1., 0.5));
        assert_eq!(film.pixel_ratio(540., 1080.), (0.5, 1.));
        assert_eq!(film.pixel_ratio(1080., 810.), (1., 0.75));
        assert_eq!(film.pixel_ratio(810., 1080.), (0.75, 1.))
    }

    #[test]
    fn pixel_at_ratio_works() {
        let film = simple_film();
        assert_eq!(film.pixel_at_ratio(0., 0.), Point::new(0., 0.5, -0.5));
        assert_eq!(film.pixel_at_ratio(1., 1.), Point::new(0., -0.5, 0.5));
        assert_eq!(film.pixel_at_ratio(1., 0.5), Point::new(0., 0., 0.5));
        assert_eq!(film.pixel_at_ratio(0.5, 1.), Point::new(0., -0.5, 0.));
    }

    #[test]
    fn pixel_at_coord_works() {
        let film = simple_film();
        assert_eq!(film.pixel_at_coord(0, 0), Point::new(0., 0.5, -0.5));
        assert_eq!(film.pixel_at_coord(1080, 1080), Point::new(0., -0.5, 0.5));
        assert_eq!(film.pixel_at_coord(1080, 540), Point::new(0., 0., 0.5));
        assert_eq!(film.pixel_at_coord(540, 1080), Point::new(0., -0.5, 0.));
    }
}
