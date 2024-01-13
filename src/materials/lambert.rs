use super::material::{Material, MaterialHitResult};
use crate::color::Color;
use crate::ray::Ray;
use glam::Vec3;
pub struct Lambert {
    albedo: Color
}

impl Material for Lambert {
    fn scatter(&self, r_in: &Ray, rec: crate::hittable::HitRecord) -> Option<super::material::MaterialHitResult> {
        let scatter_direction = rec.normal + crate::rand_vec3::random_unit_vector();
        let scattered_ray = Ray::new(rec.p, scatter_direction);
        if scatter_direction.abs().abs_diff_eq(Vec3::ZERO, 0.001) {
            return Some(MaterialHitResult {
                color: self.albedo,
                ray: scattered_ray
            })
        }
        return Some(MaterialHitResult {
            color: self.albedo,
            ray: scattered_ray
        });
    }
}

impl Lambert {
    pub fn new(color: Color) -> Self {
        Self {
            albedo: color
        }
    }
}