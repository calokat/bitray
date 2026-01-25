use spirv_std::glam::Vec2;

use crate::aabb::AABB;
use crate::interval::Interval;
use crate::materials::material::{Material, MaterialType};
use crate::ray::Ray;
use core::fmt::Debug;
use spirv_std::glam::Vec3;
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: MaterialType,
    pub uv: Vec2,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        t: f32,
        outward_normal: Vec3,
        r: &Ray,
        material: MaterialType,
        uv: Vec2,
    ) -> Self {
        let mut this = Self {
            p,
            normal: Vec3::ONE,
            t,
            front_face: false,
            material,
            uv,
        };

        this.set_face_normal(r, &outward_normal);

        return this;
    }

    pub fn miss() -> Self {
        Self {
            p: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.0,
            front_face: false,
            material: MaterialType::Void,
            uv: Vec2::ZERO,
        }
    }

    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable: Send + Sync + Debug {
    fn hit(&self, r: &Ray, ray_t: Interval) -> (bool, HitRecord);
    fn bounding_box(&self) -> AABB;
}
