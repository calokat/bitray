use crate::color::Color;

use super::material::Material;

pub struct DiffuseLightMaterial {
    color: Color,
}

impl DiffuseLightMaterial {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for DiffuseLightMaterial {
    fn scatter(
        &self,
        _: &crate::ray::Ray,
        _: &crate::hittable::HitRecord,
    ) -> Option<super::material::MaterialHitResult> {
        None
    }

    fn emit_color(&self, _: &crate::ray::Ray, _: &crate::hittable::HitRecord) -> Color {
        self.color
    }
}
