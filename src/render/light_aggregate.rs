use crate::light;

#[derive(Debug, PartialEq)]
pub struct LightAggregate {
    ambient_lights: Vec<light::AmbientLight>,
    directional_lights: Vec<light::DirectionalLight>,
    point_lights: Vec<light::PointLight>,
    spot_lights: Vec<light::SpotLight>,
}

impl LightAggregate {
    pub fn empty() -> Self {
        LightAggregate::new(vec![], vec![], vec![], vec![])
    }

    pub fn new(
        ambient_lights: Vec<light::AmbientLight>,
        directional_lights: Vec<light::DirectionalLight>,
        point_lights: Vec<light::PointLight>,
        spot_lights: Vec<light::SpotLight>,
    ) -> Self {
        LightAggregate {
            ambient_lights,
            directional_lights,
            point_lights,
            spot_lights,
        }
    }
}

impl Default for LightAggregate {
    fn default() -> Self {
        LightAggregate::empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_works() {
        let lights = LightAggregate::empty();
        assert_eq!(
            lights,
            LightAggregate {
                ambient_lights: vec![],
                directional_lights: vec![],
                point_lights: vec![],
                spot_lights: vec![],
            }
        )
    }

    #[test]
    fn default_is_empty() {
        let lights = <LightAggregate as Default>::default();
        assert_eq!(lights, LightAggregate::empty())
    }
}
