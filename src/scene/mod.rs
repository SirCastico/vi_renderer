use std::path::Path;

use crate::{primitives::{Intersectable, material_data::MaterialData}, lights::Light, rays::{intersection::IntersectionData, ray::Ray}};


#[derive(Debug, Clone, Copy, Default)]
pub struct TraceData{
    pub isect: IntersectionData,
    pub mat_data: MaterialData,
}


#[derive(Debug, Clone, Default)]
pub struct Scene<T: Intersectable>{
    prims: Vec<(T,u16)>, 
    phong_brdfs: Vec<MaterialData>,
    lights: Vec<Light>
}

impl<T: Intersectable> Scene<T>{
    pub fn new() -> Self{
        todo!();
    }

    pub fn trace(&self, ray: Ray) -> Option<TraceData>{
        todo!()
    }

    pub fn load_obj_file(&mut self, path: &Path) {
        todo!();
    }

    pub fn add_light(&mut self, light: Light){
        self.lights.push(light);
    }
}
