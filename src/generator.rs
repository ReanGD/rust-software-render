use material;
use std::f32;
use cgmath::{Vector2, Vector3, EuclideanVector, rad, sin_cos};
use mesh::{Vertex, Mesh, Model};

pub fn generate_plane(model_material: material::Material) -> Result<Model, String> {
    let mut model = Model::new();
    model.material_list.push(model_material);

    let k = 3.0_f32;
    model.vertex_buffer.push(
        Vertex::new(&Vector3::new(-0.5_f32,  0.5_f32, 0.0_f32),
                    &Vector2::new( 0.0_f32,  1.0_f32 * k),
                    &Vector3::new( 0.0_f32,  0.0_f32, 1.0_f32)));

    model.vertex_buffer.push(
        Vertex::new(&Vector3::new( 0.5_f32,  0.5_f32, 0.0_f32),
                    &Vector2::new( 1.0_f32 * k,  1.0_f32 * k),
                    &Vector3::new( 0.0_f32,  0.0_f32, 1.0_f32)));

    model.vertex_buffer.push(
        Vertex::new(&Vector3::new( 0.5_f32, -0.5_f32, 0.0_f32),
                    &Vector2::new( 1.0_f32 * k,  0.0_f32),
                    &Vector3::new( 0.0_f32,  0.0_f32, 1.0_f32)));

    model.vertex_buffer.push(
        Vertex::new(&Vector3::new(-0.5_f32, -0.5_f32, 0.0_f32),
                    &Vector2::new( 0.0_f32,  0.0_f32),
                    &Vector3::new( 0.0_f32,  0.0_f32, 1.0_f32)));

    let mut mesh = Mesh::new();
    mesh.material_id = 0;
    mesh.index_buffer.push(0);
    mesh.index_buffer.push(1);
    mesh.index_buffer.push(2);
    mesh.index_buffer.push(0);
    mesh.index_buffer.push(2);
    mesh.index_buffer.push(3);

    model.mesh_list.push(mesh);

    Ok(model)
}

pub fn generate_sphere(cnt_vertex: usize, model_material: material::Material) -> Result<Model, String> {
    let k = 3.0_f32;
    let mut model = Model::new();
    model.material_list.push(model_material);

	let plg = cnt_vertex / 2 - 1;
	let vertex_cnt = plg * cnt_vertex + 2;

    let mut angle_b = rad(-f32::consts::PI / 2.0_f32);
    let mut ty = 0.0_f32;
	let step_a = rad(f32::consts::PI * 2.0_f32 / (cnt_vertex as f32));
	let step_b = rad(f32::consts::PI / (plg as f32 + 1.0_f32));
	let step_tx = 1.0_f32 / (cnt_vertex as f32);
    let step_ty = 1.0_f32 / (plg as f32 + 1.0_f32);

    model.vertex_buffer.push(
        Vertex::new(&Vector3::new(0.0_f32, -1.0_f32, 0.0_f32),
                    &Vector2::new(0.5_f32 * k,  0.0_f32),
                    &Vector3::new(0.0_f32, -1.0_f32, 0.0_f32)));

    for _ in 0 .. plg {
		angle_b = angle_b + step_b;
        ty += step_ty;
        let (y, radius) = sin_cos(angle_b);

        let mut tx = 0.0_f32;
        let mut angle_a = rad(0.0_f32);
        for _ in 0 .. cnt_vertex {
            let (sin_a, cos_a) = sin_cos(angle_a);
            model.vertex_buffer.push(
                Vertex::new(&Vector3::new(radius * cos_a, y, radius * sin_a),
                            &Vector2::new(tx * k, ty * k),
                            &Vector3::new(radius * cos_a, y, radius * sin_a).normalize()));
            angle_a = angle_a + step_a;
            tx += step_tx;
		}
	}
    model.vertex_buffer.push(
        Vertex::new(&Vector3::new(0.0_f32, 1.0_f32, 0.0_f32),
                    &Vector2::new(0.5_f32 * k, 1.0_f32 * k),
                    &Vector3::new(0.0_f32, 1.0_f32, 0.0_f32)));

    let mut mesh = Mesh::new();
    mesh.material_id = 0;

    for ix in 0 .. (plg-1) {
		let z1 = ix * cnt_vertex + 1;
		let z2 = z1 + cnt_vertex;
		for iy in 0 .. cnt_vertex {
            let iy2 = if iy == (cnt_vertex - 1) {
                0
            } else {
                iy + 1
            };
			mesh.index_buffer.push((z1 + iy)  as u32);
            mesh.index_buffer.push((z2 + iy)  as u32);
            mesh.index_buffer.push((z1 + iy2) as u32);

			mesh.index_buffer.push((z2 + iy2) as u32);
            mesh.index_buffer.push((z1 + iy2) as u32);
            mesh.index_buffer.push((z2 + iy)  as u32);
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
        mesh.index_buffer.push(0 as u32);
        mesh.index_buffer.push((z1 + ix)  as u32);
        mesh.index_buffer.push((z1 + ix2) as u32);

        mesh.index_buffer.push((vertex_cnt - 1) as u32);
        mesh.index_buffer.push((z2 + ix2) as u32);
        mesh.index_buffer.push((z2 + ix) as u32);
    }
    model.mesh_list.push(mesh);

    Ok(model)
}
