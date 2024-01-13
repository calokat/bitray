use crate::{color::Color, rand_vec3::reflect, ray::Ray};

use super::material::{Material, MaterialHitResult};

pub struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: crate::hittable::HitRecord) -> Option<super::material::MaterialHitResult> {
        let reflected = reflect(&r_in.direction, &rec.normal);
        return Some(MaterialHitResult {
            color: self.albedo,
            ray: Ray::new(rec.p, reflected)
        });
    }
}

impl Metal {
    pub fn new(color: Color) -> Self {
        return Self {
            albedo: color
        };
    }
}