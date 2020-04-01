use crate::core::LightProperties;
use crate::{Point, Vector};
use nalgebra::Unit;

pub struct PathPoint {
    pub point: Point,
    pub incident: Unit<Vector>,
    pub normal: Unit<Vector>,
    pub properties: LightProperties,
}

impl PathPoint {
    #[allow(unused)]
    pub fn new(
        point: Point,
        incident: Unit<Vector>,
        normal: Unit<Vector>,
        properties: LightProperties,
    ) -> Self {
        PathPoint {
            point,
            incident,
            normal,
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
