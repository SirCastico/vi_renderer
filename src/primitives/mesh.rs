use crate::utils::{vector::{Point, Vector}, aabb::AABB};

use super::Intersectable;


pub struct Face{
    pub vert_inds: [u32; 3],
    pub norm_inds: [u32; 3],
    pub geo_normal: Vector,
    pub bb: AABB,
    pub has_shading_normals: bool
}

pub struct Mesh{
    pub faces: Vec<Face>,
    pub vertices: Vec<Point>,
    pub normals: Vec<Vector>
}


impl Mesh{
    
}

impl Intersectable for Mesh{
    fn intersect() -> bool {
        todo!();
    }
}





