use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct RGB{
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl RGB{
    pub fn new(r:f32, g:f32, b:f32) -> Self{
        Self{
            r,g,b
        }
    }
    pub fn y(&self) -> f32{
        return self.r*0.2126 + self.g*0.7152 + self.b*0.0722;
    }

    pub fn is_zero(&self) -> bool{
        return (self.r==0.0) && (self.g==0.0) && (self.b==0.0);
    }
}

impl ops::AddAssign<RGB> for RGB{
    fn add_assign(&mut self, rhs: RGB) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::Add<RGB> for RGB{
    type Output = RGB;

    fn add(self, other: RGB) -> RGB{
        RGB{
            r: self.r+other.r,
            g: self.g+other.g,
            b: self.b+other.b,
        }
    }
}

impl ops::Mul<RGB> for RGB{
    type Output = RGB;

    fn mul(self, other: RGB) -> RGB{
        RGB{
            r: self.r*other.r,
            g: self.g*other.g,
            b: self.b*other.b,
        }
    }
}

impl ops::Mul<f32> for RGB{
    type Output = RGB;

    fn mul(self, other: f32) -> RGB{
        RGB{
            r: self.r*other,
            g: self.g*other,
            b: self.b*other,
        }
    }
}

impl ops::Div<f32> for RGB{
    type Output = RGB;

    fn div(self, other: f32) -> RGB{
        RGB{
            r: self.r/other,
            g: self.g/other,
            b: self.b/other,
        }
    }
}

impl From<[f32; 3]> for RGB{
    fn from(value: [f32; 3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}
