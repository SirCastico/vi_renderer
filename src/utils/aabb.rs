use crate::utils::vector::{Point};

#[derive(Debug, Default, Copy, Clone)]
pub struct AABB {
    pub max: Point,
    pub min: Point
}


impl AABB {
    pub fn update(&mut self, p: &Point){
        if p.x < self.min.x {
            self.min.x = p.x;
        } else if p.x > self.max.x {
            self.max.x = p.x;
        }
        if p.y < self.min.y {
            self.min.y = p.y;
        } else if p.y > self.max.y {
            self.max.y = p.y;
        }
        if p.z < self.min.z {
            self.min.z = p.z;
        } else if p.z > self.max.z {
            self.max.z = p.z;
        }
    }

    /*
     * I suggest you implement:
     *  bool intersect (Ray r) { }
     *
     * based on PBRT's 3rd ed. book , sec 3.1.2, pags 125..128 + 214,217,221
     *
     * or https://doi.org/10.1007/978-1-4842-7185-8_2
     *
     */
    //pub fn intersect(ray: &Ray) -> bool{}
}