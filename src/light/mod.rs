use super::core::LinearColor;
use super::Point;

/// Represent a light in the scene being rendered.
pub trait Light: std::fmt::Debug {
    /// Get the illumination of that light on that point.
    fn illumination(&self, point: &Point) -> LinearColor;
}
