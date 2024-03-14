use crate::utils::vector::{Point, Vector};


#[derive(Debug, Clone, Copy, Default)]
pub struct IntersectionData{
    pub point: Point,
    pub geo_normal: Vector,
    pub wo: Vector,
}
