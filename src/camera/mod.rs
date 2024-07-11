use crate::{rays::ray::Ray, utils::Extent2D};

pub mod perspective;

pub trait Camera {
    fn generate_ray(&self, x: u32, y: u32, cam_jitter: Option<[f32; 2]>) -> Option<Ray>;
    fn get_resolution(&self) -> Extent2D;
}
