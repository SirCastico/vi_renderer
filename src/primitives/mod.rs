use crate::rays::{ray::Ray, intersection::IntersectionData};


pub mod mesh;
pub mod material_data;


pub trait Intersectable{
    fn intersect(&self, ray: Ray) -> Option<IntersectionData>;
}

