use crate::{utils::{rgb::RGB, vector::Vector}, rays::{intersection::IntersectionData, ray::Ray}, primitives::{material_data::MaterialData, Intersectable}, scene::{Scene, TraceData}, lights::Light};
use super::Shader;


#[derive(Debug, Clone, Copy, Default)]
pub struct DistributedShader {
    background: RGB,
}

impl DistributedShader {
    pub fn new(scene: Scene, background: RGB) -> Self {
        DistributedShader { background }
    }

    /* 
    pub fn shade(&self, intersected: bool, isect: IntersectionData, depth: i32) -> RGB {
        let mut color = RGB::new(0.0, 0.0, 0.0);
        
        if !intersected {
            return self.background;
        }
        
        // Intersection with a light source
        if isect.is_light(){
            return isect.le;
        }
        
        let f = isect.f; // Assuming f is of type Phong
        
        // If there is a specular component, sample it
        if !f.ks.is_zero() && depth < 4 {
            color += self.specular_reflection(isect, f, depth + 1);
        }
        
        // If there is a diffuse component, do direct lighting
        if !f.kd.is_zero() {
            color += self.direct_lighting(isect, f);
        }
        
        color
    }
    */
}