use material;
use std::f32::consts;
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

pub fn generate_sphere(points_in_circle: u32, model_material: material::Material) -> Result<Model, String> {
    let k = 4.0_f32;
    let mut model = Model::new();
    model.material_list.push(model_material);

	let step_angle = rad(consts::PI * 2.0_f32 / (points_in_circle as f32));
	let step_tx = 1.0_f32 / (points_in_circle as f32);
    let step_ty = 2.0_f32 / (points_in_circle as f32);

    let mut ty = 0.0_f32;
    let mut angle_b = rad(-consts::PI / 2.0_f32);
    for _ in 0 .. points_in_circle / 2 + 1 {
        let (y, radius) = sin_cos(angle_b);

        let mut tx = 0.0_f32;
        let mut angle_a = rad(0.0_f32);
        for ix in 0 .. points_in_circle + 1 {
            if ix == points_in_circle {
                tx = 1.0_f32;
                angle_a = rad(0.0_f32);
            }
            let (sin_a, cos_a) = sin_cos(angle_a);
            model.vertex_buffer.push(
                Vertex::new(&Vector3::new(radius * cos_a, y, radius * sin_a),
                            &Vector2::new(tx * k, ty * k),
                            &Vector3::new(radius * cos_a, y, radius * sin_a).normalize()));
            tx += step_tx;
            angle_a = angle_a + step_angle;
		}
        ty += step_ty;
		angle_b = angle_b + step_angle;
	}

    let mut mesh = Mesh::new();
    mesh.material_id = 0;

    for ix in 0 .. points_in_circle / 2 {
		let z1 = ix * (points_in_circle + 1);
		let z2 = z1 + (points_in_circle + 1);
		for iy in 0 .. points_in_circle {
			mesh.index_buffer.push((z1 + iy)  as u32);
            mesh.index_buffer.push((z2 + iy)  as u32);
            mesh.index_buffer.push((z1 + iy + 1) as u32);

			mesh.index_buffer.push((z2 + iy + 1) as u32);
            mesh.index_buffer.push((z1 + iy + 1) as u32);
            mesh.index_buffer.push((z2 + iy)  as u32);
		}
	}
    model.mesh_list.push(mesh);

    Ok(model)
}
