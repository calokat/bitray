use bitray::bvh::BVH;
use bitray::camera::Camera;
use bitray::color::Color;
use bitray::hittable::Hittable;
use bitray::materials::dielectric::Dielectric;
use bitray::materials::diffuse_light::DiffuseLightMaterial;
use bitray::materials::lambert::Lambert;
use bitray::materials::metal::Metal;
use bitray::mesh::Mesh;
use bitray::mesh::MeshOptions;
use bitray::quad::Quad;
use bitray::sphere::Sphere;
use bitray::texture::ColorTexture2D;
use bitray::texture::ImageTexture2D;
use glam::Mat4;
use glam::Vec3;
fn main() {
    let grey_texture = ColorTexture2D {
        color: Color::new(1.0, 1.0, 1.0),
    };

    let green_texture = ColorTexture2D {
        color: Color::new(0.0, 1.0, 0.0),
    };

    let red_texture = ColorTexture2D {
        color: Color::new(1.0, 0.0, 0.0),
    };

    let light_texture = ColorTexture2D {
        color: Color::new(1.0, 1.0, 1.0)
    };

    let mat_green = Lambert::new(&green_texture);

    let mat_red = Lambert::new(&red_texture);

    let mat_metal = Lambert::new(&grey_texture);

    let mat_light = DiffuseLightMaterial::new(&light_texture);

    let mesh_options = MeshOptions::from_file("box.obj".into());
    {
        let floor = Mesh::new(
            &mesh_options,
            &mat_metal,
            "Box".into(),
            Mat4::from_translation(Vec3::new(0.0, -2.0, -3.0))
                * Mat4::from_scale(Vec3::new(9.0, 1.0, 9.0)),
        );

        let wall_right = Mesh::new(
            &mesh_options,
            &mat_red,
            "Box".into(),
            Mat4::from_translation(Vec3::new(9.0, 3.0, 0.0))
                * Mat4::from_scale(Vec3::new(1.0, 4.0, 9.0)),
        );

        let wall_left = Mesh::new(
            &mesh_options,
            &mat_green,
            "Box".into(),
            Mat4::from_translation(Vec3::new(-9.0, 3.0, 0.0))
                * Mat4::from_scale(Vec3::new(1.0, 4.0, 9.0)),
        );

        let wall_back = Mesh::new(
            &mesh_options,
            &mat_metal,
            "Box".into(),
            Mat4::from_translation(Vec3::new(0.0, 2.0, -9.0))
                * Mat4::from_scale(Vec3::new(9.0, 5.0, 3.0)),
        );

        let ceiling = Quad::new(Vec3::new(-5.0, 8.0, -3.0), Vec3::Z * 5.0, Vec3::X * 5.0, &mat_light);

        let tall_box = Mesh::new(
            &mesh_options,
            &mat_metal,
            "Tall box".into(),
            Mat4::from_translation(Vec3::new(-6.0, 1.0, -2.0))
                * Mat4::from_rotation_y(25.0f32.to_radians())
                * Mat4::from_scale(Vec3::new(1.0, 4.0, 1.0)),
        );

        let short_box = Mesh::new(
            &mesh_options,
            &mat_metal,
            "Tall box".into(),
            Mat4::from_translation(Vec3::new(6.0, 0.0, -1.0))
                * Mat4::from_rotation_y(-25.0f32.to_radians())
                * Mat4::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        );

        let objects: Vec<&dyn Hittable> = vec![
            &floor,
            &wall_right,
            &wall_left,
            &wall_back,
            &ceiling,
            &tall_box,
            &short_box,
        ];
        let world = BVH::new(objects);
        // let world = HittableList::new(objects);
        let camera = Camera::new(
            16.0 / 9.0,
            600,
            100,
            20,
            Vec3::new(0.0, 5.0, 30.0),
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::Y,
            Color::new(0.0, 0.0, 0.0),
        );

        camera.render(&world);
    }
}
