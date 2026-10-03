use std::fmt::Debug;
use std::fmt::Write;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::materials::material::Material;
use crate::triangle::Triangle;
use crate::vertex::Vertex;
use crate::{Mat4, Vec2, Vec3};
use gltf::{self};

pub struct MeshOptions {
    triangles: Vec<Triangle>,
    aabb: AABB,
}

impl MeshOptions {
    pub fn from_file(filename: String) -> Result<Vec<Self>, String> {
        let (gltf, buffers, _) = gltf::import(filename).map_err(|e| e.to_string())?;

        Ok(gltf
            .meshes()
            .map(|mesh| {
                mesh.primitives().map(|prim| {
                    let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));
                    let indices: Vec<u32> = reader
                        .read_indices()
                        .map(|iter| iter.into_u32().collect())
                        .unwrap_or_default();

                    let positions: Vec<[f32; 3]> = reader
                        .read_positions()
                        .map(|iter| iter.collect())
                        .unwrap_or_default();

                    let normals: Vec<[f32; 3]> = reader
                        .read_normals()
                        .map(|iter| iter.collect())
                        .unwrap_or_default();

                    let uvs: Vec<[f32; 2]> = reader
                        .read_tex_coords(0)
                        .map(|iter| iter.into_f32().collect())
                        .unwrap_or_default();
                    let vertices: Vec<Vertex> = indices
                        .iter()
                        .map(|i| {
                            let position = positions[*i as usize];
                            let normal = normals[*i as usize];
                            let uv = uvs[*i as usize];
                            Vertex {
                                pos: Vec3 {
                                    x: position[0],
                                    y: position[1],
                                    z: position[2],
                                },
                                normal: -Vec3 {
                                    x: normal[0],
                                    y: normal[1],
                                    z: normal[2],
                                },
                                uv: Vec2 { x: uv[0], y: uv[1] },
                            }
                        })
                        .collect();
                    let (mesh_min, mesh_max) = {
                        let mut min = Vec3::MAX;
                        let mut max = Vec3::MIN;
                        for v in &vertices {
                            min = min.min(v.pos);
                            max = max.min(v.pos);
                        }
                        (min, max)
                    };
                    let triangles: Vec<Triangle> = vertices
                        .chunks(3)
                        .map(|vc| Triangle {
                            v0: vc[0],
                            v1: vc[1],
                            v2: vc[2],
                        })
                        .collect();
                    MeshOptions {
                        triangles,
                        aabb: AABB::from_extrema(mesh_min, mesh_max),
                    }
                })
            })
            .flatten()
            .collect())
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
    ) -> Option<crate::hittable::HitRecord<'_>> {
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
                    (self.inverse_transform.transpose() * intersection.normal.extend(0.0))
                        .truncate(),
                    r,
                    self.material,
                    intersection.uv,
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
