use glam::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::HitRecord;
pub struct MaterialHitResult {
    pub color: Color,
    pub ray: Ray,
}
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: HitRecord) -> Option<MaterialHitResult>;
}