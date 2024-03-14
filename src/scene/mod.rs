use std::path::Path;

use crate::{primitives::{Intersectable, material_data::MaterialData}};



pub struct Scene<T: Intersectable>{
    prims: Vec<(T,u16)>, 
    phong_brdfs: Vec<MaterialData>
    // lights
}

impl<T: Intersectable> Scene<T>{
    pub fn new() -> Self{
        todo!();
    }

    pub fn load_obj_file(&mut self, path: &Path) {
        todo!();
    }

    pub fn add_light(&mut self){
        todo!();
    }
}
