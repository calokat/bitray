use glam::Vec3;
use ray_tracing_weekend_rs::camera::Camera;
use ray_tracing_weekend_rs::color::Color;
use ray_tracing_weekend_rs::hittable::Hittable;
use ray_tracing_weekend_rs::materials::dielectric::Dielectric;
use ray_tracing_weekend_rs::materials::lambert::Lambert;
use ray_tracing_weekend_rs::materials::metal::Metal;
use ray_tracing_weekend_rs::mesh::Mesh;
use ray_tracing_weekend_rs::mesh::MeshOptions;
use ray_tracing_weekend_rs::sphere::Sphere;
fn main() {
    let mat_ground = Lambert::new(Color::new(0.8, 0.8, 0.0));
    let mat_red = Lambert::new(Color::new(0.7, 0.3, 0.3));
    let mat_metal = Metal::new(Color::new(0.8, 0.8, 0.9), 0.3);
    let mat_glass = Dielectric::new(1.5);
    let mesh_options = MeshOptions::from_file("sphere.obj".into());
    {
        let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &mat_ground);
        let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &mat_red);
        let sphere3 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &mat_glass);
        let sphere4 = Sphere::new(Vec3::new(1.0, -0.0, -1.0), 0.5, &mat_metal);
        let mesh = Mesh::new(&mesh_options, &mat_metal);
        let world: Vec<&dyn Hittable> = vec![&mesh, &sphere1, &sphere2, &sphere3, &sphere4];

        let camera = Camera::new(
            16.0 / 9.0,
            600,
            100,
            10,
            Vec3::new(0.0, 5.0, 10.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::Y,
        );

        camera.render(&world);
    }
}
