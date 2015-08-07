use cgmath::*;
use rasterization;
use device::Device;
use rasterization::triangle;

#[derive(Copy,Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            position: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
        }
    }
}

pub struct Mesh {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>,
}

impl Mesh {
    pub fn new(vertex_cnt: usize, triagle_cnt: usize) -> Mesh {
        let vertex_buffer: Vec<Vertex> = vec![Vertex::new(); vertex_cnt];
        let index_buffer: Vec<u32> = vec![0; triagle_cnt * 3];

        Mesh {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }

    pub fn get_vertex(&mut self) -> &mut Vec<Vertex> {
        &mut self.vertex_buffer
    }

    pub fn get_index(&mut self) -> &mut Vec<u32> {
        &mut self.index_buffer
    }

    pub fn draw(&self, mat: &Matrix4<f32>, device: &mut Device) {
        for triangle_index in 0..self.index_buffer.len() / 3 {
            let mut points: Vec<Point2<f32>> = vec![];
            for i in 0..3 {
                let ind = self.index_buffer[triangle_index * 3 + i] as usize;
                let v3 = self.vertex_buffer[ind].position;
                let v4 = Vector4::<f32>::new(v3.x, v3.y, v3.z, 1.0_f32);
                let p_screen = mat.mul_v(&v4);
                points.push(
                    Point2::new(
                        (p_screen.x/p_screen.w + 1.0_f32) * device.x_size as f32 * 0.5_f32,
                        (p_screen.y/p_screen.w + 1.0_f32) * device.y_size as f32 * 0.5_f32));
            }
            triangle(&mut device.cbuffer,
                     device.x_size,
                     device.y_size,
                     points[0], points[1], points[2], 0xFF00FF);
        }

    }
}

pub fn GenerateSquare() -> Mesh {
    let mut mesh = Mesh::new(4, 2);
    {
        let mut vb = mesh.get_vertex();
        vb[0].position = Vector3::new(-5.0_f32,  5.0_f32, 0.0_f32);
        vb[1].position = Vector3::new( 5.0_f32,  5.0_f32, 0.0_f32);
        vb[2].position = Vector3::new( 5.0_f32, -5.0_f32, 0.0_f32);
        vb[3].position = Vector3::new(-5.0_f32, -5.0_f32, 0.0_f32);
    }
    {
        let mut ib = mesh.get_index();
        ib[0] = 0;
        ib[1] = 1;
        ib[2] = 2;
        ib[3] = 0;
        ib[4] = 2;
        ib[5] = 3;
    }

    mesh
}
