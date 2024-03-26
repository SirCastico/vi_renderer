use std::path::Path;

use camera::perspective::Perspective;
use images::{image_rgb::ImageRGB, image_ppm::ImagePPM};
use lights::{Light, AmbientLight};
use scene::Scene;
use utils::{rgb::RGB, vector::{Point, Vector}, Extent2D};

use crate::{lights::PointLight, shaders::{light_shader::LightShader, ambient_shader::AmbientShader}};

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
    let width = 640;

    let eye = Point::new(280.0, 375.0, -830.0);
    let at = Point::new(280.0, 265.0, 280.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let fov_w = 60f32;
    let fov_h = fov_w * width as f32/height as f32;
    let fov_w_rad = fov_w*3.14/180.0;
    let fov_h_rad = fov_h*3.14/180.0;

    let camera = Perspective::new(eye, at, up, Extent2D{width, height}, fov_w_rad, fov_h_rad);
    let mut scene = Scene::new();
    scene.load_obj_file(Path::new("./models/cornell_box.obj"));

    let amb_light = Light::Ambient(AmbientLight{color: RGB{r:0.3,g:0.3,b:0.3}});

    let point_light = Light::Point(
        PointLight{
            color:RGB{r:0.9,g:0.9,b:0.9},
            position:Point::new(273.0, 495.0, 279.5)
        });

    scene.add_light(amb_light);
    scene.add_light(point_light);

    let shader = LightShader{background: RGB { r: 0.05, g: 0.05, b: 0.55 }, shadow_bias: 0.001};

    let mut image = ImageRGB::new(640, 480);

    render::standard_render(&camera, &scene, &shader, &mut image);
    
    let ppm: ImagePPM = image.into();
    ppm.save(Path::new("./out.ppm")).expect("failed to output ppm");

    println!("outputed ppm");

}

