use crate::utils::vector::{Point, Vector};
use crate::utils::rgb::RGB;
use crate::utils::aabb::AABB;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v1: Point,
    pub v2: Point,
    pub v3: Point,
    pub normal: Vector,
    pub edge1: Vector,
    pub edge2: Vector,
    pub bb: AABB, // Using AABB as the bounding box type
}

impl Default for Triangle {
    fn default() -> Self {
        let default_point = Point::default();
        let default_vector = Vector::default();
        Self {
            v1: default_point,
            v2: default_point,
            v3: default_point,
            normal: default_vector,
            edge1: default_vector,
            edge2: default_vector,
            bb: AABB::default(),
        }
    }
}


impl Triangle {
    pub fn new(v1: Point, v2: Point, v3: Point, normal: Vector) -> Self {
        let edge1 = v1.vec2point(v2);
        let edge2 = v1.vec2point(v3);
        let bb = AABB{max:v1,min:v1};
        Self {
            v1,
            v2,
            v3,
            normal,
            edge1,
            edge2,
            bb,
        }
    }

    pub fn area(&self) -> f32 {
        // double check necessário para ver se tá certo, mais tarde
        let cross_product = self.edge1.cross(self.edge2);
        let cross_product_length = cross_product.norm();
        let area = cross_product_length * 0.5;
        area.abs()
    }
}
