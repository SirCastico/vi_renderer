use crate::utils::rgb::RGB;


pub struct MaterialData{
    pub ka: RGB,
    pub kd: RGB,
    pub ks: RGB,
    pub kt: RGB,
    pub ns: f32,
}
