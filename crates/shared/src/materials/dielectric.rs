use super::material::{Material, MaterialHitResult};
use crate::color::Color;
use crate::rand_vec3::{reflect, refract};
use crate::ray::Ray;
use crate::Float;
pub struct Dielectric {
    index_of_refraction: Float,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<MaterialHitResult> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = r_in.direction.normalize();

        let cos_theta = Float::min(rec.normal.dot(-unit_direction), 1.0);
        let sin_theta = Float::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand::random() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);
        return Some(MaterialHitResult {
            color: Color::new(1.0, 1.0, 1.0),
            ray: scattered,
            pdf: None,
        });
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: Float) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: Float, ref_idx: Float) -> Float {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * Float::powf(1.0 - cosine, 5.0);
    }
}
