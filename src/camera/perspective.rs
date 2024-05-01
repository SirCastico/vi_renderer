use crate::{rays::ray::Ray, camera::Camera, utils::{vector::{Point, Vector}, Extent2D}};


#[derive(Debug, Clone, Copy, Default)]
pub struct Perspective {
    eye: Point,
    //at: Point,
    //up: Vector,
    window_extent: Extent2D,
    fov_width: f32,
    fov_height: f32,
    c2w: [[f32; 3]; 3]
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
            //at,
            //up,
            window_extent: extent,
            fov_width,
            fov_height,
            c2w: [
                [r.x, r.y, r.z],
                [up.x, up.y, up.z],
                [f.x, f.y, f.z],
            ]
        }
    }
}

impl Camera for Perspective{
    fn generate_ray(&self, x: u32, y: u32, _cam_jitter: Option<[f32; 2]>) -> Option<Ray>{
        if x>=self.window_extent.width || y>=self.window_extent.height {
            return None;
        }

        let (jitter_x, jitter_y) = match _cam_jitter {
            Some(jitter) => (jitter[0], jitter[1]),
            None => (0.5, 0.5),
        };

        let xs = (2.0*(x as f32 + jitter_x)/self.window_extent.width as f32)-1.0;
        let ys = 2.0*((self.window_extent.height - y - 1) as f32 + jitter_y)
            /self.window_extent.height as f32 - 1.0;

        let xc = xs * (self.fov_width/2.0).tan();
        let yc = ys * (self.fov_height/2.0).tan();

        let mut dir: [f32; 3] = [0.0,0.0,0.0];
        let coords = [xc,yc,-1.0];

        for i in 0..self.c2w.len() {
            for j in 0..self.c2w[i].len() {
                dir[i] += self.c2w[i][j] * coords[j];
            }
        }
        let direction = Vector::new(dir[0], dir[1], dir[2]);

        let ray = Ray::new(self.eye, direction);

        Some(ray)
    }
    fn get_resolution(&self) -> Extent2D{
        self.window_extent
    }
}
