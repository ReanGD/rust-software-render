use std;
use rand;
use cgmath::*;
use device::Device;
use rasterization::triangle;
use rand::distributions::{IndependentSample, Range};

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
    pub colors: Vec<u32>,
}

impl Mesh {
    pub fn new(vertex_cnt: usize, triagle_cnt: usize) -> Mesh {
        let vertex_buffer: Vec<Vertex> = vec![Vertex::new(); vertex_cnt];
        let index_buffer: Vec<u32> = vec![0; triagle_cnt * 3];
        let mut colors: Vec<u32> = vec![];
        
        let color_range = Range::new(0, std::u32::MAX);
        let mut rng = rand::thread_rng();
        for _ in 0..triagle_cnt {
            colors.push(color_range.ind_sample(&mut rng));
        }

        Mesh {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            colors: colors,
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
            let color = self.colors[triangle_index];
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
                     points[0], points[1], points[2], color);
        }

    }
}

#[allow(dead_code)]
pub fn generate_square() -> Mesh {
    let mut mesh = Mesh::new(4, 2);
    {
        let mut vb = mesh.get_vertex();
        vb[0].position = Vector3::new(-0.5_f32,  0.5_f32, 0.0_f32);
        vb[1].position = Vector3::new( 0.5_f32,  0.5_f32, 0.0_f32);
        vb[2].position = Vector3::new( 0.5_f32, -0.5_f32, 0.0_f32);
        vb[3].position = Vector3::new(-0.5_f32, -0.5_f32, 0.0_f32);
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

#[allow(dead_code)]
pub fn generate_cube() -> Mesh {
    let mut mesh = Mesh::new(24, 12);
    {
        let mut vb = mesh.get_vertex();
	    vb[ 0].position	= Vector3::new(-0.5_f32,-0.5_f32,-0.5_f32);
	    vb[ 1].position	= Vector3::new(-0.5_f32, 0.5_f32,-0.5_f32);
	    vb[ 2].position	= Vector3::new( 0.5_f32, 0.5_f32,-0.5_f32);
	    vb[ 3].position	= Vector3::new( 0.5_f32,-0.5_f32,-0.5_f32);
	    vb[ 4].position	= Vector3::new( 0.5_f32,-0.5_f32, 0.5_f32);
	    vb[ 5].position	= Vector3::new( 0.5_f32, 0.5_f32, 0.5_f32);
	    vb[ 6].position	= Vector3::new(-0.5_f32, 0.5_f32, 0.5_f32);
	    vb[ 7].position	= Vector3::new(-0.5_f32,-0.5_f32, 0.5_f32);
	    vb[ 8].position	= Vector3::new(-0.5_f32,-0.5_f32, 0.5_f32);
	    vb[ 9].position	= Vector3::new(-0.5_f32, 0.5_f32, 0.5_f32);
	    vb[10].position	= Vector3::new(-0.5_f32, 0.5_f32,-0.5_f32);
	    vb[11].position	= Vector3::new(-0.5_f32,-0.5_f32,-0.5_f32);
	    vb[12].position	= Vector3::new( 0.5_f32,-0.5_f32,-0.5_f32);
	    vb[13].position	= Vector3::new( 0.5_f32, 0.5_f32,-0.5_f32);
	    vb[14].position	= Vector3::new( 0.5_f32, 0.5_f32, 0.5_f32);
	    vb[15].position	= Vector3::new( 0.5_f32,-0.5_f32, 0.5_f32);
	    vb[16].position	= Vector3::new(-0.5_f32,-0.5_f32, 0.5_f32);
	    vb[17].position	= Vector3::new(-0.5_f32,-0.5_f32,-0.5_f32);
	    vb[18].position	= Vector3::new( 0.5_f32,-0.5_f32,-0.5_f32);
	    vb[19].position	= Vector3::new( 0.5_f32,-0.5_f32, 0.5_f32);
	    vb[20].position	= Vector3::new(-0.5_f32, 0.5_f32,-0.5_f32);
	    vb[21].position	= Vector3::new(-0.5_f32, 0.5_f32, 0.5_f32);
	    vb[22].position	= Vector3::new( 0.5_f32, 0.5_f32, 0.5_f32);
	    vb[23].position	= Vector3::new( 0.5_f32, 0.5_f32,-0.5_f32);
    }
    {
        let mut ib = mesh.get_index();
	    for i in 0..6 {
		    let sm = (i * 4) as u32;
            let ind = i * 6;
		    ib[ind + 0] = sm + 0; ib[ind + 1] = sm + 1; ib[ind + 2] = sm + 2;
		    ib[ind + 3] = sm + 0; ib[ind + 4] = sm + 2; ib[ind + 5] = sm + 3;
	    }
    }

    mesh
}
