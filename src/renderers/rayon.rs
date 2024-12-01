use crate::{
    camera::Camera,
    color::Color,
    hittable::Hittable,
    interval::Interval,
    pdf::{HittablePDF, MixturePDF, PDF},
    rand_vec3::random_vec_unit_disk,
    ray::Ray,
    render_parameters::RenderParameters,
    Float, Vec3,
};

use rand::random;
use rayon::prelude::*;

pub fn render(
    camera: &Camera,
    world: &dyn Hittable,
    importants: &dyn Hittable,
    render_params: RenderParameters,
) -> Vec<Vec<Color>> {
    let input_row: Vec<(i32, i32)> = vec![(0, 0); render_params.image_width as usize];
    let mut image: Vec<Vec<(i32, i32)>> =
        vec![input_row.clone(); render_params.image_height as usize];
    for j in 0..image.len() {
        for i in 0..input_row.len() {
            image[j][i] = (j as i32, i as i32);
        }
    }

    let rendered_image: Vec<Vec<Color>> = image
        .par_iter()
        .map(|row| {
            row.iter()
                .map(|(j, i)| {
                    let mut color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..render_params.num_samples {
                        color += ray_color(
                            &generate_ray(camera, (*i, *j)),
                            world,
                            importants,
                            render_params.max_depth,
                            Color::new(0.0, 0.0, 0.0),
                        )
                        .clamp();
                    }
                    color.correct_nans();
                    color
                })
                .collect()
        })
        .collect();

    return rendered_image;
}

fn ray_color(
    ray: &Ray,
    world: &dyn Hittable,
    important_objs: &dyn Hittable,
    depth: i32,
    background_color: Color,
) -> Color {
    if depth <= 0 {
        return Color::new(1.0, 1.0, 1.0);
    }
    if let Some(rec) = world.hit(
        ray,
        Interval {
            min: 0.001,
            max: Float::MAX,
        },
    ) {
        if let Some(mat_hit_res) = rec.material.scatter(ray, &rec) {
            if mat_hit_res.pdf.is_none() {
                return mat_hit_res.color
                    * ray_color(
                        &mat_hit_res.ray,
                        world,
                        important_objs,
                        depth - 1,
                        background_color,
                    );
            }
            let mat_pdf = mat_hit_res.pdf.unwrap();
            let pdf = HittablePDF::new(rec.p, important_objs);
            let mix_pdf = MixturePDF::new(&pdf, &*mat_pdf);
            let scattered = Ray::new(rec.p, mix_pdf.generate());
            let pdf_value = mix_pdf.value(&scattered.direction);
            let scattering_pdf = rec.material.scattering_pdf(ray, &rec, &scattered);

            return mat_hit_res.color
                * ray_color(
                    &scattered,
                    world,
                    important_objs,
                    depth - 1,
                    background_color,
                )
                * scattering_pdf
                / pdf_value;
        } else {
            return rec.material.emit_color(ray, &rec);
        }
    }

    return background_color;
}

pub fn generate_ray(camera: &Camera, (x, y): (i32, i32)) -> Ray {
    let pixel_center = camera.pixel00_loc
        + (x as Float * camera.pixel_delta_u)
        + (y as Float * camera.pixel_delta_v);

    let pixel_sample = pixel_center + pixel_sample_square(camera);

    let ray_origin = if camera.defocus_angle <= 0.0 {
        camera.center
    } else {
        defocus_disk_sample(camera)
    };

    let ray_direction = pixel_sample - ray_origin;

    return Ray::new(camera.center, ray_direction.normalize());
}

fn pixel_sample_square(camera: &Camera) -> Vec3 {
    let px: Float = -0.5 + random::<Float>();
    let py: Float = -0.5 + random::<Float>();

    return (px * camera.pixel_delta_u) + (py * camera.pixel_delta_v);
}
fn defocus_disk_sample(camera: &Camera) -> Vec3 {
    let p = random_vec_unit_disk();
    return camera.center + (p.x * camera.defocus_disk_u) + (p.y * camera.defocus_disk_v);
}
