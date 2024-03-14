use crate::{utils::rgb::RGB, rays::intersection::IntersectionData};

use super::Shader;



#[derive(Debug, Clone, Copy, Default)]
pub struct AmbientShader{
    pub background: RGB
}


impl Shader for AmbientShader{
    fn shade(isect: Option<IntersectionData>) -> RGB {
        RGB::default()
    }
}
