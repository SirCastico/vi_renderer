use std::ops;

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { X: x, Y: y, Z: z }
    }

    pub fn set(&mut self, v: Vector) {
        self.X = v.X;
        self.Y = v.Y;
        self.Z = v.Z;
    }

    pub fn norm(&self) -> f32 {
        return (self.X * self.X + self.Y * self.Y + self.Z * self.Z).sqrt();
    }
    
    pub fn normalize(&mut self) {
        let my_norm = self.norm();
        if my_norm > 0.0 {
            self.X /= my_norm;
            self.Y /= my_norm;
            self.Z /= my_norm;
        }
    }

    pub fn dot(&self, v2: Vector) -> f32 {
        return self.X * v2.X + self.Y * v2.Y + self.Z * v2.Z;
    }

    pub fn cross(&self, v2: Vector) -> Vector {
        Vector {
            X: self.Y * v2.Z - self.Z * v2.Y,
            Y: self.Z * v2.X - self.X * v2.Z,
            Z: self.X * v2.Y - self.Y * v2.X,
        }
    }

    pub fn abs(&self) -> Vector {
        Vector {
            X: self.X.abs(),
            Y: self.Y.abs(),
            Z: self.Z.abs(),
        }
    }

    pub fn max_dimension(&self) -> usize {
        if self.X > self.Y {
            if self.X > self.Z {
                0
            } else {
                2
            }
        } else {
            if self.Y > self.Z {
                1
            } else {
                2
            }
        }
    }

    pub fn permute(&self, x: usize, y: usize, z: usize) -> Vector {
        let xyz = [self.X, self.Y, self.Z];
        Vector {
            X: xyz[x],
            Y: xyz[y],
            Z: xyz[z],
        }
    }

    pub fn faceforward(&self, v: Vector) -> Vector {
        if self.dot(v) < 0.0 {
            -1.0 * *self
        } else {
            *self
        }
    }

    pub fn coordinate_system(&self, v2: &mut Vector, v3: &mut Vector) {
        if self.X.abs() > self.Y.abs() {
            *v2 = Vector::new(-self.Z, 0.0, self.X) / (self.X * self.X + self.Z * self.Z).sqrt();
        } else {
            *v2 = Vector::new(0.0, self.Z, -self.Y) / (self.Y * self.Y + self.Z * self.Z).sqrt();
        }
        *v3 = self.cross(*v2);
    }

    pub fn rotate(&self, rx: Vector, ry: Vector, rz: Vector) -> Vector {
        Vector {
            X: self.X * rx.X + self.Y * ry.X + self.Z * rz.X,
            Y: self.X * rx.Y + self.Y * ry.Y + self.Z * rz.Y,
            Z: self.X * rx.Z + self.Y * ry.Z + self.Z * rz.Z,
        }
    }

}

impl ops::Add<Vector> for Vector{
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            X: self.X + other.X,
            Y: self.Y + other.Y,
            Z: self.Z + other.Z,
        }
    }
}

impl ops::Sub<Vector> for Vector{
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            X: self.X - other.X,
            Y: self.Y - other.Y,
            Z: self.Z - other.Z,
        }
    }
}

impl ops::Mul<f32> for Vector{
    type Output = Vector;

    fn multf(self, scalar: f32) -> Vector {
        Vector {
            X: scalar * self.X,
            Y: scalar * self.Y,
            Z: scalar * self.Z,
        }
    }
}

impl ops::Mul<f64> for Vector{
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector {
            X: scalar * self.X,
            Y: scalar * self.Y,
            Z: scalar * self.Z,
        }
    }
}

impl ops::Mul<f64> for Vector{
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector {
            X: scalar * self.X,
            Y: scalar * self.Y,
            Z: scalar * self.Z,
        }
    }
}

impl ops::Div<f32> for Vector{
    type Output = Vector;

    fn div(self, scalar: f32) -> Vector {
        Vector {
            X: self.X / scalar,
            Y: self.Y / scalar,
            Z: self.Z / scalar,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { X: x, Y: y, Z: z }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.X = x;
        self.Y = y;
        self.Z = z;
    }

    pub fn vec2point(&self, p2: Point) -> Vector {
        Vector::new(p2.X - self.X, p2.Y - self.Y, p2.Z - self.Z)
    }

    pub fn permute(&self, x: usize, y: usize, z: usize) -> Point {
        let xyz = [self.X, self.Y, self.Z];
        Point {
            X: xyz[x],
            Y: xyz[y],
            Z: xyz[z],
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            X: self.X + other.X,
            Y: self.Y + other.Y,
            Z: self.Z + other.Z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            X: self.X - other.X,
            Y: self.Y - other.Y,
            Z: self.Z - other.Z,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, scalar: f32) -> Point {
        Point {
            X: scalar * self.X,
            Y: scalar * self.Y,
            Z: scalar * self.Z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point {
            X: scalar * self.X,
            Y: scalar * self.Y,
            Z: scalar * self.Z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, vector: Vector) -> Point {
        Point {
            X: self.X + vector.X,
            Y: self.Y + vector.Y,
            Z: self.Z + vector.Z,
        }
    }
}

