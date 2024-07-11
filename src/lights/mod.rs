use crate::primitives::triangle::Triangle;
use crate::{
    primitives::Intersectable,
    rays::{intersection::IntersectionData, ray::Ray},
    utils::{rgb::RGB, vector::Point},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct AmbientLight {
    pub color: RGB,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PointLight {
    pub color: RGB,
    pub position: Point,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AreaLight {
    pub intensity: RGB,
    pub power: RGB,
    pub tri: Triangle,
    pub pdf: f32,
}

impl AreaLight {
    pub fn new(power: RGB, tri: Triangle) -> Self {
        let pdf = 1.0 / tri.area();
        let intensity = power * pdf;
        Self {
            intensity,
            power,
            tri,
            pdf,
        }
    }

    pub fn stochastic_radiance(&self, r: &[f32; 2]) -> (RGB, Point) {
        let sqrt_r0 = r[0].sqrt();
        let alpha = 1.0 - sqrt_r0;
        let beta = (1.0 - r[1]) * sqrt_r0;
        let gamma = r[1] * sqrt_r0;
        let mut p = Point::default();
        p.x = alpha * self.tri.v1.x + beta * self.tri.v2.x + gamma * self.tri.v3.x;
        p.y = alpha * self.tri.v1.y + beta * self.tri.v2.y + gamma * self.tri.v3.y;
        p.z = alpha * self.tri.v1.z + beta * self.tri.v2.z + gamma * self.tri.v3.z;
        return (self.intensity / self.pdf, p);
    }
}

impl Intersectable for AreaLight {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData> {
        return self.tri.intersect(ray);
    }

    fn visibility(&self, _ray: &Ray, _depth: f32) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Light {
    Ambient(AmbientLight),
    Point(PointLight),
    Area(AreaLight),
}
