use std::fmt::Debug;
use std::fmt::Write;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::materials::material::Material;
use crate::triangle::Triangle;
use crate::vertex::Vertex;
use glam::{Mat4, Vec3};
use russimp::scene::PostProcess;
use russimp::scene::Scene;

pub struct MeshOptions {
    triangles: Vec<Triangle>,
    aabb: AABB,
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
                PostProcess::GenerateBoundingBoxes,
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

        let russimp_aabb = scene.meshes[0].aabb;
        let aabb_min = Vec3::new(russimp_aabb.min.x, russimp_aabb.min.y, russimp_aabb.min.z);
        let aabb_max = Vec3::new(russimp_aabb.max.x, russimp_aabb.max.y, russimp_aabb.max.z);

        Self {
            triangles,
            aabb: AABB::from_extrema(aabb_min, aabb_max),
        }
    }
}

pub struct Mesh<'a> {
    options: &'a MeshOptions,
    material: &'a dyn Material,
    name: String,
    transform: Mat4,
    inverse_transform: Mat4,
    aabb: AABB,
}

impl<'a> Mesh<'a> {
    pub fn new(
        options: &'a MeshOptions,
        material: &'a dyn Material,
        name: String,
        transform: Mat4,
    ) -> Self {
        let aabb = options.aabb.transform(transform);
        Self {
            options,
            material,
            name,
            transform,
            inverse_transform: transform.inverse(),
            aabb,
        }
    }
}

impl<'a> Hittable for Mesh<'a> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        for t in self.options.triangles.iter() {
            let mut rotated_ray = r.clone();
            rotated_ray.origin =
                (self.inverse_transform * rotated_ray.origin.extend(1.0)).truncate();
            rotated_ray.direction =
                (self.inverse_transform * rotated_ray.direction.extend(0.0)).truncate();
            if let Some(intersection) = t.ray_hit(&rotated_ray, &ray_t) {
                return Some(HitRecord::new(
                    (self.transform * intersection.p.extend(1.0)).truncate(),
                    intersection.t,
                    &intersection.normal,
                    r,
                    self.material,
                ));
            }
        }
        return None;
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

impl<'a> Debug for Mesh<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(" Mesh ")?;
        f.write_str(&self.name)?;
        f.write_char('\n')?;
        Ok(())
    }
}
