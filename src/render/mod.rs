use crate::{utils::rgb::RGB, shaders::Shader, scene::Scene, images::image_rgb::ImageRGB, camera::Camera};
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};


pub fn parallel_render<S, C>(camera: &C, scene: &Scene, shader: &S, image: &mut ImageRGB, spp: usize, jitter: bool)
where
    S: Shader + std::marker::Sync,
    C: Camera + std::marker::Sync,
{
    image.data.par_iter_mut()
        .enumerate()
        .for_each(|(i,rgb)|{
            let y = i as u32 / image.width;
            let x = i as u32 % image.width;
            let mut color = RGB::new(0.0, 0.0, 0.0);
            for _ss in 0..spp{

                
                let jitter_v = if jitter {
                    let mut rng = rand::thread_rng(); 
                    Some([rng.gen::<f32>(), rng.gen::<f32>()])
                } else {
                    None
                };
                
                let primary_ray = match camera.generate_ray(x, y, jitter_v) {
                    Some(ray) => ray,
                    None => continue, 
                };
                let tdata_opt = scene.trace(&primary_ray); 
                let this_color = shader.shade(scene, &tdata_opt); 
                color += this_color; 
            }
            color /= spp as f32;
            *rgb = color;
        });
}

pub fn standard_render<S, C>(camera: &C, scene: &Scene, shader: &S, image: &mut ImageRGB, spp: usize, jitter: bool)
where
    S: Shader + std::marker::Sync,
    C: Camera + std::marker::Sync,
{
    for y in 0..image.height { 
        for x in 0..image.width { 
            let mut color = RGB::new(0.0, 0.0, 0.0);
            for _ss in 0..spp{

                
                let jitter_v = if jitter {
                    let mut rng = rand::thread_rng(); 
                    Some([rng.gen::<f32>(), rng.gen::<f32>()])
                } else {
                    None
                };
                
                let primary_ray = match camera.generate_ray(x, y, jitter_v) {
                    Some(ray) => ray,
                    None => continue, 
                };
                let tdata_opt = scene.trace(&primary_ray); 
                let this_color = shader.shade(scene, &tdata_opt); 
                color += this_color; 
                
            }

            color /= spp as f32;
            image.set(x, y, &color); 
        } 
    } 
}

pub struct IncrementalRenderer{
    pub spp_stride: u32,
    pub spp_current: u32,
    pub spp_bound: Option<u32>,
    pub jitter: bool,
}

impl IncrementalRenderer{

    pub fn new(stride: u32, bound: Option<u32>, jitter: bool) -> Self{
        Self { spp_stride: stride, spp_current: 0, spp_bound: bound, jitter }
    }

    pub fn has_finished(&self) -> bool{
        return if let Some(bound) = self.spp_bound{
            self.spp_current>bound
        } else {
            false
        }
    }

    pub fn render<S, C>(&mut self, camera: &C, scene: &Scene, shader: &S, image: &mut ImageRGB)
        where
            S: Shader + std::marker::Sync,
            C: Camera + std::marker::Sync,
    {
        if self.has_finished() {
            return;
        }
        image.data.par_iter_mut()
            .enumerate()
            .for_each(|(i,rgb)|{
                let y = i as u32 / image.width;
                let x = i as u32 % image.width;
                let mut color = RGB::new(0.0, 0.0, 0.0);
                for _ in 0..self.spp_stride{
                    let jitter_v = if self.jitter {
                        let mut rng = rand::thread_rng();
                        Some([rng.gen::<f32>(), rng.gen::<f32>()])
                    } else {
                        None
                    };
                    
                    let primary_ray = match camera.generate_ray(x, y, jitter_v) {
                        Some(ray) => ray,
                        None => continue, 
                    };
                    let tdata_opt = scene.trace(&primary_ray); 
                    let this_color = shader.shade(scene, &tdata_opt); 
                    color += this_color; 
                }
                *rgb = (*rgb * self.spp_current as f32 + color)/(self.spp_current+self.spp_stride) as f32;
            });
        self.spp_current += self.spp_stride;
    }
}
