//! Utility module to compute overall illumination

use crate::light::*;
use serde::Deserialize;
use std::iter::Iterator;

#[derive(Debug, PartialEq, Deserialize)]
/// A struct centralizing the light computation logic.
pub struct LightAggregate {
    #[serde(default)]
    ambients: Vec<AmbientLight>,
    #[serde(default)]
    directionals: Vec<DirectionalLight>,
    #[serde(default)]
    points: Vec<PointLight>,
    #[serde(default)]
    spots: Vec<SpotLight>,
}

impl LightAggregate {
    /// Creates a new empty `LightAggregate`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::scene::LightAggregate;
    /// #
    /// let la = LightAggregate::empty();
    /// assert_eq!(la.ambient_lights_iter().count(), 0);
    /// assert_eq!(la.spatial_lights_iter().count(), 0);
    /// ```
    pub fn empty() -> Self {
        LightAggregate::new(vec![], vec![], vec![], vec![])
    }

    /// Creates a new `LightAggregate` from `Vec`s of [`Light`]s.
    ///
    /// [`Light`]: ../../light/trait.Light.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use pathtracer::scene::LightAggregate;
    /// #
    /// let la = LightAggregate::new(
    ///     Vec::new(),
    ///     Vec::new(),
    ///     Vec::new(),
    ///     Vec::new(),
    /// );
    /// assert_eq!(la.ambient_lights_iter().count(), 0);
    /// assert_eq!(la.spatial_lights_iter().count(), 0);
    /// ```
    pub fn new(
        ambients: Vec<AmbientLight>,
        directionals: Vec<DirectionalLight>,
        points: Vec<PointLight>,
        spots: Vec<SpotLight>,
    ) -> Self {
        LightAggregate {
            ambients,
            directionals,
            points,
            spots,
        }
    }

    /// Returns an iterator over the aggregate's [`AmbientLight`]s.
    ///
    /// [`AmbientLight`]: ../../light/ambient_light/struct.AmbientLight.html
    pub fn ambient_lights_iter(&self) -> impl Iterator<Item = &'_ dyn Light> {
        self.ambients.iter().map(|l| l as &dyn Light)
    }

    /// Returns an iterator over the aggregate's [`SpatialLight`]s.
    ///
    /// This simply merges iterators over [`DirectionalLight`], [`PointLight`] and [`SpotLight`].
    ///
    /// [`SpatialLight`]: ../../light/trait.SpatialLight.html
    /// [`DirectionalLight`]: ../../light/directional_light/struct.DirectionalLight.html
    /// [`PointLight`]: ../../light/point_light/struct.PointLight.html
    /// [`Spotight`]: ../../light/spot_light/struct.Spotight.html
    pub fn spatial_lights_iter(&self) -> impl Iterator<Item = &'_ dyn SpatialLight> {
        self.directionals
            .iter()
            .map(|l| l as &dyn SpatialLight)
            .chain(self.points.iter().map(|l| l as &dyn SpatialLight))
            .chain(self.spots.iter().map(|l| l as &dyn SpatialLight))
    }

    /// Returns an iterator over the aggregate's [`SampleLight`]s.
    ///
    /// This simply merges iterators over [`SpotLight`], and [`PointLight`].
    ///
    /// [`SampleLight`]: ../../light/trait.SampleLight.html
    /// [`PointLight`]: ../../light/point_light/struct.PointLight.html
    /// [`Spotight`]: ../../light/spot_light/struct.Spotight.html
    pub fn sample_lights_iter(&self) -> impl Iterator<Item = &dyn SampleLight> {
        self.spots
            .iter()
            .map(|sl| sl as &dyn SampleLight)
            .chain(self.points.iter().map(|pl| pl as &dyn SampleLight))
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
                ambients: vec![],
                directionals: vec![],
                points: vec![],
                spots: vec![],
            }
        )
    }

    #[test]
    fn default_is_empty() {
        let lights = <LightAggregate as Default>::default();
        assert_eq!(lights, LightAggregate::empty())
    }

    #[test]
    fn deserialization_works() {
        use crate::{core::LinearColor, Point, Vector};

        let yaml = r#"
            ambients:
              - color: {r: 1.0, g: 0.5, b: 0.2}
            directionals:
              - direction: [1.0, 0.0, 0.0]
                color: {r: 1.0, g: 0.5, b: 0.2}
            points:
              - position: [1.0, 1.0, 1.0]
                color: {r: 1.0, g: 0.5, b: 0.2}
            spots:
              - position: [0.0, 0.0, 0.0]
                direction: [1.0, 0.0, 0.0]
                fov: 90.0
                color: {r: 1.0, g: 0.5, b: 0.2}
        "#;
        let expected = LightAggregate::new(
            vec![AmbientLight::new(LinearColor::new(1., 0.5, 0.2))],
            vec![DirectionalLight::new(
                Vector::x_axis(),
                LinearColor::new(1., 0.5, 0.2),
            )],
            vec![PointLight::new(
                Point::new(1., 1., 1.),
                LinearColor::new(1., 0.5, 0.2),
            )],
            vec![SpotLight::degrees_new(
                Point::origin(),
                Vector::x_axis(),
                90.,
                LinearColor::new(1., 0.5, 0.2),
            )],
        );
        let lights: LightAggregate = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(lights, expected)
    }
}
