use glam::Vec3;
use wavefront_obj::obj::{ObjSet, Primitive, Vertex};
use crate::{hittable::{Hittable, HitRecord}, materials::material::Material};
pub struct Mesh<'a> {
    obj: ObjSet,
    translation: Vec3,
    material: &'a dyn Material,
}

impl<'a> Mesh<'a> {
    pub fn new(obj: ObjSet, translation: Vec3, material: &'a dyn Material) -> Self {
        Self {
            obj,
            translation,
            material,
        }
    }
}

impl<'a> Hittable for Mesh<'a> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval) -> Option<HitRecord> {
        for obj in self.obj.objects.iter() {
            for geom in obj.geometry.iter() {
                for shape in geom.shapes.iter() {
                    if let Primitive::Triangle(v0, v1, v2) = shape.primitive {
                        let pos0 = obj.vertices[v0.0];
                        let pos1 = obj.vertices[v1.0];
                        let pos2 = obj.vertices[v2.0];

                        let pos0 = Vec3::new(pos0.x as f32, pos0.y as f32, pos0.z as f32);
                        let pos1 = Vec3::new(pos1.x as f32, pos1.y as f32, pos1.z as f32);
                        let pos2 = Vec3::new(pos2.x as f32, pos2.y as f32, pos2.z as f32);

                        let normal = match v0.2 {
                            Some(index) => {
                                let normal = obj.normals[index];
                                Vec3::new(normal.x as f32, normal.y as f32, normal.z as f32).normalize()
                            },
                            None => {
                                let p0p1 = pos1 - pos0;
                                let p0p2 = pos2 - pos0;
                                -p0p1.cross(p0p2)
                            }
                        };

                        if normal.dot(r.direction) == 0.0 {
                            return None;
                        }
                        let d = -normal.dot(pos0);
                        let t = -(normal.dot(r.origin) + d) / normal.dot(r.direction);
                        if t < 0.0 {
                            return None;
                        }
                        let p = r.at(t);

                        let edge0 = pos1 - pos0;
                        let edge1 = pos2 - pos1;
                        let edge2 = pos0 - pos2;

                        let c0 = edge0.cross(p - pos0);
                        let c1 = edge1.cross(p - pos1);
                        let c2 = edge2.cross(p - pos2);

                        if normal.dot(c0) > 0.0 && normal.dot(c1) > 0.0 && normal.dot(c2) > 0.0 {
                            return Some(HitRecord::new(p, t, &normal, r, self.material));
                        }
                    }
                }
            }
        }
        None
    }
}
