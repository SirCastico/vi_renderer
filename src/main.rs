use std::path::Path;

use images::image_rgb::ImageRGB;
use primitives::{mesh::Mesh, Intersectable};
use scene::Scene;
use shaders::{ambient_shader::AmbientShader, Shader};

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

    let mut scene = Scene::<Mesh>::new();
    scene.load_obj_file(Path::new(""));
    // scene.add_light();

    let shader = AmbientShader::default();

    let mut image = ImageRGB::new(640, 480);

    render::standard_render(&scene, &shader, &mut image);
    
    //let ppm = image_to_ppm(image);
    //ppm.write_out();


}

