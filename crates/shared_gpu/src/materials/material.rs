use crate::color::Color;
use crate::materials::lambert::Lambert;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct MaterialHitResult {
    pub color: Color,
    pub ray: Ray,
}

#[derive(Clone, Copy)]
pub enum MaterialType {
    Lambert,
    Void,
}

#[derive(Clone, Copy)]
pub enum Material {
    LambertMaterial(Lambert),
    VoidMaterial,
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<MaterialHitResult> {
        match &self {
            &Material::LambertMaterial(lambert) => lambert.scatter(r_in, rec),
            &Material::VoidMaterial => None,
        }
    }
}
