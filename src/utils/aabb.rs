use crate::{utils::vector::Point, rays::ray::Ray};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    pub max: Point,
    pub min: Point
}

impl Default for AABB{
    fn default() -> Self {
        Self {
            max: Point { x: f32::MIN, y: f32::MIN, z: f32::MIN },
            min: Point { x: f32::MAX, y: f32::MAX, z: f32::MAX },
        }
    }
}

impl AABB {
    pub fn update(&mut self, p: &Point){
        if p.x < self.min.x {
            self.min.x = p.x;
        } 
        if p.x > self.max.x {
            self.max.x = p.x;
        }
        if p.y < self.min.y {
            self.min.y = p.y;
        } 
        if p.y > self.max.y {
            self.max.y = p.y;
        }
        if p.z < self.min.z {
            self.min.z = p.z;
        } 
        if p.z > self.max.z {
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

    // based on https://tavianator.com/2011/ray_box.html
    // relies on ieee 754 division by zero
    pub fn intersect(&self, ray: &Ray) -> bool{
        let tx1 = (self.min.x - ray.origin.x)*ray.direction_inv.x;
        let tx2 = (self.max.x - ray.origin.x)*ray.direction_inv.x;

        let mut tmin = tx1.min(tx2);
        let mut tmax = tx1.max(tx2);

        let ty1 = (self.min.y - ray.origin.y)*ray.direction_inv.y;
        let ty2 = (self.max.y - ray.origin.y)*ray.direction_inv.y;

        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));

        let tz1 = (self.min.z - ray.origin.z)*ray.direction_inv.z;
        let tz2 = (self.max.z - ray.origin.z)*ray.direction_inv.z;

        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));

        return tmax >= tmin;
    }
}

#[cfg(test)]
mod tests{
    use crate::{utils::vector::{Point, Vector}, rays::ray::Ray};

    use super::AABB;

    #[test]
    fn aabb_update(){
        let a = Point::new(1.0,1.0,1.0);
        let b = Point::new(-1.0, -1.0, -1.0);
        let mut aabb_u = AABB::default();
        aabb_u.update(&a);
        aabb_u.update(&b);

        println!("{:?}", aabb_u);

        assert!(aabb_u.max==a);
        assert!(aabb_u.min==b);
    }

    #[test]
    fn aabb_intersection(){
        let a = Point::new(1.0,1.0,1.0);
        let b = Point::new(-1.0, -1.0, -1.0);
        let aabb = AABB {
            max: a,  
            min: b,  
        };
        let mut aabb_u = AABB::default();
        aabb_u.update(&a);
        aabb_u.update(&b);

        let aabb_far = AABB {
            max: Point::new(5.0, 5.0, 5.0),  
            min: Point::new(4.0, 4.0, 4.0),  
        };
        let ray = Ray::new(Point::new(5.0,0.0,0.0), Vector::new(-1.0, 0.0, 0.0));

        assert!(aabb.intersect(&ray));
        assert!(aabb_u.intersect(&ray));
        assert!(!aabb_far.intersect(&ray));
    }
}
