use crate::core::LinearColor;
use crate::Vector;
use image::RgbImage;
use nalgebra::Unit;
use rand::prelude::thread_rng;
use rand::Rng;

pub fn reflected(incident: Unit<Vector>, normal: Unit<Vector>) -> Unit<Vector> {
    let proj = incident.dot(&normal);
    let delt = normal.into_inner() * (proj * 2.);
    Unit::new_normalize(incident.as_ref() - delt)
}

/// Returns None if the ray was totally reflected, Some(refracted_ray, reflected_amount) if not
pub fn refracted(
    incident: Unit<Vector>,
    normal: Unit<Vector>,
    indices: &mut RefractionInfo,
    new_index: f32,
) -> Option<(Unit<Vector>, f32)> {
    let cos1 = incident.dot(&normal);
    let normal = if cos1 < 0. {
        // Entering object, change the medium
        indices.enter_medium(new_index); // The old index is now in old_index
        normal
    } else {
        // Exiting object, exit the medium
        indices.exit_medium(); // We swapped the indices
        -normal
    };
    let (n_1, n_2) = (indices.old_index, indices.new_index);
    let eta = n_1 / n_2;
    let k = 1. - eta * eta * (1. - cos1 * cos1);
    if k < 0. {
        return None;
    }
    let cos1 = cos1.abs();
    let cos2 = k.sqrt();
    let refracted = eta * incident.as_ref() + (eta * cos1 - cos2) * normal.as_ref();
    let f_r = (n_2 * cos1 - n_1 * cos2) / (n_2 * cos1 + n_1 * cos2);
    let f_t = (n_1 * cos2 - n_2 * cos1) / (n_1 * cos2 + n_2 * cos1);
    let refl_t = (f_r * f_r + f_t * f_t) / 2.;
    //Some((refracted, 0.))
    Some((Unit::new_normalize(refracted), refl_t))
}

#[derive(Debug, PartialEq, Clone)]
pub struct RefractionInfo {
    pub old_index: f32,
    pub new_index: f32,
}

impl RefractionInfo {
    pub fn with_index(index: f32) -> Self {
        RefractionInfo {
            old_index: index,
            new_index: index,
        }
    }

    pub fn enter_medium(&mut self, index: f32) {
        *self = RefractionInfo {
            old_index: self.new_index,
            new_index: index,
        }
    }

    pub fn exit_medium(&mut self) {
        std::mem::swap(&mut self.old_index, &mut self.new_index)
    }
}

/// Returns a random ray in the hemisphere described by a normal unit-vector
/// It is cosine-sampled, which is convenient for path-tracing.
pub fn sample_hemisphere(normal: Unit<Vector>) -> Unit<Vector> {
    let mut rng = thread_rng();
    let azimuth = rng.gen::<f32>() * std::f32::consts::PI * 2.;
    // Cosine weighted importance sampling
    let cos_elevation: f32 = rng.gen();
    let sin_elevation = f32::sqrt(1. - cos_elevation * cos_elevation);

    let x = sin_elevation * azimuth.cos();
    let y = cos_elevation;
    let z = sin_elevation * azimuth.sin();

    // Calculate orthonormal base, defined by (normalb_b, normal, normal_t)
    // Pay attention to degenerate cases when (y, z) is small for use with cross product
    let normal_t = if normal.x.abs() > normal.y.abs() {
        Vector::new(normal.z, 0., -normal.x).normalize()
    } else {
        Vector::new(0., -normal.z, normal.y).normalize()
    };
    let normal_b = normal.cross(&normal_t);

    // Perform the matrix calculation by hand...
    // The probability to have picked the ray is inversely proportional to cosine of the angle with
    // the normal
    Unit::new_normalize(Vector::new(
        x * normal_b.x + y * normal.x + z * normal_t.x,
        x * normal_b.y + y * normal.y + z * normal_t.y,
        x * normal_b.z + y * normal.z + z * normal_t.z,
    ))
}

pub fn buffer_to_image(buffer: &[LinearColor], passes: u32, width: u32, height: u32) -> RgbImage {
    let mut image = RgbImage::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let i = x as usize + y as usize * width as usize;
        *pixel = (buffer[i].clone() / passes as f32).into();
    }

    image
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_hemisphere_work() {
        // NOTE(Bruno): should use some test-case generation for failure-reproduction purposes...
        let mut rng = thread_rng();
        for _ in 0..100 {
            let normal = Unit::new_normalize(Vector::new(rng.gen(), rng.gen(), rng.gen()));
            for _ in 0..100 {
                let (sample, proportion) = sample_hemisphere(normal);
                let cos_angle = normal.dot(&sample);
                assert!(cos_angle >= 0.);
                assert!(1. / cos_angle - proportion < std::f32::EPSILON);
            }
        }
    }
}
