use std::path::Path;


#[derive(Debug, Clone, Copy, Default)]
pub struct PPMPixel{
    pub rgb: [u8; 3]
}

#[derive(Debug, Clone, Default)]
pub struct ImagePPM{
    pub data: Box<[PPMPixel]>,
    pub width: u32,
    pub height: u32,
}

impl ImagePPM{
    fn new(width: u32, height: u32) -> Self{
        Self {
            data: Vec::with_capacity((width*height) as usize).into(),
            width,
            height
        }
    }

    fn save(path: &Path){
        todo!();
    }
}

