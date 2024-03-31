use glam::Vec3;
use crate::{hittable::{Hittable, HitRecord}, materials::material::Material};
pub struct Triangle<'a> {
    pos0: Vec3,
    pos1: Vec3,
    pos2: Vec3,
    material: &'a dyn Material,
}

impl<'a> Triangle<'a> {
    pub fn new(pos0: Vec3, pos1: Vec3, pos2: Vec3, material: &'a dyn Material) -> Self {
        Self {
            pos0,
            pos1,
            pos2,
            material
        }
    }
}

impl<'a> Hittable for Triangle<'a> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval) -> Option<HitRecord> {
        let p0p1 = self.pos1 - self.pos0;
        let p0p2 = self.pos2 - self.pos0;
        let normal = p0p1.cross(p0p2).normalize();

        if normal.dot(r.direction) == 0.0 {
            return None;
        }
        let d = -normal.dot(self.pos0);
        let t = -(normal.dot(r.origin) + d) / normal.dot(r.direction);
        if t < 0.0 {
            return None;
        }
        let p = r.at(t);

        let edge0 = self.pos1 - self.pos0;
        let edge1 = self.pos2 - self.pos1;
        let edge2 = self.pos0 - self.pos2;

        let c0 = edge0.cross(p - self.pos0);
        let c1 = edge1.cross(p - self.pos1);
        let c2 = edge2.cross(p - self.pos2);

        if normal.dot(c0) > 0.0 && normal.dot(c1) > 0.0 && normal.dot(c2) > 0.0 {
            return Some(HitRecord::new(p, t, &normal, r, self.material));
        }
        
        None
    }
}
