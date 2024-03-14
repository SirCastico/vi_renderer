use crate::{primitives::Intersectable, shaders::Shader, scene::Scene, images::image_rgb::ImageRGB};



pub fn standard_render<I, S>(scene: &Scene<I>, shader: &S, image: &mut ImageRGB)
    where I: Intersectable,
          S: Shader
{

}
