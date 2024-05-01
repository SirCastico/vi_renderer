use crate::utils::vector::{Point, Vector};

#[derive(Debug, Clone, Copy, Default)]
pub struct IntersectionData {
    pub point: Point,
    pub geo_normal: Vector,
    pub wo: Vector,
    pub depth: f32,
}

impl IntersectionData {
    //Não sei como é suposto ser esta função
    pub fn is_light(&self) -> bool {
        self.point != Point::default()
    }
}
