use crate::{utils::rgb::RGB, primitives::{material_data::MaterialData, Intersectable}, scene::{Scene, TraceData}};

pub mod ambient_shader;


pub trait Shader{
    fn shade(&self, scene: &Scene, isect: &Option<TraceData>, mat_data: &MaterialData) -> RGB;
}
