use std;
use cgmath::*;
use shader::*;
use device::Device;
use material::Material;
use rasterization::triangle;

#[derive(Copy,Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex: Vector2<f32>,
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new() -> Vertex {
        Vertex {
            position: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            normal: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            tex: Vector2::new(0.0_f32, 0.0_f32),
        }
    }
}

pub struct Mesh {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>,
    pub material: Material,
}

pub struct Model {
    pub mesh_arr: Vec<Mesh>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertex_buffer: Vec::<Vertex>::new(),
            index_buffer: Vec::<u32>::new(),
            material: Material::new(),
        }
    }

    pub fn vertex(&mut self, buffer: Vec<Vertex>) {
        self.vertex_buffer = buffer;
    }

    pub fn index(&mut self, buffer: Vec<u32>) -> Result<(), String> {
        if buffer.len() % 3 != 0 {
            return Err(format!("len of index buffer can be N*3, real len is {}", buffer.len()));
        }
        self.index_buffer = buffer;

        Ok(())
    }

    pub fn calc_normal(&mut self) -> Result<(), String> {
        if self.vertex_buffer.len() == 0 ||
            self.index_buffer.len() == 0 {
                return Err("fill vertex and index buffer before calc normals".to_string());
            }
        for vertex in &mut self.vertex_buffer {
            vertex.normal = Vector3::<f32>::new(0.0_f32, 0.0_f32, 0.0_f32);
        }
        let ib = &self.index_buffer;
        // let vb = &self.vertex_buffer;
        for i in 0..ib.len() / 3 {
            let ind = i * 3;
            let v0_ind = ib[ind + 0] as usize;
            let v1_ind = ib[ind + 1] as usize;
            let v2_ind = ib[ind + 2] as usize;
            let v0 = self.vertex_buffer[v0_ind].position;
            let v1 = self.vertex_buffer[v1_ind].position;
            let v2 = self.vertex_buffer[v2_ind].position;
            let normal = v2.sub_v(&v1).cross(&(v1.sub_v(&v0)));
            self.vertex_buffer[v0_ind].normal.add_self_v(&normal);
            self.vertex_buffer[v1_ind].normal.add_self_v(&normal);
            self.vertex_buffer[v2_ind].normal.add_self_v(&normal);
        }

        for vertex in &mut self.vertex_buffer {
            vertex.normal.normalize_self();
        }

        Ok(())
    }

    pub fn draw(&self, shader: &mut Shader, device: &mut Device) -> u32 {
        shader.set_material(&self.material);
        let vertex_func = match self.material.texture {
            None => shader.vertex_func[0],
            Some(_) => shader.vertex_func[1],
        };
        let cnt_triangle = self.index_buffer.len() / 3;
        for indexes in self.index_buffer.chunks(3) {
            let mut points_2d: [Point3<f32>; 3] = [Point3::<f32>::new(0.0, 0.0, 0.0); 3];
            let mut tex_coord: [Vector2<f32>; 3] = [Vector2::<f32>::new(0.0, 0.0); 3];
            let mut vertex_out = [[0.0_f32;MAX_OUT_VALUES];3];
            let mut vertex_out_len = 0;
            for i in 0..3 {
                let p = self.vertex_buffer[indexes[i] as usize];
                let v = p.position;
                let n = p.normal;
                let t = p.tex;

                shader.reset(Vector4::<f32>::new(v.x, v.y, v.z, 1.0_f32), Vector4::<f32>::new(n.x, n.y, n.z, 1.0_f32), t);
                let p_screen = vertex_func(shader);
                vertex_out_len = shader.vertex_out_len;
                let inverse_w = 1.0_f32 / p_screen.w;

                for ind in 0..vertex_out_len {
                    vertex_out[i][ind] = shader.out_vertex_data[ind] * inverse_w;
                }

                tex_coord[i] = t;
                points_2d[i] = Point3::new(
                    (p_screen.x * inverse_w + 1.0_f32) * device.x_size as f32 * 0.5_f32,
                    (p_screen.y * inverse_w + 1.0_f32) * device.y_size as f32 * 0.5_f32,
                    inverse_w);
            }

            let col0 = Vector3::new(points_2d[0].x, points_2d[1].x, points_2d[2].x);
            let col1 = Vector3::new(points_2d[0].y, points_2d[1].y, points_2d[2].y);
            let col2 = Vector3::new(1.0_f32,     1.0_f32,     1.0_f32    );
            let d = Matrix3::from_cols(col0, col1, col2).determinant();
            if d > 0.0_f32 {
                continue;
            }
            // if d < 0.0_f32 {
            //     continue;
            // }

            // calc mip level:
            if self.material.texture.is_some() {
                let texture = shader.texture.as_ref().unwrap();
                let ba_pixel = Vector2::new(points_2d[1].x, points_2d[1].y)
                    .sub_v(&Vector2::new(points_2d[0].x, points_2d[0].y));
                let ca_pixel = Vector2::new(points_2d[2].x, points_2d[2].y)
                    .sub_v(&Vector2::new(points_2d[0].x, points_2d[0].y));

                let tex_size = Vector2::new(texture.levels[0].size_x as f32,
                                            texture.levels[0].size_y as f32);
                let ba_texel = tex_coord[1].sub_v(&tex_coord[0]).mul_v(&tex_size);
                let ca_texel = tex_coord[2].sub_v(&tex_coord[0]).mul_v(&tex_size);
                // cross product in 2d = 2 * square of triangle
                let sq_pixel = ba_pixel.x * ca_pixel.y - ba_pixel.y * ca_pixel.x;
                let sq_texel = ba_texel.x * ca_texel.y - ba_texel.y * ca_texel.x;
                let lod = (sq_texel / sq_pixel).abs().sqrt().max(1.0_f32).log2() as usize;
                shader.texture_lod = std::cmp::min(lod, texture.levels.len() - 1);
                // println!("{}", shader.texture_lod);
            }

            triangle(&mut device.cbuffer,
                     &mut device.zbuffer,
                     device.x_size,
                     device.y_size,
                     points_2d, vertex_out, vertex_out_len, shader);
        }

        cnt_triangle as u32
    }
}

impl Model {
    pub fn new() -> Model {
        Model {
            mesh_arr: Vec::<Mesh>::new(),
        }
    }

    pub fn add(&mut self, mesh: Mesh) {
        self.mesh_arr.push(mesh);
    }

    pub fn draw(&self, shader: &mut Shader, device: &mut Device) -> u32 {
        let mut triangle_cnt: u32 = 0;
        for mesh in &self.mesh_arr {
            triangle_cnt += mesh.draw(shader, device);
        }

        triangle_cnt
    }
}
