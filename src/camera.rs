use glam::Vec3;
use std::fs::OpenOptions;
use crate::hittable::Hittable;
use crate::rand_vec3;
use std::io::Write;
use crate::ray::Ray;
use crate::color::Color;
use crate::interval::Interval;
use rand::prelude::*;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    num_samples: i32,
    max_depth: i32,
}

impl Camera {
    pub fn new(aspect: f32, width: i32, num_samples: i32, max_depth: i32) -> Self {
        let mut cam = Self::default();
        cam.aspect_ratio = aspect;
        cam.image_width = width;
        cam.num_samples = num_samples;
        cam.max_depth = max_depth;
        cam.initialize();
        cam
    }

    pub fn render(&self, world: &dyn Hittable) {

        let mut file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open("image.ppm")
        .unwrap();

        write!(file_out, "P3\n {} {}\n 255\n", self.image_width, self.image_height).unwrap();

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.num_samples {
                    let r = self.get_ray(i, j as f32);
                    pixel_color = pixel_color + Self::ray_color(&r, world, self.max_depth);
                }    
                file_out
                .write(pixel_color.to_ppm_str(self.num_samples).into_bytes().as_ref())
                .unwrap();
            }
            println!("Scanline {} complete", j);
        }    
    }
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Vec3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(ray, Interval { min: 0.001, max: f32::MAX }) {
            if let Some(mat_hit_res) = rec.material.scatter(ray, rec) {
                return mat_hit_res.color * Self::ray_color(&mat_hit_res.ray, world, depth - 1);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }
        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }

    fn get_ray(&self, i: i32, j: f32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square(); 
        
        let ray_direction = pixel_sample - self.center;

        return Ray::new(self.center, ray_direction);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px: f32 = -0.5 + random::<f32>();
        let py: f32 = -0.5 + random::<f32>();

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
    
}