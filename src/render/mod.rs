use crate::{primitives::Intersectable, shaders::Shader, scene::Scene, images::image_rgb::ImageRGB, camera::Camera};



pub fn standard_render<S, C>(camera: &C,scene: &Scene, shader: &S, image: &mut ImageRGB)
    where S: Shader,
          C: Camera
{

}
