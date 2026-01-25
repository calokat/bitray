use crate::Float;
use crate::Vec3;
use crate::rand::Rng;
use crate::rand_vec3::random_vec_unit_disk;
use crate::ray::Ray;
use crate::render_parameters::RenderParameters;
use glam::UVec2;
use glam::Vec2;
use glam::Vec4;
pub struct Camera {
    aspect_ratio: Float,
    image_width: i32,
    image_height: i32,
    pub center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    num_samples: i32,
    max_depth: i32,
    vertical_fov: Float,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
    pub defocus_angle: Float,
    focus_distance: Float,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, render_params: RenderParameters) -> Self {
        let aspect_ratio = render_params.image_width as Float / render_params.image_height as Float;
        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let vertical_fov = 60.0f32;

        let theta = vertical_fov.to_radians();
        let h = libm::tanf(theta / 2.0);

        let focus_distance = 1.0f32;
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height
            * (render_params.image_width as Float / render_params.image_height as Float);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / render_params.image_width as Float;
        let pixel_delta_v = viewport_v / render_params.image_height as Float;

        let center = look_from;

        let viewport_upper_left =
            center - (focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_disk_u = Vec3::ZERO;
        let defocus_disk_v = Vec3::ZERO;
        let cam = Self {
            aspect_ratio: aspect_ratio,
            image_width: render_params.image_width,
            image_height: (render_params.image_width as Float / aspect_ratio) as i32,
            num_samples: render_params.num_samples,
            max_depth: render_params.max_depth,
            vertical_fov: 60.0,
            look_from,
            look_at,
            up,
            defocus_angle: 0.0,
            focus_distance: 1.0,
            w,
            u,
            v,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        };
        cam
    }

    pub fn into_gpu_camera(&self) -> [f32; 16] {
        let mut result: [f32; 16] = [0.0; 16];

        self.pixel00_loc
            .extend(1.0)
            .write_to_slice(&mut result[0..4]);
        self.pixel_delta_u
            .extend(0.0)
            .write_to_slice(&mut result[4..8]);
        self.pixel_delta_v
            .extend(0.0)
            .write_to_slice(&mut result[8..12]);
        self.center.extend(1.0).write_to_slice(&mut result[12..16]);

        result
    }

    pub fn generate_ray(&self, screen_coords: UVec2) -> Ray {
        let pixel_center = self.pixel00_loc
            + (screen_coords.x as Float * self.pixel_delta_u)
            + (screen_coords.y as Float * self.pixel_delta_v);

        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction.normalize());
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = Rng::new();
        let px: Float = -0.5 + rng.f32();
        let py: Float = -0.5 + rng.f32();

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_vec_unit_disk();
        return self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v);
    }
}

pub struct GPUCamera {
    pub pixel00_loc: Vec4,
    pub pixel_delta_u: Vec4,
    pub pixel_delta_v: Vec4,
    pub center: Vec4,
}
