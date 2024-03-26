use crate::utils::vector::{Point, Vector};



#[derive(Debug, Clone, Copy, Default)]
pub struct Ray{
    pub origin: Point,
    pub direction: Vector,
    pub direction_inv: Vector,
}

//pub const EPSILON: f32 = 1e-3;
pub const EPSILON: f32 = f32::EPSILON;

impl Ray{
    pub fn new(origin: Point, mut direction: Vector) -> Self{
        direction.normalize();
        let direction_inv = 1.0/direction;
        Self {
            origin,
            direction,
            direction_inv
        }
    }
    pub fn adjust_origin(&mut self, normal: &Vector){
        let mut offset = EPSILON * *normal;
        if self.direction.dot(*normal) < 0f32 {
            offset = -1f32 * offset;
        }
        self.origin = self.origin + offset;
    }
}
