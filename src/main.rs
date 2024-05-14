use std::path::Path;

use camera::perspective::Perspective;
use images::{image_rgb::ImageRGB, image_ppm::ImagePPM};
use lights::{Light, AmbientLight};
use scene::Scene;
use utils::{rgb::RGB, vector::{Point, Vector}, Extent2D};

use crate::{lights::{AreaLight, PointLight}, primitives::triangle::Triangle, shaders::{ambient_shader::AmbientShader, whitted_shader::WhittedShader}};

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

    let height = 800;
    let width = 800;

    let eye = Point::new(280.0, 375.0, -800.0);
    let at = Point::new(280.0, 300.0, 280.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let fov_w = 60f32;
    let fov_h = fov_w * width as f32/height as f32;
    let fov_w_rad = fov_w*3.14/180.0;
    let fov_h_rad = fov_h*3.14/180.0;

    let camera = Perspective::new(eye, at, up, Extent2D{width, height}, fov_w_rad, fov_h_rad);
    let mut scene = Scene::new();
    scene.load_obj_file(Path::new("./models/cornell_box_VI.obj"));

    let amb_light = Light::Ambient(AmbientLight{color: RGB{r:0.3,g:0.3,b:0.3}});

    let point_light = Light::Point(
        PointLight{
            color:RGB{r:0.9,g:0.9,b:0.9},
            position:Point::new(273.0, 495.0, 279.5)
        });

    let a_light = Light::Area(
        AreaLight::new(
            RGB::new(0.8, 0.8, 0.8), 
            Triangle::new(
                Point::new(253.0, 525.0, 279.0), 
                Point::new(303.0, 525.0, 279.0), 
                Point::new(273.0, 525.0, 330.0), 
                Vector::new(0.0, -1.0, 0.0),
            )
        )
    );
    scene.add_light(amb_light);
    //scene.add_light(point_light);
    scene.add_light(a_light);

    let shader = WhittedShader{background: RGB { r: 0.05, g: 0.05, b: 0.55 }, shadow_bias: 0.005f32, reflection_depth: 3};
    //let shader = AmbientShader{background: RGB { r: 0.05, g: 0.05, b: 0.55 }};

    let mut image = ImageRGB::new(height, width);

    render::standard_render(&camera, &scene, &shader, &mut image, 16);
    
    let ppm: ImagePPM = image.into();
    ppm.save(Path::new("./out.ppm")).expect("failed to output ppm");

    println!("outputed ppm");

}

