use crate::Float;
use crate::Vec2;
use crate::interval::Interval;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::{Vec3, aabb::AABB};
use core::fmt::Debug;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
    pub material: Material,
    pub uv: Vec2,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        t: Float,
        outward_normal: Vec3,
        r: &Ray,
        material: Material,
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> bool;
    fn bounding_box(&self) -> AABB;
}
