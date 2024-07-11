use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Copy, Default)]
pub struct PPMPixel {
    pub rgb: [u8; 3],
}

#[derive(Debug, Clone, Default)]
pub struct ImagePPM {
    pub data: Box<[PPMPixel]>,
    pub width: u32,
    pub height: u32,
}

impl ImagePPM {
    pub fn new(width: u32, height: u32) -> Self {
        let mut v = Vec::new();
        v.resize((width * height) as usize, PPMPixel::default());
        Self {
            data: v.into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        write!(file, "P6\n{} {}\n255\n", self.width, self.height)?;

        for pixel in self.data.iter() {
            file.write_all(&pixel.rgb)?;
        }

        Ok(())
    }
}
