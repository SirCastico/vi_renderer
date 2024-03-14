use crate::utils::vector::{Point, Vector};



#[derive(Debug, Clone, Copy, Default)]
pub struct Ray{
    pub origin: Point,
    pub direction: Vector,
}

const EPSILON: f32 = 1e-3;

impl Ray{
    fn adjust_origin(&mut self, normal: &Vector){
        let mut offset = EPSILON * *normal;
        if self.direction.dot(*normal) < 0f32 {
            offset = -1f32 * offset;
        }
        self.origin = self.origin + offset;
    }
}
