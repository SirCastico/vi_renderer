use crate::{utils::rgb::RGB,primitives::Intersectable, shaders::Shader, scene::Scene, images::image_rgb::ImageRGB, camera::Camera};



pub fn standard_render<S, C>(camera: &C, scene: &Scene, shader: &S, image: &mut ImageRGB)
where
    S: Shader,
    C: Camera,
{
    // Get resolution from the image
    let width = image.width;
    let height = image.height;

    // Main rendering loop: iterate over each pixel in the image
    for y in 0..height { // Loop over rows
        for x in 0..width { // Loop over columns
            // Generate primary ray from the camera
            let cam_jitter = None;
            let primary_ray = match camera.generate_ray(x, y, cam_jitter) {
                Some(ray) => ray,
                None => continue, 
            };

            // Trace the primary ray through the scene to find intersections
            let intersection = scene.trace(&primary_ray);

            let intr_op = intersection;

            // Shade the intersection using the provided shader, with depth=0
            let color = if let Some(intersection) = &intersection {
                shader.shade(scene, &intr_op, &intersection.mat_data)
            } else {
                RGB::new(0.0, 0.0, 0.0) //O que é suposto acontecer nesta situação exatamente?
            };

            // Write the resulting color to the image frame buffer
            image.set(x, y, &color); 
        } // loop over columns
    } // loop over rows
}