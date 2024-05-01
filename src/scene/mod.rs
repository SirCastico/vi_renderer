use std::{path::Path, fs, io::Write};

use crate::{primitives::{Intersectable, material_data::MaterialData, mesh::Mesh}, lights::Light, rays::{intersection::IntersectionData, ray::Ray}, utils::{rgb::RGB, vector::{Point, Vector}}};


#[derive(Debug, Clone, Copy, Default)]
pub struct TraceData{
    pub isect: IntersectionData,
    pub mat_data: MaterialData,
}

#[derive(Debug, Clone, Default)]
pub struct Scene {
    pub prims: Vec<(Mesh, u16)>, 
    pub materials_data: Vec<MaterialData>,
    pub lights: Vec<Light>,
}

impl Scene{
    pub fn new() -> Self{
        Self {
            prims: Vec::new(),
            materials_data: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn trace(&self, ray: &Ray) -> Option<TraceData>{
        if self.prims.len() == 0 {
            return None;
        }
        let mut curr_trace_opt: Option<TraceData> = None;
        for (prim, ind) in self.prims.iter() {
            if let Some(isect) = prim.intersect(ray){
                if let Some(curr_trace) = curr_trace_opt {
                    if curr_trace.isect.depth > isect.depth {
                        curr_trace_opt = Some(TraceData{
                            isect,
                            mat_data: self.materials_data[*ind as usize],
                        });
                    }
                } else {
                    curr_trace_opt = Some(TraceData{
                        isect,
                        mat_data: self.materials_data[*ind as usize],
                    });
                }
            }
        }
        curr_trace_opt
    }

    pub fn visibility(&self, ray: &Ray, depth: f32) -> bool{
        todo!()
    }

    pub fn load_obj_file(&mut self, path: &Path) {
        let (mut obj_models, obj_materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS).expect("failed to load OBJ file");
        let obj_materials = obj_materials.expect("failed to load MTL file");

        if self.materials_data.len()==0{
            self.materials_data.reserve(obj_materials.len()+1);
            self.materials_data.push(MaterialData::default());
        } else {
            self.materials_data.reserve(obj_materials.len());
        }
        self.prims.reserve(obj_models.len());

        let mats_start_ind = self.materials_data.len();
        for obj_mat in obj_materials{
            let mut mat = MaterialData::default();
            if let Some(ka) = obj_mat.ambient {
                mat.ka = ka.into();
            }
            if let Some(kd) = obj_mat.diffuse {
                mat.kd = kd.into();
            }
            if let Some(ks) = obj_mat.specular {
                mat.ks = ks.into();
            }
            if let Some(ns) = obj_mat.shininess {
                mat.ns = ns;
            }
            if let Some(tf_str) = obj_mat.unknown_param.get("Tf"){
                let mut spl_iter = tf_str.split(' ');
                let r = spl_iter.next().unwrap().parse::<f32>().unwrap();
                let g = spl_iter.next().unwrap().parse::<f32>().unwrap();
                let b = spl_iter.next().unwrap().parse::<f32>().unwrap();
                mat.kt = RGB::new(r, g, b);
            }
            self.materials_data.push(mat);
        }
        let mut file = fs::File::create("verts.txt").unwrap();

        for obj_model in obj_models.iter_mut(){
            let obj_mesh = &mut obj_model.mesh;
            let obj_positions = std::mem::take(&mut obj_mesh.positions);
            let obj_pos_inds = std::mem::take(&mut obj_mesh.indices);
            let obj_normals = std::mem::take(&mut obj_mesh.normals);
            let obj_normal_inds = std::mem::take(&mut obj_mesh.normal_indices);

            let positions: Vec<Point> = obj_positions
                .chunks_exact(3)
                .map(|a|{Point::new(a[0], a[1], a[2])})
                .collect();

            let normals: Vec<Vector> = obj_normals
                .chunks_exact(3)
                .map(|a|{Vector::new(a[0], a[1], a[2])})
                .collect();

            //println!("Mesh name: {}\n n_verts:{}", obj_model.name, positions.len());

            file.write_all(obj_model.name.as_bytes()).unwrap();
            file.write_all("\n".as_bytes()).unwrap();

            for inds in obj_pos_inds.chunks_exact(3){
                let p0 = positions[inds[0] as usize];
                let p1 = positions[inds[1] as usize];
                let p2 = positions[inds[2] as usize];

                write!(file, "{:?},{:?},{:?}\n", p0, p1, p2).unwrap();
            }
            file.write_all("\n".as_bytes()).unwrap();

            let mesh = Mesh::new(positions, normals, obj_pos_inds, obj_normal_inds);
            let mat_ind: u16 = if let Some(m_id) = obj_mesh.material_id{
                (m_id + mats_start_ind).try_into().unwrap()
            } else {
                0
            };

            self.prims.push((mesh, mat_ind));
        }
    }

    pub fn add_light(&mut self, light: Light){
        self.lights.push(light);
    }
}
