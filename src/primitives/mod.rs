use crate::rays::{ray::Ray, intersection::IntersectionData};


pub mod mesh;
pub mod material_data;
pub mod triangle;


pub trait Intersectable{
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData>;

    fn visibility(&self, ray: &Ray, depth: f32) -> bool;
}

