use glam::Vec3;
use std::vec::Vec;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t:  f32,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: f32::default(),
            front_face: true,
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        None
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
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