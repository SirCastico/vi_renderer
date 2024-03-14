use crate::{utils::rgb::RGB, rays::intersection::IntersectionData};

pub mod ambient_shader;


pub trait Shader{
    fn shade(isect: Option<IntersectionData>) -> RGB;
}
