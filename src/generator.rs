use material;
use cgmath::{Vector2, Vector3};
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
