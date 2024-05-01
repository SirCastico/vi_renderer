use crate::{utils::{rgb::RGB, vector::Vector}, rays::{intersection::IntersectionData, ray::Ray}, primitives::{material_data::MaterialData, Intersectable}, scene::{Scene, TraceData}, lights::Light};
use super::Shader;



#[derive(Debug, Clone, Copy, Default)]
pub struct LightShader{
    pub background: RGB,
    pub shadow_bias: f32,
    pub reflection_depth: u16
}

impl LightShader{
    fn shade_impl(&self, scene: &Scene, tdata_opt: &Option<TraceData>, depth: u16) -> RGB {
        let mut color = RGB::new(0.0, 0.0, 0.0);

        // if no intersection, return background
        if tdata_opt.is_none() {
            return self.background;
        }
        let tdata = tdata_opt.unwrap();

        // specular

        if !tdata.mat_data.ks.is_zero() && depth>0{
            let cos = tdata.isect.geo_normal.dot(tdata.isect.wo);
            let ray_dir = 2.0 * cos * tdata.isect.geo_normal - tdata.isect.wo;

            let origin = tdata.isect.point + tdata.isect.geo_normal * self.shadow_bias;
            let sp_ray = Ray::new(origin, ray_dir);

            let sp_tdata_opt = scene.trace(&sp_ray);
            return self.shade_impl(scene, &sp_tdata_opt, depth-1)
        }

        // Loop over scene's light sources and process Ambient Lights
        for light in &scene.lights {
            if let Light::Ambient(ambient_light) = light {
                color += tdata.mat_data.ka * ambient_light.color;
            }
            if let Light::Point(point_light) = light {
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
                     color += tdata.mat_data.kd * light.point_radiance(&tdata.isect.point) * 0f32.max(g_normal.dot(ray_dir));
                }
            }
        }

        color
    }
}

impl Shader for LightShader {
    fn shade(&self, scene: &Scene, tdata_opt: &Option<TraceData>) -> RGB {
        self.shade_impl(scene, tdata_opt, self.reflection_depth)
    }
}

