use cgmath::*;
use shader::*;
use device::Device;
use rasterization::triangle;

#[derive(Copy,Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new() -> Vertex {
        Vertex {
            position: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            normal: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
        }
    }
}

pub struct Mesh {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>,
}

pub struct Model {
    pub mesh_arr: Vec<Mesh>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertex_buffer: Vec::<Vertex>::new(),
            index_buffer: Vec::<u32>::new(),
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
            // let normal = v1.sub_v(&v0).cross(&(v2.sub_v(&v1)));
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
        let vertex_func = shader.vertex_func;
        let cnt_triangle = self.index_buffer.len() / 3;
        for indexes in self.index_buffer.chunks(3) {
            let mut points: [Point3<f32>; 3] = [Point3::<f32>::new(0.0, 0.0, 0.0); 3];
            let mut vertex_out = [[0.0_f32;MAX_OUT_VALUES];3];
            let mut vertex_out_len = 0;
            for i in 0..3 {
                let v = self.vertex_buffer[indexes[i] as usize].position;
                let n = self.vertex_buffer[indexes[i] as usize].normal;

                shader.reset(Vector4::<f32>::new(v.x, v.y, v.z, 1.0_f32), Vector4::<f32>::new(n.x, n.y, n.z, 1.0_f32));
                let p_screen = vertex_func(shader);
                vertex_out_len = shader.vertex_out_len;
                let inverse_w = 1.0_f32 / p_screen.w;

                for ind in 0..vertex_out_len {
                    vertex_out[i][ind] = shader.out_vertex_data[ind] * inverse_w;
                }
                
                points[i] = Point3::new(
                    (p_screen.x * inverse_w + 1.0_f32) * device.x_size as f32 * 0.5_f32,
                    (p_screen.y * inverse_w + 1.0_f32) * device.y_size as f32 * 0.5_f32,
                    inverse_w);
            }

            let col0 = Vector3::new(points[0].x, points[1].x, points[2].x);
            let col1 = Vector3::new(points[0].y, points[1].y, points[2].y);
            let col2 = Vector3::new(1.0_f32,     1.0_f32,     1.0_f32    );
            let d = Matrix3::from_cols(col0, col1, col2).determinant();
            if d > 0.0_f32 {
                continue;
            }
            // if d < 0.0_f32 {
            //     continue;
            // }
            
            triangle(&mut device.cbuffer,
                     &mut device.zbuffer,
                     device.x_size,
                     device.y_size,
                     points, vertex_out, vertex_out_len, shader);
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
