use std::process::exit;

use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use crate::{
    bvh::BVH,
    camera::Camera,
    color::Color,
    hittable::{Hittable, HittableList},
    image_writer,
    materials::metal::Metal,
    render_parameters::RenderParameters,
    renderers,
    sphere::Sphere,
    texture::ColorTexture2D,
    Float, Vec3,
};

fn random_vec_range(rng: &mut StdRng, min: Float, max: Float) -> Vec3 {
    Vec3 {
        x: min + (max - min) * rng.gen::<Float>(),
        y: min + (max - min) * rng.gen::<Float>(),
        z: min + (max - min) * rng.gen::<Float>(),
    }
}

fn random_vec_in_unit_sphere(rng: &mut StdRng) -> Vec3 {
    loop {
        let p = random_vec_range(rng, -1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn random_unit_vector(rng: &mut StdRng) -> Vec3 {
    random_vec_in_unit_sphere(rng).normalize()
}

pub fn spheres() {
    let mut rng = StdRng::seed_from_u64(42);

    let colors: Vec<ColorTexture2D> = (0..128)
        .map(|_| {
            let color_elements = random_vec_range(&mut rng, 0.0, 1.0);
            let color_texture = ColorTexture2D {
                color: Color::new(color_elements.x, color_elements.y, color_elements.z),
            };
            color_texture
        })
        .collect();

    let metals: Vec<Metal> = colors
        .iter()
        .map(|c| {
            let fuzz: Float = rng.gen();
            Metal::new(c, fuzz)
        })
        .collect();

    let world: Vec<Sphere> = (0..500)
        .map(|_| {
            let center_dir = random_unit_vector(&mut rng);
            let center = center_dir * 15.0 + center_dir * rng.gen::<Float>() * 50.0;
            let radius = rng.gen::<Float>() * 4.0;
            Sphere::new(
                center,
                radius,
                metals.choose(&mut rng).expect("metals cannot be empty"),
                "Sphere".to_string(),
            )
        })
        .collect();

    let world: Vec<Box<dyn Hittable>> = world
        .into_iter()
        .map(|w| Box::new(w) as Box<dyn Hittable>)
        .collect();
    let bvh = BVH::new(world);

    let render_params = RenderParameters {
        image_height: 800,
        image_width: 800,
        aspect_ratio: 1.0,
        num_samples: 300,
        max_depth: 5,
        background_color: Color::new(1.0, 1.0, 1.0),
    };

    let camera = Camera::new(
        Vec3::new(0.0, 150.0, -150.0),
        Vec3::ZERO,
        Vec3::Y,
        render_params,
    );

    let scene_render = renderers::rayon::render(&camera, &bvh, render_params);

    image_writer::write_image(
        &scene_render,
        (render_params.image_width, render_params.image_height),
        render_params.num_samples,
    )
    .expect("Image should be writable");
}
