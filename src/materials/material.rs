use crate::color::Color;
use crate::hittable::HitRecord;
use crate::pdf::PDF;
use crate::ray::Ray;
pub struct MaterialHitResult {
    pub color: Color,
    pub ray: Ray,
    pub pdf: Option<Box<dyn PDF>>,
}
pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialHitResult> {
        None
    }
    fn emit_color(&self, _: &Ray, _: &HitRecord) -> Color {
        return Color::new(0.0, 0.0, 0.0);
    }
    fn scattering_pdf(&self, r_in: &Ray, hit_record: &HitRecord, scattered_ray: &Ray) -> f32 {0.0}
}

pub struct EmptyMaterial {}

impl Material for EmptyMaterial {}