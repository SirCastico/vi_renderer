use crate::{rays::Ray, utils::Extent2D};

pub mod perspective;

pub trait Camera {
    fn generate_ray(&self, x: u32, y: u32, r: &Ray, cam_jitter: Option<f32>) -> bool;
    fn get_resolution(&self) -> Extent2D; 
}