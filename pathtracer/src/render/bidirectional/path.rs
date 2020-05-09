use crate::core::LinearColor;
use crate::Point;

pub struct PathPoint {
    pub point: Point,
    pub luminance: LinearColor,
}

impl PathPoint {
    #[allow(unused)]
    pub fn new(point: Point, luminance: LinearColor) -> Self {
        PathPoint { point, luminance }
    }
}

pub struct Path {
    pub origin: Point,
    pub points: Vec<PathPoint>,
}

impl Path {
    #[allow(unused)]
    pub fn new(origin: Point) -> Self {
        Path {
            origin,
            points: Vec::new(),
        }
    }

    #[allow(unused)]
    pub fn push_point(&mut self, new_point: PathPoint) {
        self.points.push(new_point)
    }
}
