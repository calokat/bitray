use crate::color::Color;
use crate::hittable::HitRecord;
use crate::pdf::PDF;
use crate::ray::Ray;
use crate::Float;
pub struct MaterialHitResult {
    pub color: Color,
    pub ray: Ray,
    pub pdf: Option<Box<dyn PDF>>,
}
pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<MaterialHitResult> {
        None
    }
    fn emit_color(&self, _: &Ray, _: &HitRecord) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
    fn scattering_pdf(&self, _r_in: &Ray, _hit_record: &HitRecord, _scattered_ray: &Ray) -> Float {0.0}
}
