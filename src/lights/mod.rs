use crate::utils::{vector::Point, rgb::RGB};


#[derive(Debug, Clone, Copy, Default)]
pub struct AmbientLight{
    pub color: RGB
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PointLight{
    pub color: RGB,
    pub position: Point,
}


#[derive(Debug, Clone, Copy)]
pub enum Light{
    Ambient(AmbientLight),
    Point(PointLight)
}


impl Light{
    pub fn point_radiance(&self, point: &Point) -> RGB{
        match self{
            Self::Ambient(ambient) => {
                todo!()
            }
            Self::Point(point) => {
                todo!()
            }

        }
    }
}
