use crate::{rays::ray::Ray, camera::Camera, utils::{vector::{Point, Vector}, Extent2D}};


#[derive(Debug, Clone, Copy, Default)]
pub struct Perspective {
    pub eye: Point,
    pub at: Point,
    pub up: Vector,
    pub extent: Extent2D,
    pub fov_width: f32,
    pub fov_height: f32,
    pub c2w: [[f32; 3]; 3]
}

impl Perspective{
    pub fn new(eye: Point, at: Point, up: Vector, extent: Extent2D, fov_width: f32, fov_height: f32) -> Self{
        Self{
            eye,
            at,
            up,
            extent,
            fov_width,
            fov_height,
            ..Default::default()
        }
    }
}

impl Camera for Perspective{
    fn generate_ray(&self, x: u32, y: u32, r: &Ray, cam_jitter: Option<f32>) -> bool{
        todo!()
    }
    fn get_resolution(&self) -> Extent2D{
        self.extent
    }
}
