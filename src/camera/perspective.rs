use crate::{rays::Ray, camera::Camera, utils::{vector::{Point, Vector}, Extent2D}};


#[derive(Debug, Clone, Copy, Default)]
struct Perspective {
    eye: Point,
    at: Point,
    up: Vector,
    extent: Extent2D,
    fov_width: f32,
    fov_heigh: f32,
    c2w: [[f32; 3]; 3]
}

impl Camera for Perspective{
    fn generate_ray(&self, x: u32, y: u32, r: &Ray, cam_jitter: Option<f32>) -> bool{
        todo!()
    }
    fn get_resolution(&self) -> Extent2D{
        self.extent
    }
}