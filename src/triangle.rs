use std::fmt::Debug;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::material::Material;
use crate::{Vec2, Vec3};

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vertex::Vertex;
use crate::Float;

pub struct Triangle<'a> {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
    pub material: &'a dyn Material,
}

pub struct TriangleRayIntersection {
    pub t: Float,
    pub normal: Vec3,
    pub p: Vec3,
    pub uv: Vec2,
}

impl<'a> Triangle<'a> {
    pub fn ray_hit(&self, r: &Ray, _ray_t: &Interval) -> Option<TriangleRayIntersection> {
        let v0v1 = self.v1.pos - self.v0.pos;
        let v0v2 = self.v2.pos - self.v0.pos;
        let pvec = r.direction.cross(v0v2);
        let det = v0v1.dot(pvec);
        if det < Float::EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
        let tvec = r.origin - self.v0.pos;
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let qvec = tvec.cross(v0v1);
        let v = r.direction.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let normal =
            ((1.0 - u - v) * self.v0.normal + u * self.v1.normal + v * self.v2.normal).normalize();
        let t = v0v2.dot(qvec) * inv_det - 0.1;
        let tex_uv = ((1.0 - u - v) * self.v0.uv + u * self.v1.uv + v * self.v2.uv).normalize();

        if t < 0.0 {
            return None;
        }
        return Some(TriangleRayIntersection {
            t,
            normal,
            p: r.at(t),
            uv: tex_uv,
        });
    }
}

impl<'a> Debug for Triangle<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Triangle")?;
        Ok(())
    }
}

impl<'a> Hittable for Triangle<'a> {
    fn bounding_box(&self) -> crate::aabb::AABB {
        let aabb = AABB::default();
        let aabb = aabb.to_contain_point(self.v0.pos);
        let aabb = aabb.to_contain_point(self.v1.pos);
        let aabb = aabb.to_contain_point(self.v2.pos);
        return aabb;
    }
    fn centroid(&self) -> Vec3 {
        self.v0.pos + self.v1.pos + self.v2.pos / 3.0
    }

    fn get_name(&self) -> String {
        "Triangle".to_string()
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<crate::hittable::HitRecord<'_>> {
        let ray_hit = self.ray_hit(r, &ray_t);
        ray_hit.and_then(|rh| {
            Some(HitRecord::new(
                rh.p,
                rh.t,
                rh.normal,
                r,
                self.material,
                rh.uv,
            ))
        })
    }

    fn surface_area(&self) -> f32 {
        let base = self.v2.pos - self.v0.pos;
        let hypotenuse = self.v1.pos - self.v0.pos;
        let angle = base.angle_between(hypotenuse);
        let sine = angle.sin();
        let height = sine * hypotenuse.length();
        return 0.5 * base.length() * height;
    }
}
