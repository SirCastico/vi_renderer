use rand::{thread_rng, Rng};

use crate::{utils::{rgb::RGB, vector::Vector}, rays::{ray::Ray}, scene::{Scene, TraceData}, lights::Light};
use super::Shader;


#[derive(Debug, Clone, Copy, Default)]
pub struct DistributedShader {
    pub background: RGB,
    pub shadow_bias: f32,
    pub reflection_depth: u16,
}

impl DistributedShader{
    fn shade_impl(&self, scene: &Scene, tdata_opt: &Option<TraceData>, depth: u16) -> RGB {
        let mut color = RGB::new(0.0, 0.0, 0.0);

        // if no intersection, return background
        if tdata_opt.is_none() {
            return self.background;
        }
        let tdata = tdata_opt.unwrap();
        if let Some(le) = tdata.mat_data.le{
            return le;
        }
        
        // specular
        if !tdata.mat_data.ks.is_zero() && depth>0{
            let gn = tdata.isect.geo_normal.face_forward(tdata.isect.wo);
            let cos = gn.dot(tdata.isect.wo);
            let ray_dir = 2.0 * cos * gn - tdata.isect.wo;

            let origin = tdata.isect.point + gn * self.shadow_bias;
            let sp_ray = Ray::new(origin, ray_dir);

            let sp_tdata_opt = scene.trace(&sp_ray);
            color += self.shade_impl(scene, &sp_tdata_opt, depth-1);
        }
        
        for light in &scene.lights {
            match light{
                Light::Ambient(ambient_light) => {
                    color += tdata.mat_data.ka * ambient_light.color;
                }
                Light::Point(point_light) => {
                    if tdata.mat_data.kd.is_zero() {
                        continue;
                    }
                    let mut ray_dir: Vector = (point_light.position - tdata.isect.point).into();
                    let light_dist = ray_dir.norm();
                    ray_dir.normalize();
    
                    let mut g_normal = tdata.isect.geo_normal.face_forward(tdata.isect.wo);
                    g_normal.normalize();
    
                    if ray_dir.dot(g_normal) < 0.0{
                        continue;
                    }
    
                    let ray_o = tdata.isect.point + g_normal * self.shadow_bias;
                    let ray: Ray = Ray::new(ray_o, ray_dir);
                    let light_tdata_opt = scene.trace(&ray); // TODO: visibility instead of trace
                    
                    if light_tdata_opt.is_none() || light_tdata_opt.unwrap().isect.depth >= light_dist {
                         color += tdata.mat_data.kd * point_light.color * 0f32.max(g_normal.dot(ray_dir));
                    }
                }
                Light::Area(area_light) => {
                    if tdata.mat_data.kd.is_zero() {continue;}
                        
                    let mut rng = thread_rng();
                    let rnd = [rng.gen(), rng.gen()];
                    let (l_int, l_point) = area_light.stochastic_radiance(&rnd);
    
                    let mut l_dir: Vector = (l_point - tdata.isect.point).into();
                    let light_dist = l_dir.norm();
                    l_dir.normalize();
    
                    let cosl = l_dir.dot(tdata.isect.geo_normal.face_forward(l_dir));
                    let cosl_la = l_dir.dot(area_light.tri.normal);
    
                    if cosl>0. && cosl_la<=0.0 {
                        let mut g_normal = tdata.isect.geo_normal.face_forward(tdata.isect.wo);
                        g_normal.normalize();
    
                        let ray_o = tdata.isect.point + g_normal * self.shadow_bias;
                        let ray: Ray = Ray::new(ray_o, l_dir);
                        let light_tdata_opt = scene.trace(&ray); // TODO: visibility instead of trace
                        
                        if light_tdata_opt.is_none() || light_tdata_opt.unwrap().isect.depth >= light_dist-self.shadow_bias
                            || light_tdata_opt.unwrap().mat_data.le.is_some() {
                             color += tdata.mat_data.kd * l_int * 0f32.max(g_normal.dot(l_dir));
                        }
                    }
                }
            }
        }

        color
    }
    
}

impl Shader for DistributedShader {
    fn shade(&self, scene: &Scene, tdata_opt: &Option<TraceData>) -> RGB {
        self.shade_impl(scene, tdata_opt, self.reflection_depth)
    }
}
