use bitray::camera::Camera;
use bitray::color::Color;
use bitray::hittable::Hittable;
use bitray::hittable::HittableList;
use bitray::image_writer;
use bitray::materials::diffuse_light::DiffuseLightMaterial;
use bitray::materials::lambert::Lambert;
use bitray::materials::metal::Metal;
use bitray::quad::Quad;
use bitray::render_parameters::RenderParameters;
use bitray::renderers;
use bitray::sphere::Sphere;
use bitray::texture::ColorTexture2D;
use bitray::Vec3;

fn main() {
    let grey_texture = ColorTexture2D {
        color: Color::new(1.0, 1.0, 1.0),
    };

    let green_texture = ColorTexture2D {
        color: Color::new(0.12, 0.45, 0.15),
    };

    let blue_texture = ColorTexture2D {
        color: Color::new(0.3, 0.3, 0.8),
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
        let wall_left = Quad::new(
            Vec3::new(555.0, 555.0, 0.0),
            Vec3::Y * -555.0,
            Vec3::Z * 555.0,
            &mat_green,
        );
        let wall_right = Quad::new(
            Vec3::new(0.0, 0.0, 555.0),
            -Vec3::Z * 555.0,
            Vec3::Y * 555.0,
            &mat_red,
        );

        let floor = Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Z * 555.0,
            Vec3::X * 555.0,
            &mat_lambert,
        );
        let wall_back = Quad::new(
            Vec3::new(555.0, 0.0, 555.0),
            Vec3::X * -555.0,
            Vec3::Y * 555.0,
            &mat_lambert,
        );
        let ceiling = Quad::new(
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::X * 555.0,
            Vec3::Z * 555.0,
            &mat_lambert,
        );
        let light = Quad::new(
            Vec3::new(213.0, 554.0, 227.0),
            Vec3::Z * 105.0,
            Vec3::X * 130.0,
            &mat_light,
        );

        let sphere = Sphere::new(
            Vec3::new(275.0f32, 50.0f32, 275.0f32),
            50.0f32,
            &mat_metal,
            "Red Sphere".into(),
        );

        let objects: Vec<&dyn Hittable> = vec![
            &light,
            &floor,
            &wall_left,
            &wall_right,
            &floor,
            &wall_back,
            &ceiling,
            &sphere,
        ];
        let world = HittableList::new(objects);

        let render_params = RenderParameters {
            image_width: 1024,
            image_height: 512,
            num_samples: 1500,
            background_color: Color::new(0.0, 0.0, 0.0),
            max_depth: 20,
        };

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, -30.0),
            Vec3::new(0.0, 0.0, 30.0),
            Vec3::Y,
            render_params,
        );

        let importants = &light;

        let render_fn = renderers::wgpu::render;

        let scene_render = render_fn(&camera, &world, importants, render_params);

        image_writer::write_image(
            &scene_render,
            (render_params.image_width, render_params.image_height),
            1500,
        )
        .expect("Image should be writable");
    }
}
