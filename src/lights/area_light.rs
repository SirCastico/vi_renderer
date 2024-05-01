use crate::utils::{vector::Point, rgb::RGB, vector::Vector};
use crate::primitives::triangle::Triangle;


#[derive(Debug, Clone, Copy, Default)]
pub struct AreaLight{
    pub intensity: RGB,
    pub power: RGB,
    pub gem: Triangle,
    pub pdf: f32,
}

impl AreaLight {
    pub fn new(power: RGB, v1: Point, v2: Point, v3: Point, normal: Vector) -> Self {
        let gem = Triangle::new(v1, v2, v3, normal);
        let pdf = 1.0 / gem.area();
        let intensity = power * pdf;
        Self {
            intensity,
            power,
            gem,
            pdf,
        }
    } 
    
    pub fn sample_l(&self, r: &[f32; 2], p: &mut Point, pdf: &mut f32) -> RGB {
        let sqrt_r0 = r[0].sqrt();
        let alpha = 1.0 - sqrt_r0;
        let beta = (1.0 - r[1]) * sqrt_r0;
        let gamma = r[1] * sqrt_r0;
        p.x = alpha * self.gem.v1.x + beta * self.gem.v2.x + gamma * self.gem.v3.x;
        p.y = alpha * self.gem.v1.y + beta * self.gem.v2.y + gamma * self.gem.v3.y;
        p.z = alpha * self.gem.v1.z + beta * self.gem.v2.z + gamma * self.gem.v3.z;
        *pdf = self.pdf;
        return self.intensity
    }
}