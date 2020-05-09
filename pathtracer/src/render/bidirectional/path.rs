use crate::core::LightProperties;
use crate::{Point, Vector};
use nalgebra::Unit;

pub struct PathPoint {
    pub point: Point,
    pub pdf: f32,
    pub properties: LightProperties,
}

impl PathPoint {
    #[allow(unused)]
    pub fn new(
        point: Point,
        pdf: 32,
        properties: LightProperties,
    ) -> Self {
        PathPoint {
            point,
            pdf,
            properties,
        }
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
