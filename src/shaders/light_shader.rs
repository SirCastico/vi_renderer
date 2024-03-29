use crate::{utils::{rgb::RGB, vector::Vector}, rays::{intersection::IntersectionData, ray::Ray}, primitives::{material_data::MaterialData, Intersectable}, scene::{Scene, TraceData}, lights::Light};
use super::Shader;



#[derive(Debug, Clone, Copy, Default)]
pub struct LightShader{
    pub background: RGB,
    pub shadow_bias: f32
}


impl Shader for LightShader {
    fn shade(&self, scene: &Scene, tdata_opt: &Option<TraceData>) -> RGB {
        let mut color = RGB::new(0.0, 0.0, 0.0);

        // if no intersection, return background
        if tdata_opt.is_none() {
            return self.background;
        }
        let tdata = tdata_opt.unwrap();

        // Loop over scene's light sources and process Ambient Lights
        for light in &scene.lights {
            if let Light::Ambient(ambient_light) = light {
                color += tdata.mat_data.ka * ambient_light.color;
            }
            if let Light::Point(point_light) = light {
                let mut ray_dir: Vector = (point_light.position - tdata.isect.point).into();
                let light_dist = ray_dir.norm();
                ray_dir.normalize();

                let mut g_normal = tdata.isect.geo_normal.face_forward(tdata.isect.wo);
                g_normal.normalize();

                let ray_o = tdata.isect.point + g_normal * self.shadow_bias;
                let ray: Ray = Ray::new(ray_o, ray_dir);
                let light_tdata_opt = scene.trace(&ray);
                
                if light_tdata_opt.is_none() || light_tdata_opt.unwrap().isect.depth >= light_dist {
                     color += tdata.mat_data.kd * light.point_radiance(&tdata.isect.point) * 0f32.max(g_normal.dot(ray_dir));
                }
            }
        }

        color
    }
}

