use crate::rays::{intersection::IntersectionData, ray::Ray};

pub mod material_data;
pub mod mesh;
pub mod triangle;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData>;

    fn visibility(&self, ray: &Ray, depth: f32) -> bool;
}
