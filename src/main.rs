use std::sync::atomic::{AtomicU64, Ordering};
use std::{path::Path, thread, time::Instant};

use camera::{perspective::Perspective, Camera};
use images::image_rgb::{self, ImageRGB};
use lights::Light;
use minifb::{Key, Window, WindowOptions};
use render::standard_render;
use scene::Scene;
use shaders::Shader;
use utils::{
    rgb::RGB,
    vector::{Point, Vector},
    Extent2D,
};

use crate::{
    lights::AreaLight,
    primitives::triangle::Triangle,
    render::IncrementalRenderer,
    shaders::path_tracer_shader::PathTracerShader,
    swapchain::DoubleBufferSwapChain,
};

mod camera;
mod images;
mod lights;
mod primitives;
mod rays;
mod render;
mod scene;
mod shaders;
mod swapchain;
mod utils;

fn main() {
    let height = 800;
    let width = 800;

    let eye = Point::new(280.0, 375.0, -800.0);
    let at = Point::new(280.0, 300.0, 280.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    let fov_w = 60f32;
    let fov_h = fov_w * width as f32 / height as f32;
    let fov_w_rad = fov_w * 3.14 / 180.0;
    let fov_h_rad = fov_h * 3.14 / 180.0;

    let camera = Perspective::new(
        eye,
        at,
        up,
        Extent2D { width, height },
        fov_w_rad,
        fov_h_rad,
    );
    let mut scene = Scene::new();
    scene.load_obj_file(Path::new("./models/cornell_box_VI.obj"));

    let b_light1 = Light::Area(AreaLight::new(
        RGB::new(1.0, 1.0, 1.0),
        Triangle::new(
            Point::new(343.0, 548.0, 227.0),
            Point::new(343.0, 548.0, 332.0),
            Point::new(213.0, 548.0, 332.0),
            Vector::new(0.0, -1.0, 0.0),
        ),
    ));
    let b_light2 = Light::Area(AreaLight::new(
        RGB::new(1.0, 1.0, 1.0),
        Triangle::new(
            Point::new(213.0, 548.0, 332.0),
            Point::new(213.0, 548.0, 227.0),
            Point::new(343.0, 548.0, 227.0),
            Vector::new(0.0, -1.0, 0.0),
        ),
    ));

    scene.add_light(b_light1);
    scene.add_light(b_light2);

    let shader = PathTracerShader {
        background: RGB {
            r: 0.05,
            g: 0.05,
            b: 0.55,
        },
        collision_bias: 0.001f32,
        reflection_depth: 2,
        continue_prob: 0.5,
    };

    //let shader = DistributedShader{
    //    background: RGB { r: 0.05, g: 0.05, b: 0.55 },
    //    shadow_bias: 0.001f32,
    //    reflection_depth: 2,
    //};

    let renderer = IncrementalRenderer::new(1, Some(64), true);

    let mut window = Window::new(
        "yep",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    let inst = Instant::now();

    render_loop_with_swapchain(camera, scene, shader, window, width, height, renderer);
    //render_loop(camera, scene, shader, window, width, height, renderer);
    //render_loop_sequential(camera, scene, shader, window, width, height, 128, true);

    println!(
        "total time: {}.{} seconds",
        inst.elapsed().as_secs(),
        inst.elapsed().subsec_millis()
    );
}

fn render_loop_sequential<C, S>(
    camera: C,
    scene: Scene,
    shader: S,
    mut window: Window,
    width: u32,
    height: u32,
    spp: usize,
    jitter: bool,
) where
    C: Camera + std::marker::Sync,
    S: Shader + std::marker::Sync,
{
    let mut image = ImageRGB::new(height, width);

    let mut buf: Vec<u32> = std::iter::repeat(0)
        .take((width * height) as usize)
        .collect();

    standard_render(&camera, &scene, &shader, &mut image, spp, jitter);
    image.write_to_0rgb_u32(&mut buf, image_rgb::tonemap_reinhard);

    window
        .update_with_buffer(&buf, width as usize, height as usize)
        .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}

fn render_loop<C, S>(
    camera: C,
    scene: Scene,
    shader: S,
    mut window: Window,
    width: u32,
    height: u32,
    mut renderer: IncrementalRenderer,
) where
    C: Camera + std::marker::Sync,
    S: Shader + std::marker::Sync,
{
    let mut image = ImageRGB::new(height, width);

    let mut buf: Vec<u32> = std::iter::repeat(0)
        .take((width * height) as usize)
        .collect();
    let mut frame_number: u64 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let inst = Instant::now();

        if !renderer.has_finished() {
            renderer.render(&camera, &scene, &shader, &mut image);
            image.write_to_0rgb_u32(&mut buf, image_rgb::tonemap_reinhard);
            frame_number += 1;

            let upd_inst = Instant::now();
            window
                .update_with_buffer(&buf, width as usize, height as usize)
                .unwrap();

            println!("upd elapsed: {} micros", upd_inst.elapsed().as_micros());
        } else {
            window.update();
        }

        println!(
            "frame: {} | elapsed: {} ms",
            frame_number,
            inst.elapsed().as_millis()
        );
    }
}

fn render_loop_with_swapchain<C, S>(
    camera: C,
    scene: Scene,
    shader: S,
    mut window: Window,
    width: u32,
    height: u32,
    mut renderer: IncrementalRenderer,
) where
    C: Camera + std::marker::Sync,
    S: Shader + std::marker::Sync,
{
    let swpchain = DoubleBufferSwapChain::new(width, height);

    let frame_number = AtomicU64::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let mut image = ImageRGB::new(height, width);

            while !renderer.has_finished() {
                let inst = Instant::now();
                renderer.render(&camera, &scene, &shader, &mut image);

                let is_open = swpchain.update_back(|b| {
                    image.write_to_0rgb_u32(b, image_rgb::tonemap_reinhard);
                });

                if !is_open {
                    println!("[renderer] closed");
                    break;
                }

                frame_number.fetch_add(1, Ordering::Relaxed);
                println!("([renderer] frame-gen: {} ms) ", inst.elapsed().as_millis());
            }
            swpchain.close();
            println!("[renderer] closing");
        });

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let inst = Instant::now();
            let is_open = swpchain.wait_use_front(|buffer| {
                //let upd_inst = Instant::now();
                window
                    .update_with_buffer(buffer, width as usize, height as usize)
                    .unwrap();
                //print!("[event loop] upd: {} micros | ", upd_inst.elapsed().as_micros());
            });
            if !is_open {
                println!("[event loop] closed");
                window.update();
            }
            println!(
                "[event loop] frame: {} | elapsed: {} ms",
                frame_number.load(Ordering::Relaxed),
                inst.elapsed().as_millis()
            );
        }
        swpchain.close();
        println!("[event loop] closing");
    });
}
