use crate::{utils::rgb::RGB, rays::intersection::IntersectionData, primitives::{material_data::MaterialData, Intersectable}, scene::{Scene, TraceData}};

use super::Shader;



#[derive(Debug, Clone, Copy, Default)]
pub struct AmbientShader{
    pub background: RGB
}


impl Shader for AmbientShader{
    fn shade(&self, scene: &Scene, isect: &Option<TraceData>, mat_data: &MaterialData) -> RGB {
        todo!()
    }
}

