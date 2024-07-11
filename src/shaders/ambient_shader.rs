use super::Shader;
use crate::{
    lights::Light,
    scene::{Scene, TraceData},
    utils::rgb::RGB,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct AmbientShader {
    pub background: RGB,
}

impl Shader for AmbientShader {
    fn shade(&self, scene: &Scene, tdata_opt: &Option<TraceData>) -> RGB {
        let mut color = RGB::new(0.0, 0.0, 0.0);

        // if no intersection, return background
        if tdata_opt.is_none() {
            return self.background;
        }
        let tdata = tdata_opt.unwrap();

        // verify whether the intersected object has an ambient component
        let ka = tdata.mat_data.ka;
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
