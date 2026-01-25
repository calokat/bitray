use super::material::MaterialHitResult;
use crate::onb::ONB;
use crate::rand_vec3::random_cosine_direction;
use crate::ray::Ray;
use crate::texture::Sampler2D;
use core::f32;
#[derive(Clone, Copy)]
pub struct Lambert {
    albedo: Sampler2D,
}

impl Lambert {
    pub fn new(color: Sampler2D) -> Self {
        Self { albedo: color }
    }

    pub fn scatter(
        &self,
        _r_in: &Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<super::material::MaterialHitResult> {
        return Some(MaterialHitResult {
            color: self.albedo.sample(rec.uv),
            ray: Ray::new(
                rec.p,
                ONB::new(&rec.normal).transform(&random_cosine_direction()),
            ),
        });
    }

    pub fn scattering_pdf(
        &self,
        _r_in: &Ray,
        hit_record: &crate::hittable::HitRecord,
        scattered_ray: &Ray,
    ) -> f32 {
        let cos_theta = hit_record
            .normal
            .normalize()
            .dot(scattered_ray.direction.normalize());
        (0.0 as f32).max(cos_theta / f32::consts::PI)
    }
}
