use crate::{utils::rgb::RGB, rays::intersection::IntersectionData, primitives::{material_data::MaterialData, Intersectable}, scene::{Scene, TraceData}, lights::Light};
use super::Shader;



#[derive(Debug, Clone, Copy, Default)]
pub struct AmbientShader{
    pub background: RGB
}


impl Shader for AmbientShader {
    fn shade(&self, scene: &Scene, isect: &Option<TraceData>, mat_data: &MaterialData) -> RGB {
        let mut color = RGB::new(0.0, 0.0, 0.0);

        // if no intersection, return background
        if isect.is_none() {
            return self.background;
        }

        // verify whether the intersected object has an ambient component
        let ka = mat_data.ka;
        if ka.is_zero() {
            return color;
        }

        // Loop over scene's light sources and process Ambient Lights
        for light in &scene.lights {
            if let Light::Ambient(ambient_light) = light {
                color += ka * ambient_light.color;
            }
        }

        color
    }
}

