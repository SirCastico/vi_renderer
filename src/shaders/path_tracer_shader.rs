use core::f32;

use rand::{thread_rng, Rng};

use crate::{lights::Light, rays::ray::{self, Ray}, scene::{Scene, TraceData}, utils::{rgb::RGB, vector::Vector}};

use super::Shader;



pub struct PathTracerShader{
    pub background: RGB,
    pub shadow_bias: f32,
    pub reflection_prob: f32,
    pub reflection_depth: u16
}

impl PathTracerShader{

    fn diffuse_reflection(&self, scene: &Scene, tdata: &TraceData, depth:u16) -> RGB{
        let color = RGB::new(0.0, 0.0, 0.0);

        let mut rng = thread_rng();
        let rnd: [f32; 2] = [rng.gen(), rng.gen()];

        let cos_theta = rnd[1].sqrt();
        let d_around_z = Vector::new(
            (2.0*f32::consts::PI*rnd[0]).cos()*(1.0-rnd[1]).sqrt(), 
            (2.0*f32::consts::PI*rnd[0]).sin()*(1.0-rnd[1]).sqrt(), 
            cos_theta, 
        );
        let pdf = cos_theta / f32::consts::PI;
        let (rx, ry) = tdata.isect.geo_normal.coordinate_system();

        let diffuse = Ray::new(
            tdata.isect.point,
            d_around_z.rotate(rx, ry, tdata.isect.geo_normal),
        );

        let ntdata_opt = scene.trace(&diffuse);
        if let Some(ntdata) = ntdata_opt{
            if ntdata.mat_data.le.is_some(){
                return color;
            }
        }
        let rcolor: RGB;
        if depth>0{
            rcolor = self.shade_impl(scene, &ntdata_opt, depth-1);
        }
        else {
            rcolor = self.shade_impl(scene, &ntdata_opt, depth);
        }
        return (tdata.mat_data.kd * cos_theta * rcolor) / pdf;
    }

    fn specular_reflection(&self, scene: &Scene, tdata: &TraceData, depth:u16) -> RGB{
        let gn = tdata.isect.geo_normal.face_forward(tdata.isect.wo);
        let cos = gn.dot(tdata.isect.wo);
        let ray_dir = 2.0 * cos * gn - tdata.isect.wo;

        let origin = tdata.isect.point + gn * self.shadow_bias;
        let sp_ray = Ray::new(origin, ray_dir);

        let sp_tdata_opt = scene.trace(&sp_ray);
        let rcolor: RGB;
        if depth>0{
            rcolor = self.shade_impl(scene, &sp_tdata_opt, depth-1);
        }
        else {
            rcolor = self.shade_impl(scene, &sp_tdata_opt, depth);
        }
        return rcolor * tdata.mat_data.ks;
    }

    fn direct_lighting_smpl(&self, scene: &Scene, tdata: &TraceData) -> RGB{
        let mut color = RGB::default();
        let rnd_ind = thread_rng().gen::<usize>() % scene.lights.len();

        match scene.lights[rnd_ind]{
            Light::Ambient(ambient_light) => {
                color += tdata.mat_data.ka * ambient_light.color;
            }
            Light::Point(point_light) => {
                if tdata.mat_data.kd.is_zero() {
                    return color;
                }
                let mut ray_dir: Vector = (point_light.position - tdata.isect.point).into();
                let light_dist = ray_dir.norm();
                ray_dir.normalize();

                let mut g_normal = tdata.isect.geo_normal.face_forward(tdata.isect.wo);
                g_normal.normalize();

                if ray_dir.dot(g_normal) < 0.0{
                    return color;
                }

                let ray_o = tdata.isect.point + g_normal * self.shadow_bias;
                let ray: Ray = Ray::new(ray_o, ray_dir);
                let light_tdata_opt = scene.trace(&ray); // TODO: visibility instead of trace
                
                if light_tdata_opt.is_none() || light_tdata_opt.unwrap().isect.depth >= light_dist {
                    color += tdata.mat_data.kd * point_light.color * 0f32.max(g_normal.dot(ray_dir));
                }
            }
            Light::Area(area_light) => {
                if tdata.mat_data.kd.is_zero() {
                    return color;
                }
                
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
        color * scene.lights.len() as f32
    }

    fn direct_lighting(&self, scene: &Scene, tdata: &TraceData) -> RGB{
        let mut color = RGB::default();

        for light in scene.lights.iter(){
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
                    if tdata.mat_data.kd.is_zero() {
                        continue;
                    }
                    
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

        let rnd_depth: f32 = thread_rng().gen();
        if depth>0 || rnd_depth<self.reflection_prob{
            //println!("yep: {}", depth);
            let lcolor: RGB;
            let mdata = &tdata.mat_data;
            let s_p = mdata.ks.y() / (mdata.ks.y()+mdata.kd.y());
            let rnd: f32 = thread_rng().gen();
            if rnd <= s_p || s_p >= (1.0-ray::EPSILON){
                lcolor = self.specular_reflection(scene, &tdata, depth);
            } else {
                lcolor = self.diffuse_reflection(scene, &tdata, depth);
            }
            if depth>0 {
                color += lcolor;
            } else {
                color += lcolor / self.reflection_prob;
            }
        }

        color += self.direct_lighting_smpl(scene, &tdata);
        color
    }

}

impl Shader for PathTracerShader{
    fn shade(&self, scene: &crate::scene::Scene, tdata_opt: &Option<crate::scene::TraceData>) -> RGB {
        self.shade_impl(scene, tdata_opt, self.reflection_depth)
    }
}