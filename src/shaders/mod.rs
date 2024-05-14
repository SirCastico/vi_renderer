use crate::{utils::rgb::RGB, primitives::{material_data::MaterialData, Intersectable}, scene::{Scene, TraceData}};

pub mod ambient_shader;
pub mod whitted_shader;
pub mod distributed_shader;
pub mod path_tracer_shader;


pub trait Shader{
    fn shade(&self, scene: &Scene, tdata_opt: &Option<TraceData>) -> RGB;
}
