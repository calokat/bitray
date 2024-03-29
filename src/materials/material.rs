use crate::ray::Ray;
use crate::color::Color;
use crate::hittable::HitRecord;
pub struct MaterialHitResult {
    pub color: Color,
    pub ray: Ray,
}
pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialHitResult>;
}