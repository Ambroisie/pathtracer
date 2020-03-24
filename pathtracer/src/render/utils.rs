use crate::Vector;
use nalgebra::Unit;

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
