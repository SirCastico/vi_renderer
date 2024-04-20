use crate::{utils::{vector::{Point, Vector}, aabb::AABB}, rays::{ray::{Ray, self}, intersection::IntersectionData}};

use super::Intersectable;


#[derive(Debug, Clone, Copy, Default)]
pub struct Face{
    pub positions: [Point; 3],
    //pub normals: [Vector; 3],
}

#[derive(Debug, Clone, Default)]
pub struct Mesh{
    pub positions: Box<[Point]>,
    pub pos_inds: Box<[u32]>,
    pub normals: Box<[Vector]>,
    pub norm_inds: Box<[u32]>,
    pub face_aabbs: Box<[AABB]>,
    pub aabb: AABB,
}


impl Mesh{
    pub fn new(positions: Vec<Point>, normals: Vec<Vector>, pos_inds: Vec<u32>, norm_inds: Vec<u32>) -> Self{
        let mut mesh_aabb = AABB::default();
        for pos in positions.iter(){
            mesh_aabb.update(pos);
        }
        let mut face_aabbs: Vec<AABB> = Vec::with_capacity(pos_inds.len()/3);

        for face_inds in pos_inds.chunks_exact(3){
            let ta = &positions[face_inds[0] as usize];
            let tb = &positions[face_inds[1] as usize];
            let tc = &positions[face_inds[2] as usize];

            let mut face_aabb = AABB::default();
            face_aabb.update(ta);
            face_aabb.update(tb);
            face_aabb.update(tc);
            face_aabbs.push(face_aabb);
        }
        //println!("n_faces:{}", face_aabbs.len());
        Self {
            positions: positions.into_boxed_slice(),
            normals: normals.into_boxed_slice(),
            pos_inds: pos_inds.into_boxed_slice(),
            norm_inds: norm_inds.into_boxed_slice(),
            aabb: mesh_aabb,
            face_aabbs: face_aabbs.into_boxed_slice()
        }
    }    
}

fn triangle_intersect(ray: &Ray, face: &Face) -> Option<IntersectionData> {
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
        return Some(IntersectionData { 
            point: ipoint, 
            geo_normal: e1.cross(e2), 
            wo: -1.0*ray.direction, 
            depth: t
        });
    } else {
        return None;
    }
}

impl Intersectable for Mesh{
    fn intersect(&self, ray: &Ray) -> Option<IntersectionData> {
        let mut isect: Option<IntersectionData> = None;
        let mut min_depth = f32::MAX;

        if !self.aabb.intersect(ray) {
            return isect;
        }
        
        for (i,bb) in self.face_aabbs.iter().enumerate(){
            if bb.intersect(ray) {

                let ta = self.positions[self.pos_inds[i*3] as usize];
                let tb = self.positions[self.pos_inds[i*3+1] as usize];
                let tc = self.positions[self.pos_inds[i*3+2] as usize];

                //let na = self.normals[self.norm_inds[i*3] as usize];
                //let nb = self.normals[self.norm_inds[i*3+1] as usize];
                //let nc = self.normals[self.norm_inds[i*3+2] as usize];

                let face = Face{
                    positions: [ta,tb,tc],
                    //normals: [na,nb,nc],
                };

                if let Some(face_isect) = triangle_intersect(ray, &face) {
                    if face_isect.depth < min_depth {
                        min_depth = face_isect.depth;
                        isect = Some(face_isect);
                    }
                }
            }
        }

        isect
    }

    fn visibility(&self, ray: &Ray, depth: f32) -> bool {
        todo!()
    }
}


#[cfg(test)]
mod tests{
    use crate::{utils::vector::{Point, Vector}, rays::ray::Ray};

    use super::{Face, triangle_intersect};


    #[test]
    fn triangle_intersect_test(){
        let face = Face{
            positions: [
                Point::new(0.0, 0.0, 0.0),
                Point::new(2.0, 0.0, 0.0),
                Point::new(0.0, 2.0, 0.0),
            ]
        };
        let face_ord = Face{
            positions: [
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 2.0, 0.0),
                Point::new(2.0, 0.0, 0.0),
            ]
        };
        let ray = Ray::new(
            Point::new(1.0, 1.0, 1.0), 
            Vector::new(0.0, 0.0, -1.0),
        );

        assert!(triangle_intersect(&ray, &face).is_some());
        assert!(triangle_intersect(&ray, &face_ord).is_some());

    }
}



