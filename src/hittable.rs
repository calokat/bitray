use glam::Vec3;
use std::vec::Vec;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t:  f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, t: f32, outward_normal: &Vec3, r: &Ray, material: &'a dyn Material) -> Self {
        let mut this = Self {
            p,
            normal: Vec3::ONE,
            t,
            front_face: false,
            material
        };

        this.set_face_normal(r, outward_normal);

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

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl Hittable for Vec<&dyn Hittable> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max;

        for object in self {
            if let Some(hit_record) = object.hit(r, Interval { min: ray_t.min, max: closest_so_far }) {
                closest_so_far = hit_record.t;
                rec = Some(hit_record);
            }
        }

        return rec;
    }
}