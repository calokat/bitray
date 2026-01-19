use crate::{color::Color, texture::Sampler2D};

use super::material::Material;

pub struct DiffuseLightMaterial<'a> {
    color: &'a dyn Sampler2D,
}

impl<'a> DiffuseLightMaterial<'a> {
    pub fn new(color: &'a dyn Sampler2D) -> Self {
        Self { color }
    }
}

impl<'a> Material for DiffuseLightMaterial<'a> {
    fn scatter(
        &self,
        _: &crate::ray::Ray,
        _: &crate::hittable::HitRecord,
    ) -> Option<super::material::MaterialHitResult> {
        None
    }

    fn emit_color(&self, _: &crate::ray::Ray, hc: &crate::hittable::HitRecord) -> Color {
        self.color.sample(hc.uv)
    }
}
