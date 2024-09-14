use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
pub struct MaterialHitResult {
    pub color: Color,
    pub ray: Ray,
}
pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialHitResult>;
    fn emit_color(&self, _: &Ray, _: &HitRecord) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
}
