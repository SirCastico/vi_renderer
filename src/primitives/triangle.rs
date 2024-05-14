use crate::rays::intersection::IntersectionData;
use crate::rays::ray::{self, Ray};
use crate::utils::vector::{Point, Vector};
use crate::utils::aabb::AABB;

use super::Intersectable;

#[derive(Debug, Clone, Copy, Default)]
pub struct Triangle {
    pub v1: Point,
    pub v2: Point,
    pub v3: Point,
    pub normal: Vector,
    pub bb: AABB, // Using AABB as the bounding box type
}


impl Triangle {
    pub fn new(v1: Point, v2: Point, v3: Point, normal: Vector) -> Self {
        let mut bb = AABB::default();
        bb.update(&v1);
        bb.update(&v2);
        bb.update(&v3);
        Self {
            v1,
            v2,
            v3,
            normal,
            bb,
        }
    }

    pub fn area(&self) -> f32 {
        // double check necessário para ver se tá certo, mais tarde
        let edge1: Vector = (self.v2 - self.v1).into();
        let edge2: Vector = (self.v3 - self.v1).into();
        let cross_product = edge1.cross(edge2);
        let cross_product_length = cross_product.norm();
        let area = cross_product_length * 0.5;
        area.abs()
    }
}

impl Intersectable for Triangle{
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData> {
        if !self.bb.intersect(ray){
            return Option::None;
        }
        let face = Face { 
            positions: [self.v1,self.v2,self.v3] 
        };
        return triangle_intersect(ray, &face)
    }

    fn visibility(&self, ray: &Ray, depth: f32) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Face{
    pub positions: [Point; 3],
    //pub normals: [Vector; 3],
}

pub fn triangle_intersect(ray: &Ray, face: &Face) -> Option<IntersectionData> {
    // Based on https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm

    let e1: Vector = (face.positions[1] - face.positions[0]).into();
    let e2: Vector = (face.positions[2] - face.positions[0]).into();

    let ray_cross_e2 = ray.direction.cross(e2);
    let det = e1.dot(ray_cross_e2);

    if det > -ray::EPSILON && det < ray::EPSILON{
        return None; // ray is parallel
    }

    let inv_det = 1.0 / det;
    let s: Vector = (ray.origin - face.positions[0]).into();
    let u = inv_det * s.dot(ray_cross_e2);

    if u<0.0 || u>1.0{
        return None;
    }

    let s_cross_e1 = s.cross(e1);
    let v = inv_det * ray.direction.dot(s_cross_e1);
    if v<0.0 || u+v>1.0 {
        return None;
    }

    let t = inv_det * e2.dot(s_cross_e1);

    if t > ray::EPSILON{
        let ipoint = ray.origin + ray.direction * t;
        let mut gn = e1.cross(e2);
        gn.normalize();
        return Some(IntersectionData { 
            point: ipoint, 
            geo_normal: gn, 
            wo: -1.0*ray.direction, 
            depth: t,
            ..Default::default()
        });
    } else {
        return None;
    }
}