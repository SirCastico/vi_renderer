use crate::utils::rgb::RGB;

use super::image_ppm::ImagePPM;

#[derive(Debug, Clone, Default)]
pub struct ImageRGB {
    pub data: Box<[RGB]>,
    pub width: u32,
    pub height: u32,
}

impl ImageRGB {
    pub fn new(width: u32, height: u32) -> Self {
        let mut v = Vec::new();
        v.resize((width * height) as usize, RGB::new(0.0, 0.0, 0.0));
        Self {
            data: v.into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn set(&mut self, x: u32, y: u32, rgb: &RGB) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.data[(y * self.width + x) as usize] = *rgb;
        true
    }

    pub fn add(&mut self, x: u32, y: u32, rgb: &RGB) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.data[(y * self.width + x) as usize] += *rgb;
        true
    }

    pub fn write_to_0rgb_u32<F>(&self, out: &mut [u32], tonemapper: F)
    where
        F: Fn(f32) -> f32,
    {
        assert!(out.len() >= self.data.len());
        for (i, pixel) in self.data.iter().enumerate() {
            let r: u32 = (tonemapper(pixel.r) * 255.0).min(255.0) as u32;
            let g: u32 = (tonemapper(pixel.g) * 255.0).min(255.0) as u32;
            let b: u32 = (tonemapper(pixel.b) * 255.0).min(255.0) as u32;
            out[i] = 0u32 | r << 16 | g << 8 | b;
        }
    }
}

pub fn tonemap_reinhard(val: f32) -> f32 {
    val / (val + 1.0)
}

impl From<ImageRGB> for ImagePPM {
    fn from(value: ImageRGB) -> Self {
        let mut ppm = ImagePPM::new(value.width, value.height);
        for (i, pixel) in value.data.iter().enumerate() {
            let r = (pixel.r * 255.0).min(255.0) as u8;
            let g = (pixel.g * 255.0).min(255.0) as u8;
            let b = (pixel.b * 255.0).min(255.0) as u8;
            ppm.data[i].rgb = [r, g, b];
        }
        ppm
    }
}

impl std::ops::DivAssign<f32> for RGB {
    fn div_assign(&mut self, other: f32) {
        self.r /= other;
        self.g /= other;
        self.b /= other;
    }
}
