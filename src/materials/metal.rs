use crate::{
    rand_vec3::{random_unit_vector, reflect},
    ray::Ray,
    texture::Sampler2D,
};

use super::material::{Material, MaterialHitResult};

pub struct Metal<'a> {
    albedo: &'a dyn Sampler2D,
    fuzz: f32,
}

impl<'a> Material for Metal<'a> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<super::material::MaterialHitResult> {
        let reflected = reflect(&r_in.direction, &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_unit_vector());
        if rec.normal.dot(scattered.direction) < 0.0 {
            return None;
        }
        return Some(MaterialHitResult {
            color: self.albedo.sample(rec.uv),
            ray: scattered,
        });
    }
}

impl<'a> Metal<'a> {
    pub fn new(albedo: &'a dyn Sampler2D, fuzz: f32) -> Self {
        return Self { albedo, fuzz };
    }
}
