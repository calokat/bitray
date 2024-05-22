use crate::{
    color::Color,
    rand_vec3::{random_unit_vector, reflect},
    ray::Ray,
};

use super::material::{Material, MaterialHitResult};

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Material for Metal {
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
            color: self.albedo,
            ray: scattered,
        });
    }
}

impl Metal {
    pub fn new(color: Color, fuzz: f32) -> Self {
        return Self {
            albedo: color,
            fuzz,
        };
    }
}
