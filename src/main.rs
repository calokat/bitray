use glam::Vec3;
use ray_tracing_weekend_rs::hittable::Hittable;
use ray_tracing_weekend_rs::sphere::Sphere;
use ray_tracing_weekend_rs::camera::Camera;
fn main() {    
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let camera = Camera::new(16.0 / 9.0, 400, 50);

    camera.render(&world);
}
