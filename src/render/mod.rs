use crate::{utils::rgb::RGB, shaders::Shader, scene::Scene, images::image_rgb::ImageRGB, camera::Camera};
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};


pub fn standard_render<S, C>(camera: &C, scene: &Scene, shader: &S, image: &mut ImageRGB, spp: usize)
where
    S: Shader + std::marker::Sync,
    C: Camera + std::marker::Sync,
{
    // Get resolution from the image
    let jitter = true; // Set jitter to true

    image.data.par_iter_mut()
        .enumerate()
        .for_each(|(i,rgb)|{
            let y = i as u32 / image.width;
            let x = i as u32 % image.width;
            let mut color = RGB::new(0.0, 0.0, 0.0);
            for _ss in 0..spp{

                
                let jitter_v = if jitter {
                    let mut rng = rand::thread_rng(); // Initialize the random number generator
                    Some([rng.gen::<f32>(), rng.gen::<f32>()])
                } else {
                    None
                };
                
                let primary_ray = match camera.generate_ray(x, y, jitter_v) {
                    Some(ray) => ray,
                    None => continue, 
                };
                let tdata_opt = scene.trace(&primary_ray); // Trace the primary ray through the scene to find intersections
                let this_color = shader.shade(scene, &tdata_opt); // Shade the intersection using the provided shader
                color += this_color; // Accumulate color
            }
            color /= spp as f32;
            *rgb = color;
        });

    // Main rendering loop: iterate over each pixel in the image
    //for y in 0..image.height { // Loop over rows
    //    for x in 0..image.width { // Loop over columns
    //        let mut color = RGB::new(0.0, 0.0, 0.0);
    //        for _ss in 0..spp{

    //            
    //            let jitter_v = if jitter {
    //                let mut rng = rand::thread_rng(); // Initialize the random number generator
    //                Some([rng.gen::<f32>(), rng.gen::<f32>()])
    //            } else {
    //                None
    //            };
    //            
    //            let primary_ray = match camera.generate_ray(x, y, jitter_v) {
    //                Some(ray) => ray,
    //                None => continue, 
    //            };
    //            let tdata_opt = scene.trace(&primary_ray); // Trace the primary ray through the scene to find intersections
    //            let this_color = shader.shade(scene, &tdata_opt); // Shade the intersection using the provided shader
    //            color += this_color; // Accumulate color
    //            
    //        }

    //        color /= spp as f32;
    //        // Write the resulting color to the image frame buffer
    //        image.set(x, y, &color); 
    //    } // loop over columns
    //} // loop over rows
}
