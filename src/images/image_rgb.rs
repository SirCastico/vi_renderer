use crate::utils::rgb::RGB;

#[derive(Debug, Clone, Default)]
pub struct ImageRGB{
    pub data: Box<[RGB]>,
    pub width: u32,
    pub height: u32
}

impl ImageRGB{
    pub fn new(width: u32, height: u32) -> Self{
        Self {
            data: Vec::with_capacity((width*height) as usize).into(),
            width,
            height
        }
    }

    pub fn set(&mut self, x: u32, y: u32, rgb: &RGB) -> bool{
        if x>=self.width || y>=self.height {
            return false;
        }
        self.data[(y*self.width+x) as usize] = *rgb;
        true
    }

    pub fn add(&mut self, x: u32, y: u32, rgb: &RGB) -> bool{
        if x>=self.width || y>=self.height {
            return false;
        }
        self.data[(y*self.width+x) as usize] += *rgb;
        true
    }
}
