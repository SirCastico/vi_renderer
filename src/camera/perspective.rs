use crate::{rays::ray::Ray, camera::Camera, utils::{vector::{Point, Vector}, Extent2D}};


#[derive(Debug, Clone, Copy, Default)]
pub struct Perspective {
    eye: Point,
    at: Point,
    up: Vector,
    window_extent: Extent2D,
    fov_width: f32,
    fov_height: f32,
    c2w: [[f32; 3]; 4]
}

impl Perspective{
    pub fn new(eye: Point, at: Point, up: Vector, extent: Extent2D, fov_width: f32, fov_height: f32) -> Self{
        let mut f: Vector = (eye-at).into();
        f.normalize();

        let mut r = up.cross(f);
        r.normalize();

        let mut up = f.cross(r);
        up.normalize();

        Self{
            eye,
            at,
            up,
            window_extent: extent,
            fov_width,
            fov_height,
            c2w: [
                [r.x, r.y, r.z],
                [up.x, up.y, up.z],
                [f.x, f.y, f.z],
                [eye.x,eye.y,eye.z],
            ]
        }
    }
}

impl Camera for Perspective{
    fn generate_ray(&self, x: u32, y: u32, cam_jitter: Option<f32>) -> Option<Ray>{
        if x>=self.window_extent.width || y>=self.window_extent.height {
            return None;
        }

        let aspect_ratio = self.window_extent.width as f32 / self.window_extent.height as f32;
        let xs = (2.0*(x as f32 + 0.5)/self.window_extent.width as f32)-1.0;
        let ys = 1.0 - 2.0*(y as f32 + 0.5)/self.window_extent.height as f32;

        let xc = xs * aspect_ratio * (self.fov_width/2.0).tan();
        let yc = ys * (self.fov_height/2.0).tan();

        let mut ray = Ray::default();
        ray.origin = self.eye;
        let mut dir: [f32; 4] = [0.0,0.0,0.0,0.0];
        let coords = [xc,yc,-1.0];

        for i in 0..self.c2w.len() {
            for j in 0..self.c2w[i].len() {
                dir[i] += self.c2w[i][j] * coords[j];
            }
        }
        ray.direction = Vector::new(dir[0]/dir[3], dir[1]/dir[3], dir[2]/dir[3]);

        Some(ray)
    }
    fn get_resolution(&self) -> Extent2D{
        self.window_extent
    }
}
