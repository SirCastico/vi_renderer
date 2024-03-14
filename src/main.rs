use std::path::Path;

use camera::perspective::Perspective;
use images::image_rgb::ImageRGB;
use lights::{Light, AmbientLight};
use primitives::{mesh::Mesh, Intersectable};
use scene::Scene;
use shaders::{ambient_shader::AmbientShader, Shader};
use utils::{rgb::RGB, vector::{Point, Vector}, Extent2D};

mod utils;
mod camera;
mod rays;
mod images;
mod shaders;
mod scene;
mod lights;
mod primitives;
mod render;


fn main() {

    let height = 640;
    let width = 480;

    let eye = Point::new(0.0, 0.0, 0.0);
    let at = Point::new(0.0,0.0,1.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let fov_w = 60f32;
    let fov_h = fov_w * height as f32/width as f32;
    let fov_w_rad = fov_w*3.14/180.0;
    let fov_h_rad = fov_h*3.14/180.0;

    let camera = Perspective::new(eye, at, up, Extent2D{width, height}, fov_w_rad, fov_h_rad);
    let amb_light = Light::Ambient(AmbientLight{color: RGB{r:0.9,g:0.9,b:0.9}});
    let mut scene = Scene::<Mesh>::new();
    scene.load_obj_file(Path::new(""));
    scene.add_light(amb_light);

    let shader = AmbientShader::default();

    let mut image = ImageRGB::new(640, 480);

    render::standard_render(&camera, &scene, &shader, &mut image);
    
    //let ppm = image_to_ppm(image);
    //ppm.write_out();


}

