use cgmath::*;
use shader::*;
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
    pub normal_buffer: Vec<Vector3<f32>>,
    pub index_buffer: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertex_buffer: Vec::<Vertex>::new(),
            normal_buffer: Vec::<Vector3<f32>>::new(),
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
        self.normal_buffer.clear();
        let ib = &self.index_buffer;
        let vb = &self.vertex_buffer;
        for i in 0..ib.len() / 3 {
            let ind = i * 3;
            let v0 = vb[ib[ind + 0] as usize].position;
            let v1 = vb[ib[ind + 1] as usize].position;
            let v2 = vb[ib[ind + 2] as usize].position;
            self.normal_buffer.push(v0.sub_v(&v1).cross(&v0.sub_v(&v2)).normalize());
        }

        Ok(())
    }

    pub fn draw(&self, shader: &mut Shader, device: &mut Device) -> u32 {
        let cnt_triangle = self.index_buffer.len() / 3;
        for (triangle_index, indexes) in self.index_buffer.chunks(3).enumerate() {
            let norm = self.normal_buffer[triangle_index];
            shader.set_vec4(VEC_NORM, Vector4::new(norm.x, norm.y, norm.z, 0.0_f32));
            
            let mut points: Vec<Point3<f32>> = vec![];
            for i in 0..3 {
                let v3 = self.vertex_buffer[indexes[i] as usize].position;
                let v4 = Vector4::<f32>::new(v3.x, v3.y, v3.z, 1.0_f32);
                shader.set_vec4(VEC_POS, v4);
                let p_screen = shader.vertex();
                let inverse_w = 1.0_f32 / p_screen.w;
                
                points.push(
                    Point3::new(
                        (p_screen.x * inverse_w + 1.0_f32) * device.x_size as f32 * 0.5_f32,
                        (p_screen.y * inverse_w + 1.0_f32) * device.y_size as f32 * 0.5_f32,
                        inverse_w));
            }

            let col0 = Vector3::new(points[0].x, points[1].x, points[2].x);
            let col1 = Vector3::new(points[0].y, points[1].y, points[2].y);
            let col2 = Vector3::new(1.0_f32,     1.0_f32,     1.0_f32    );
            let d = Matrix3::from_cols(col0, col1, col2).determinant();
            if d > 0.0_f32 {
                continue;
            }
            
            triangle(&mut device.cbuffer,
                     &mut device.zbuffer,
                     device.x_size,
                     device.y_size,
                     points[0], points[1], points[2], shader);
        }

        cnt_triangle as u32
    }
}

#[allow(dead_code)]
pub fn generate_square() -> Result<Mesh, String> {
    let mut mesh = Mesh::new();
    let mut vb: Vec<Vertex> = vec![Vertex::new(); 4];
    vb[0].position = Vector3::new(-0.5_f32,  0.5_f32, 0.0_f32);
    vb[1].position = Vector3::new( 0.5_f32,  0.5_f32, 0.0_f32);
    vb[2].position = Vector3::new( 0.5_f32, -0.5_f32, 0.0_f32);
    vb[3].position = Vector3::new(-0.5_f32, -0.5_f32, 0.0_f32);
    mesh.vertex(vb);
    let mut ib: Vec<u32> = vec![0; 2 * 3];
    ib[0] = 0;
    ib[1] = 1;
    ib[2] = 2;
    ib[3] = 0;
    ib[4] = 2;
    ib[5] = 3;
    try!(mesh.index(ib));

    Ok(mesh)
}

#[allow(dead_code)]
pub fn generate_cube() -> Result<Mesh, String> {
    let mut mesh = Mesh::new();
    let mut vb: Vec<Vertex> = vec![Vertex::new(); 24];
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
    mesh.vertex(vb);
    let mut ib: Vec<u32> = vec![0; 12 * 3];
	for i in 0..6 {
		let sm = (i * 4) as u32;
        let ind = i * 6;
		ib[ind + 0] = sm + 0; ib[ind + 1] = sm + 1; ib[ind + 2] = sm + 2;
		ib[ind + 3] = sm + 0; ib[ind + 4] = sm + 2; ib[ind + 5] = sm + 3;
	}
    try!(mesh.index(ib));

    Ok(mesh)
}
