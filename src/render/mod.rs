use crate::{primitives::Intersectable, shaders::Shader, scene::Scene, images::image_rgb::ImageRGB, camera::Camera};



pub fn standard_render<I, S, C>(camera: &C,scene: &Scene<I>, shader: &S, image: &mut ImageRGB)
    where I: Intersectable,
          S: Shader,
          C: Camera
{

}
