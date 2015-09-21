extern crate obj;
extern crate genmesh;
extern crate sdl2;
extern crate sdl2_image;
extern crate time;
extern crate cgmath;

mod mesh;
mod utils;
mod scene;
mod tests;
mod device;
mod memory;
mod shader;
mod texture;
mod material;
mod importobj;
mod generator;
mod rasterization;

use cgmath::*;
use importobj::ModelObj;

pub fn main() {
    let eye;
    let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);
    let center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
    let model;
    let init_matrix;
    let shader_type = shader::ShaderType::Lambert;
    let model_index = 0;

    let mut angle = rad(0.0_f32);
    let mut add_angle = rad(2.0_f32 * std::f32::consts::PI / 180.0_f32);
    match model_index {
        0 => {
            eye = Point3::new(0.0_f32, 0.0_f32, -0.5_f32);
            add_angle = rad(0.0_f32);
            init_matrix = Matrix4::from(Matrix3::from_angle_x(rad(std::f32::consts::PI * 0.25_f32)));
            let mut mat = material::Material::new();
            mat.create_texture(utils::get_base_dir().unwrap().join("lego.png").as_path()).unwrap();
            model = generator::generate_plane(mat).unwrap();
        },
        1 => {
            eye = Point3::new(0.0_f32, 0.3_f32, 0.6_f32);
            model = ModelObj::load(std::path::Path::new("monster1/monster.obj")).unwrap();
            // model = ModelObj::load("nokia/nokia.obj").unwrap();
            // model = ModelObj::load("droid/attack_droid.obj").unwrap();
            init_matrix = model.to_center_matrix();
        },
        _ => return
    };

    let mut shader = shader::Shader::new(shader_type);

    let mut scene = scene::Scene::new(800, 600);
    scene.proj(deg(100.0_f32), 0.1_f32, 100.0_f32)
        .view(eye, center, up)
        .light(Vector3::new(1.0_f32, 1.0_f32, -1.0_f32))
        .ambient_intensity(1.0_f32);

    while scene.start(0xAAAAAA) {
        angle = angle + add_angle;
        scene.draw(&model, Matrix4::from(Matrix3::from_angle_y(angle)) * init_matrix, &mut shader).present();
    }
}
