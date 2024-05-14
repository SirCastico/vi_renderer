use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }

    pub fn set(&mut self, v: Vector) {
        self.x = v.x;
        self.y = v.y;
        self.z = v.z;
    }

    pub fn norm(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }
    
    pub fn normalize(&mut self) {
        let my_norm = self.norm();
        if my_norm > 0.0 {
            self.x /= my_norm;
            self.y /= my_norm;
            self.z /= my_norm;
        }
    }

    pub fn dot(&self, v2: Vector) -> f32 {
        return self.x * v2.x + self.y * v2.y + self.z * v2.z;
    }

    pub fn cross(&self, v2: Vector) -> Vector {
        Vector {
            x: self.y * v2.z - self.z * v2.y,
            y: self.z * v2.x - self.x * v2.z,
            z: self.x * v2.y - self.y * v2.x,
        }
    }

    pub fn abs(&self) -> Vector {
        Vector {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn max_dimension(&self) -> usize {
        if self.x > self.y {
            if self.x > self.z {
                0
            } else {
                2
            }
        } else {
            if self.y > self.z {
                1
            } else {
                2
            }
        }
    }

    pub fn permute(&self, x: u32, y: u32, z: u32) -> Vector {
        let xyz = [self.x, self.y, self.z];
        Vector {
            x: xyz[x as usize],
            y: xyz[y as usize],
            z: xyz[z as usize],
        }
    }

    pub fn face_forward(&self, v: Vector) -> Vector {
        if self.dot(v) < 0.0 {
            -1.0 * *self
        } else {
            *self
        }
    }

    pub fn coordinate_system(&self) -> (Vector, Vector){
        let v2: Vector;
        let v3: Vector;
        if self.x.abs() > self.y.abs() {
            v2 = Vector::new(-self.z, 0.0, self.x) / (self.x * self.x + self.z * self.z).sqrt();
        } else {
            v2 = Vector::new(0.0, self.z, -self.y) / (self.y * self.y + self.z * self.z).sqrt();
        }
        v3 = self.cross(v2);
        return (v2,v3);
    }

    pub fn rotate(&self, rx: Vector, ry: Vector, rz: Vector) -> Vector {
        Vector {
            x: self.x * rx.x + self.y * ry.x + self.z * rz.x,
            y: self.x * rx.y + self.y * ry.y + self.z * rz.y,
            z: self.x * rx.z + self.y * ry.z + self.z * rz.z,
        }
    }

}

impl ops::Add<Vector> for Vector{
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<Vector> for Vector{
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f32> for Vector{
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        Vector {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}


impl ops::Mul<Vector> for f32{
    type Output = Vector;

    fn mul(self, v: Vector) -> Vector {
        Vector {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

//impl ops::Mul<Vector> for f64{
//    type Output = Vector;
//
//    fn mul(self, v: Vector) -> Vector {
//        Vector {
//            x: (self * v.x as f64) as f32,
//            y: (self * v.y as f64) as f32,
//            z: (self * v.z as f64) as f32,
//        }
//    }
//}


impl ops::Div<f32> for Vector{
    type Output = Vector;

    fn div(self, scalar: f32) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl ops::Div<Vector> for f32{
    type Output = Vector;

    fn div(self, v: Vector) -> Vector {
        Vector {
            x: self / v.x,
            y: self / v.y,
            z: self / v.z,
        }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn vec2point(&self, p2: Point) -> Vector {
        Vector::new(p2.x - self.x, p2.y - self.y, p2.z - self.z)
    }

    pub fn permute(&self, x: usize, y: usize, z: usize) -> Point {
        let xyz = [self.x, self.y, self.z];
        Point {
            x: xyz[x],
            y: xyz[y],
            z: xyz[z],
        }
    }
}

impl Into<Vector> for Point{
    fn into(self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f32> for Point {
    type Output = Point;

    fn mul(self, scalar: f32) -> Point {
        Point {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}

impl ops::Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, p: Point) -> Point {
        Point {
            x: self * p.x,
            y: self * p.y,
            z: self * p.z,
        }
    }
}


impl ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, p: Point) -> Point {
        Point {
            x: (self * p.x as f64) as f32,
            y: (self * p.y as f64) as f32,
            z: (self * p.z as f64) as f32,
        }
    }
}


impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, vector: Vector) -> Point {
        Point {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}
