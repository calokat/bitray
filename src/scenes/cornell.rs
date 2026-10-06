use crate::bvh::BVH;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable::HittableList;
use crate::image_writer;
use crate::materials::diffuse_light::DiffuseLightMaterial;
use crate::materials::lambert::Lambert;
use crate::materials::metal::Metal;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::quad::Quad;
use crate::render_parameters::RenderParameters;
use crate::renderers;
use crate::sphere::Sphere;
use crate::texture::ColorTexture2D;
use crate::Mat4;
use crate::Vec3;

pub fn cornell() {
    let grey_texture = ColorTexture2D {
        color: Color::new(1.0, 1.0, 1.0),
    };

    let green_texture = ColorTexture2D {
        color: Color::new(0.12, 0.45, 0.15),
    };

    let blue_texture = ColorTexture2D {
        color: Color::new(0.3, 0.3, 1.0),
    };

    let red_texture = ColorTexture2D {
        color: Color::new(1.0, 0.0, 0.0),
    };

    let light_texture = ColorTexture2D {
        color: Color::new(10.0, 10.0, 10.0),
    };

    let mat_green = Lambert::new(&green_texture);

    let mat_red = Lambert::new(&red_texture);

    let mat_lambert = Lambert::new(&grey_texture);
    let mat_metal = Metal::new(&blue_texture, 0.0);

    let mat_light = DiffuseLightMaterial::new(&light_texture);

    {
        let wall_left = Box::new(Quad::new(
            Vec3::new(555.0, 555.0, 0.0),
            Vec3::Y * -555.0,
            Vec3::Z * 555.0,
            &mat_green,
        ));
        let wall_right = Box::new(Quad::new(
            Vec3::new(0.0, 0.0, 555.0),
            -Vec3::Z * 555.0,
            Vec3::Y * 555.0,
            &mat_red,
        ));

        let floor = Box::new(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Z * 555.0,
            Vec3::X * 555.0,
            &mat_lambert,
        ));
        let wall_back = Box::new(Quad::new(
            Vec3::new(555.0, 0.0, 555.0),
            Vec3::X * -555.0,
            Vec3::Y * 555.0,
            &mat_lambert,
        ));
        let ceiling = Box::new(Quad::new(
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::X * 555.0,
            Vec3::Z * 555.0,
            &mat_lambert,
        ));
        let light = Box::new(Quad::new(
            Vec3::new(213.0, 554.0, 227.0),
            Vec3::Z * 105.0,
            Vec3::X * 130.0,
            &mat_light,
        ));

        let sphere = Box::new(Sphere::new(
            Vec3::new(500.0f32, 50.0f32, 500.0f32),
            50.0f32,
            &mat_metal,
            "Red Sphere".into(),
        ));

        let mesh_opts = MeshOptions::from_file("models/demon.glb".to_string());

        let mesh_opts = mesh_opts.ok();

        let render_params = RenderParameters {
            aspect_ratio: 1.0,
            image_width: 800,
            image_height: 800,
            num_samples: 100,
            background_color: Color::new(2.0, 2.0, 2.0),
            max_depth: 20,
        };

        let camera = Camera::new(
            Vec3::new(278.0, 278.0, -800.0),
            Vec3::new(278.0, 278.0, 0.0),
            Vec3::Y,
            render_params,
        );

        let render_fn = renderers::rayon::render;

        match mesh_opts {
            Some(mo) => {
                let mesh = Box::new(Mesh::new(
                    &mo[0],
                    &mat_metal,
                    "bad_guy".to_string(),
                    Mat4::from_translation(Vec3::new(500.0f32, 50.0f32, 500.0f32)),
                ));
                let objects: Vec<Box<dyn Hittable>> =
                    vec![floor, wall_left, wall_right, wall_back, ceiling, mesh];
                let world = BVH::new(objects);
                let scene_render = render_fn(&camera, &world, render_params);

                image_writer::write_image(
                    &scene_render,
                    (render_params.image_width, render_params.image_height),
                    render_params.num_samples,
                )
                .expect("Image should be writable");
            }
            None => {
                let objects: Vec<Box<dyn Hittable>> =
                    vec![floor, wall_left, wall_right, wall_back, ceiling, sphere];
                let world = BVH::new(objects);
                let scene_render = render_fn(&camera, &world, render_params);

                image_writer::write_image(
                    &scene_render,
                    (render_params.image_width, render_params.image_height),
                    render_params.num_samples,
                )
                .expect("Image should be writable");
            }
        }
    }
}
