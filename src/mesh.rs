use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::materials::material::Material;
use crate::triangle::Triangle;
use crate::vertex::Vertex;
use glam::Vec3;
use russimp::scene::PostProcess;
use russimp::scene::Scene;

pub struct MeshOptions {
    triangles: Vec<Triangle>,
}

impl MeshOptions {
    pub fn from_file(filename: String) -> Self {
        let scene = Scene::from_file(
            &filename,
            vec![
                PostProcess::Triangulate,
                PostProcess::CalculateTangentSpace,
                PostProcess::JoinIdenticalVertices,
                PostProcess::SortByPrimitiveType,
            ],
        )
        .unwrap();

        let mut triangles: Vec<Triangle> = Vec::new();

        for m in scene.meshes.iter() {
            for f in m.faces.iter() {
                let mut triangle_face: Vec<Vertex> = Vec::new();
                for i in f.0.iter() {
                    let pos = m.vertices[*i as usize];
                    let normal = m.normals[*i as usize];
                    let v = Vertex {
                        pos: Vec3::new(pos.x, pos.y, pos.z),
                        normal: Vec3::new(normal.x, normal.y, normal.z),
                    };
                    triangle_face.push(v);
                }
                assert!(triangle_face.len() == 3);
                triangles.push(Triangle {
                    v0: triangle_face[0],
                    v1: triangle_face[1],
                    v2: triangle_face[2],
                });
            }
        }

        Self { triangles }
    }
}

pub struct Mesh<'a> {
    options: &'a MeshOptions,
    material: &'a dyn Material,
}

impl<'a> Mesh<'a> {
    pub fn new(options: &'a MeshOptions, material: &'a dyn Material) -> Self {
        Self { options, material }
    }
}

impl<'a> Hittable for Mesh<'a> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        for t in self.options.triangles.iter() {
            if let Some(intersection) = t.ray_hit(r, &ray_t) {
                return Some(HitRecord::new(
                    intersection.p,
                    intersection.t,
                    &intersection.normal,
                    r,
                    self.material,
                ));
            }
        }
        return None;
    }
}
