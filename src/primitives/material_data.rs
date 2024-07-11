use crate::utils::rgb::RGB;

#[derive(Debug, Clone, Copy, Default)]
pub struct MaterialData {
    pub ka: RGB, // ambient
    pub kd: RGB, // diffuse
    pub ks: RGB, // specular
    pub kt: RGB,
    pub le: Option<RGB>,
    pub ns: f32,
}
