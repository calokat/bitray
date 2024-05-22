use bitray::bvh::BVH;
use bitray::camera::Camera;
use bitray::color::Color;
use bitray::hittable::Hittable;
use bitray::materials::dielectric::Dielectric;
use bitray::materials::lambert::Lambert;
use bitray::materials::metal::Metal;
use bitray::mesh::Mesh;
use bitray::mesh::MeshOptions;
use bitray::sphere::Sphere;
use glam::Vec3;
fn main() {
    let mat_ground = Lambert::new(Color::new(0.8, 0.8, 0.0));
    let mat_red = Lambert::new(Color::new(0.7, 0.3, 0.3));
    let mat_metal = Metal::new(Color::new(1.0, 1.0, 1.0), 0.0);
    let mat_metal_2 = Metal::new(Color::new(0.8, 0.8, 0.9), 0.3);
    let mat_glass = Dielectric::new(1.5);
    let mesh_options = MeshOptions::from_file("box.obj".into());
    {
        let green_sphere = Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            &mat_ground,
            "Green Sphere".into(),
        );
        let metal_sphere = Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            &mat_metal,
            "Metal Sphere".into(),
        );
        let glass_sphere = Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            &mat_glass,
            "Glass Sphere".into(),
        );
        let red_sphere = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            &mat_red,
            "Red Sphere".into(),
        );
        let mesh = Mesh::new(&mesh_options, &mat_metal_2, "Box".into());
        let objects: Vec<&dyn Hittable> = vec![
            &red_sphere,
            &glass_sphere,
            &metal_sphere,
            &mesh,
            &green_sphere,
        ];
        let world = BVH::new(objects);
        // let world = HittableList::new(objects);
        let camera = Camera::new(
            16.0 / 9.0,
            600,
            100,
            20,
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        );

        camera.render(&world);
    }
}
