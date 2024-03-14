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

    pub fn trace(&self, ray: &Ray) -> Option<TraceData>{
        if self.prims.len() == 0 {
            return None;
        }
        let mut curr_trace_opt: Option<TraceData> = None;
        for (prim, ind) in self.prims.iter() {
            if let Some(isect) = prim.intersect(ray){
                if let Some(curr_trace) = curr_trace_opt {

                } else {
                    curr_trace_opt = Some(TraceData{
                        isect,
                        mat_data: self.phong_brdfs[*ind as usize],
                    });
                }
            } else {

            }
        }
        todo!()
    }

    pub fn load_obj_file(&mut self, path: &Path) {
        todo!();
    }

    pub fn add_light(&mut self, light: Light){
        self.lights.push(light);
    }
}
