use std;
use cgmath::*;
use mesh::{Vertex, Mesh, Model};

pub fn generate_plane() -> Result<Model, String> {
    let mut model = Model::new();
    let mut mesh = Mesh::new();
    let mut vb: Vec<Vertex> = vec![Vertex::new(); 4];
    vb[0].position = Vector3::new(-0.5_f32,  0.5_f32, 0.0_f32);
    vb[0].normal   = Vector3::new( 0.0_f32,  0.0_f32, 1.0_f32);
    vb[1].position = Vector3::new( 0.5_f32,  0.5_f32, 0.0_f32);
    vb[1].normal   = Vector3::new( 0.0_f32,  0.0_f32, 1.0_f32);
    vb[2].position = Vector3::new( 0.5_f32, -0.5_f32, 0.0_f32);
    vb[2].normal   = Vector3::new( 0.0_f32,  0.0_f32, 1.0_f32);
    vb[3].position = Vector3::new(-0.5_f32, -0.5_f32, 0.0_f32);
    vb[3].normal   = Vector3::new( 0.0_f32,  0.0_f32, 1.0_f32);
    mesh.vertex(vb);
    let mut ib: Vec<u32> = vec![0; 2 * 3];
    ib[0] = 0;
    ib[1] = 1;
    ib[2] = 2;
    ib[3] = 0;
    ib[4] = 2;
    ib[5] = 3;
    try!(mesh.index(ib));
    model.add(mesh);

    Ok(model)
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

pub fn generate_sphere(cnt_vertex: usize) -> Result<Model, String> {
    let mut model = Model::new();
    let mut mesh = Mesh::new();

	let plg = cnt_vertex / 2 - 1;
	let vertex_cnt = plg * cnt_vertex + 2;
	let index_cnt = 6 * cnt_vertex * plg;
    
    let mut vb: Vec<Vertex> = vec![Vertex::new(); vertex_cnt];
    let mut angle_b = rad(-std::f32::consts::PI / 2.0_f32);
	let step_a = rad(std::f32::consts::PI * 2.0_f32 / (cnt_vertex as f32));
	let step_b = rad(std::f32::consts::PI / (plg as f32 + 1.0_f32));
    let mut ind = 1;
    for _ in 0 .. plg {
		angle_b = angle_b + step_b;
        let (y, radius) = sin_cos(angle_b);

        let mut angle_a = rad(0.0_f32);
        for _ in 0 .. cnt_vertex {
            let (sin_a, cos_a) = sin_cos(angle_a);
			vb[ind].position = Vector3::new(radius * cos_a, y, radius * sin_a);
			vb[ind].normal   = Vector3::new(radius * cos_a, y, radius * sin_a).normalize();
            ind += 1;
            angle_a = angle_a + step_a;
		}
	}
	vb[0].position = Vector3::new(0.0_f32, -1.0_f32, 0.0_f32);
	vb[0].normal = Vector3::new(0.0_f32, -1.0_f32, 0.0_f32);
	vb[vertex_cnt - 1].position = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);
	vb[vertex_cnt - 1].normal = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);

    mesh.vertex(vb);

    let mut ib: Vec<u32> = vec![0; index_cnt];
    let mut ind = 0;
    for ix in 0 .. (plg-1) {
		let z1 = ix * cnt_vertex + 1;
		let z2 = z1 + cnt_vertex;
		for iy in 0 .. cnt_vertex {
            let iy2 = if iy == (cnt_vertex - 1) {
                0
            } else {
                iy + 1
            };
			ib[ind + 0] = (z1 + iy)  as u32;
            ib[ind + 1] = (z1 + iy2) as u32;
            ib[ind + 2] = (z2 + iy)  as u32;

			ib[ind + 3] = (z2 + iy2) as u32;
            ib[ind + 4] = (z2 + iy)  as u32;
            ib[ind + 5] = (z1 + iy2) as u32;
            ind += 6;
		}
	}

    let z1 = 1;
    let z2 = vertex_cnt - cnt_vertex - 1;
    for ix in 0 .. cnt_vertex {
        let ix2 = if ix == (cnt_vertex - 1) {
            0
        } else {
            ix + 1
        };
        ib[ind + 0] = 0 as u32;
        ib[ind + 1] = (z1 + ix2) as u32;
        ib[ind + 2] = (z1 + ix)  as u32;

        ib[ind + 3] = (vertex_cnt - 1) as u32;
        ib[ind + 4] = (z2 + ix) as u32;
        ib[ind + 5] = (z2 + ix2) as u32;
        ind += 6;
    }
    try!(mesh.index(ib));
    // mesh.calc_normal();
    model.add(mesh);

    Ok(model)
}
