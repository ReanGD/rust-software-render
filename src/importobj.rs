use std;
use material;
use std::rc::Rc;
use genmesh::Polygon;
use utils::get_full_path;
use cgmath::{Vector, Vector2, Vector3};
use mesh::{Model, Mesh, Vertex};
use obj::{Obj, Material, IndexTuple, load};
use memory::cast_to;

pub struct ModelObj<'a> {
    model_dir: std::path::PathBuf,
    map: std::collections::HashMap<IndexTuple, u32>,
    model: Model,
    position_buffer: &'a [Vector3<f32>],
    normal_buffer: &'a [Vector3<f32>],
    tex_buffer: Vec<Vector2<f32>>,
}

impl<'a> ModelObj<'a> {
    fn get_index(&mut self, index: IndexTuple) -> u32 {
        let zero2 = Vector2::new(0.0_f32, 0.0_f32);
        let zero3 = Vector3::new(0.0_f32, 0.0_f32, 0.0_f32);
        if self.map.get(&index).is_none() {
            self.model.vertex_buffer.push(
                Vertex::new(
                    &self.position_buffer[index.0],
                    &match index.1 {
                        Some(t) => self.tex_buffer[t],
                        None => zero2,
                    },
                    &match index.2 {
                        Some(n) => self.normal_buffer[n],
                        None => zero3,
                    }));
        };
        let len = self.model.vertex_buffer.len() as u32 - 1;
        self.map.entry(index).or_insert(len).clone()
    }

    fn parse(&mut self, model_obj: &Obj<Rc<Material>>) -> Result<(), String> {
        for object in model_obj.object_iter() {
            for group in object.group_iter() {
                let mut mesh = Mesh::new();
                for polygon in group.indices() {
                    match polygon {
                        &Polygon::PolyTri(p) => {
                            let ind0 = self.get_index(p.x);
                            let ind1 = self.get_index(p.y);
                            let ind2 = self.get_index(p.z);
                            mesh.index_buffer.push(ind0);
                            mesh.index_buffer.push(ind1);
                            mesh.index_buffer.push(ind2);
                        },
                        &Polygon::PolyQuad(p) => {
                            let ind0 = self.get_index(p.x);
                            let ind1 = self.get_index(p.y);
                            let ind2 = self.get_index(p.z);
                            let ind3 = self.get_index(p.w);
                            mesh.index_buffer.push(ind0);
                            mesh.index_buffer.push(ind1);
                            mesh.index_buffer.push(ind2);
                            mesh.index_buffer.push(ind0);
                            mesh.index_buffer.push(ind2);
                            mesh.index_buffer.push(ind3);
                        },
                    };
                }
                let material = group.material.clone();
                mesh.material_id = match material {
                    Some(m) => {
                        let mut mat = material::Material::new();
                        match m.ka {
                            Some(v) => mat.ambient = Vector3::new(v[0], v[1], v[2]).mul_s(255.0_f32),
                            None => {},
                        };
                        match m.kd {
                            Some(v) => mat.diffuse = Vector3::new(v[0], v[1], v[2]).mul_s(255.0_f32),
                            None => {},
                        };
                        match m.ks {
                            Some(v) => mat.specular = Vector3::new(v[0], v[1], v[2]).mul_s(255.0_f32),
                            None => {},
                        };
                        match m.map_kd {
                            Some(ref path) => try!(mat.texture_from_dir(&self.model_dir, &path)),
                            None => {},
                        };
                        mat.calc_ambient_intensity();
                        self.model.material_list.push(mat);
                        self.model.material_list.len() - 1
                    },
                    None => 0,
                };
                self.model.mesh_list.push(mesh);
            }
        }

        Ok(())
    }

    fn calc_aabb(position: &[[f32; 3]]) -> (Vector3<f32>, Vector3<f32>) {
        let p = position[0];
        let mut min = Vector3::new(p[0], p[1], p[2]);
        let mut max = min;
        for p in position {
            min.x = min.x.min(p[0]);
            max.x = max.x.max(p[0]);
            min.y = min.y.min(p[1]);
            max.y = max.y.max(p[1]);
            min.z = min.z.min(p[2]);
            max.z = max.z.max(p[2]);
        }

        (min, max)
    }

    pub fn load(filename: &str) -> Result<Model, String> {
        let filepath = try!(get_full_path(filename));
        let mut model_dir = std::path::PathBuf::from(filepath.clone());
        if !model_dir.pop() {
            return Err(format!("not found parent dir for filepath {}", filepath));
        }

        let model_obj: Obj<Rc<Material>> = load(std::path::Path::new(&filepath)).unwrap();
        let (min, max) = ModelObj::calc_aabb(model_obj.position());
        let mut this = ModelObj {
            model_dir: model_dir,
            map: std::collections::HashMap::<IndexTuple, u32>::new(),
            model: Model::new(min, max),
            position_buffer: cast_to(model_obj.position()),
            normal_buffer: cast_to(model_obj.normal()),
            tex_buffer: Vec::<Vector2<f32>>::with_capacity(model_obj.texture().len()),
        };

        for p in model_obj.texture() {
            let u = p[0] % 1.0_f32;
            let v = p[1] % 1.0_f32;
            this.tex_buffer.push(Vector2::new(if u < 0.0_f32 { u + 1.0_f32 } else { u },
                                              if v < 0.0_f32 { v + 1.0_f32 } else { v }));
        }

        let mut def_mat = material::Material::new();
        def_mat.diffuse = Vector3::new(255.0_f32, 0.0_f32, 0.0_f32);
        def_mat.ambient = Vector3::new(255.0_f32, 0.0_f32, 0.0_f32);
        this.model.material_list.push(def_mat);

        try!(this.parse(&model_obj));

        Ok(this.model)
    }
}
