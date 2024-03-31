use glam::Vec3;
use ray_tracing_weekend_rs::hittable::Hittable;
use ray_tracing_weekend_rs::mesh::Mesh;
use ray_tracing_weekend_rs::sphere::Sphere;
use ray_tracing_weekend_rs::triangle::Triangle;
use ray_tracing_weekend_rs::camera::Camera;
use ray_tracing_weekend_rs::materials::lambert::Lambert;
use ray_tracing_weekend_rs::materials::metal::Metal;
use ray_tracing_weekend_rs::materials::dielectric::Dielectric;
use ray_tracing_weekend_rs::color::Color;
use wavefront_obj::obj;
use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let mat_ground = Lambert::new(Color::new(0.8, 0.8, 0.0));
    let mat_red = Lambert::new(Color::new(0.7, 0.3, 0.3));
    let mat_metal = Metal::new(Color::new(0.8, 0.6, 0.2), 0.3);
    let mat_glass = Dielectric::new(1.5);
    let mut open_opts = OpenOptions::new().read(true).open("sphere.obj").expect("sphere.obj should exist");
    {
        // let mut obj_contents: String = String::new();
        // open_opts.read_to_string(&mut obj_contents);
        // let cube = match obj::parse(obj_contents) {
        //     Ok(c) => {
        //         Mesh::new(c, Vec3::new(0.0, 1.0, -5.0), &mat_red)
        //     },
        //     Err(e) => {
        //         panic!("Cannot load mesh: {e}");
        //     }
        // };

        let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &mat_ground);
        let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &mat_red);
        let sphere3 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &mat_glass);
        let sphere4 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &mat_metal);
        
        let tri = Triangle::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::new(0.0, 9.0, 0.0), Vec3::new(5.0, 0.0, 0.0), &mat_ground);
        
        let world: Vec<&dyn Hittable> = vec![
            &sphere1,
            &sphere2,
            &sphere3,
            &sphere4,
            &tri,
        ];


        let camera = Camera::new(16.0 / 9.0, 800, 100, 50, Vec3::new(0.0, 4.0, 10.0), Vec3::new(1.5, -1.0, -1.0), Vec3::X);

        camera.render(&world);
    }
}
