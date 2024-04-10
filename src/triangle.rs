use glam::Vec3;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vertex::Vertex;

pub struct Triangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

pub struct TriangleRayIntersection {
    pub t: f32,
    pub normal: Vec3,
    pub p: Vec3,
}

impl Triangle {
    pub fn ray_hit(&self, r: &Ray, ray_t: &Interval) -> Option<TriangleRayIntersection> {
        let v0v1 = self.v1.pos - self.v0.pos;
        let v0v2 = self.v2.pos - self.v0.pos;
        let pvec = r.direction.cross(v0v2);
        let det = v0v1.dot(pvec);
        if det < f32::EPSILON {
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
        return Some(TriangleRayIntersection {
            t,
            normal,
            p: r.at(t),
        });
    }
}
