use crate::aabb::AABB;
use crate::interval::Interval;
use crate::materials::material::Material;
use crate::ray::Ray;
use core::fmt::Debug;
use glam::Vec3;
use std::vec::Vec;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Vec3,
        t: f32,
        outward_normal: &Vec3,
        r: &Ray,
        material: &'a dyn Material,
    ) -> Self {
        let mut this = Self {
            p,
            normal: Vec3::ONE,
            t,
            front_face: false,
            material,
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

pub trait Hittable: Send + Sync + Debug {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
    fn get_name(&self) -> &String;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    aabb: AABB,
    name: String,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        let mut bb = AABB::default();
        for h in &objects {
            bb = bb.to_contain(&h.bounding_box());
        }
        // println!(
        //     "x {:?} y {:?} z {:?} len {:?}",
        //     bb.x,
        //     bb.y,
        //     bb.z,
        //     objects.len()
        // );
        Self {
            objects,
            aabb: bb,
            name: "List".into(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(
                r,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            ) {
                closest_so_far = hit_record.t;
                rec = Some(hit_record);
            }
        }

        return rec;
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }

    fn get_name(&self) -> &String {
        return &self.name;
    }
}

impl Debug for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        for h in &self.objects {
            f.write_fmt(format_args!(" {:?}\n", h))?;
        }
        if self.objects.is_empty() {
            f.write_str("EMPTY\n")?;
        }
        Ok(())
    }
}
