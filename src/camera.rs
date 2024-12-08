use crate::render_parameters::RenderParameters;
use crate::Float;
use crate::Vec3;
#[derive(Default)]
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
        let mut cam = Self::default();
        cam.aspect_ratio = render_params.image_width as Float / render_params.image_height as Float;
        cam.image_width = render_params.image_width;
        cam.num_samples = render_params.num_samples;
        cam.max_depth = render_params.max_depth;
        cam.vertical_fov = 60.0;
        cam.look_from = look_from;
        cam.look_at = look_at;
        cam.up = up;
        cam.defocus_angle = 0.0;
        cam.focus_distance = 1.0;
        cam.initialize();
        cam
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as Float / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.look_from;

        // Determine viewport dimensions.
        let theta = self.vertical_fov.to_radians();
        let h = Float::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width =
            viewport_height * (self.image_width as Float / self.image_height as Float);

        self.w = (self.look_from - self.look_at).normalize();
        self.u = self.up.cross(self.w).normalize();
        self.v = self.w.cross(self.u);
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as Float;
        self.pixel_delta_v = viewport_v / self.image_height as Float;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_distance * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius =
            self.focus_distance * Float::tan((self.defocus_angle / 2.0).to_radians());
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        println!("aspect ratio {:?}", self.aspect_ratio);
    }
}
