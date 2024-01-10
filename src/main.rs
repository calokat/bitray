use glam::Vec3;
use ray_tracing_weekend_rs::color::Color;
use ray_tracing_weekend_rs::ray::Ray;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::ops::{Div, Mul};

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> Option<f32> {
    let oc: Vec3 = r.origin - *center;
    let a = r.direction.length_squared();
    let half_b = Vec3::dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    } else {
        return Some((-half_b - f32::sqrt(discriminant)) / a);
    }
}

fn ray_color(ray: Ray) -> Color {
    if let Some(t) = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        let n = ray.at(t) - Vec3::new(0.0, 0.0, -1.0);
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_dir = ray.direction.normalize();
    let a = 0.5 * (unit_dir.y + 1.0);
    assert!(a >= 0.0 && a <= 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u.div(image_width as f32);
    let pixel_delta_v = viewport_v.div(image_height as f32);

    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u.mul(0.5)
        - viewport_v.mul(0.5);

    let pixel00_loc = viewport_upper_left + (0.5 * (pixel_delta_u + pixel_delta_v));

    let mut file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .open("image.ppm")
        .unwrap();

    write!(file_out, "P3\n {} {}\n 255\n", image_width, image_height).unwrap();
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray: Ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(ray);

            file_out
                .write(color.to_ppm_str().into_bytes().as_ref())
                .unwrap();
        }
    }
}
