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
use std::ops::Mul;

pub fn main() {
    let eye;
    let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);
    let center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
    let mut model;
    let init_matrix;
    let shader_type = shader::ShaderType::CookTorrance;
    let model_index = 4;

    let mut angle = rad(0.0_f32);
    let add_angle;
    let cube_map_path = utils::get_base_dir().unwrap().join("cubemap/");
    match model_index {
        0 => {
            eye = Point3::new(0.0_f32, 0.0_f32, -0.5_f32);
            add_angle = rad(0.0_f32);
            init_matrix = Matrix4::from(Matrix3::from_angle_x(rad(std::f32::consts::PI * 0.25_f32)));
            let mut mat = material::Material::new();
            mat.create_texture(utils::get_base_dir().unwrap().join("lego.png").as_path()).unwrap();
            model = generator::generate_plane(mat).unwrap();
            model.add_texture_cube(&cube_map_path, "jpg").unwrap();
        },
        1 => {
            eye = Point3::new(0.0_f32, 1.1_f32, -1.1_f32);
            add_angle = rad(0.0_f32);
            let mat = material::Material::new();
            model = generator::generate_sphere(60, mat).unwrap();
            model.add_texture_cube(&cube_map_path, "jpg").unwrap();
            init_matrix = model.to_center_matrix();
        },
        2 => {
            eye = Point3::new(0.0_f32, 0.3_f32, 0.6_f32);
            add_angle = rad(0.01_f32);
            model = ModelObj::load(std::path::Path::new("monster/monster.obj")).unwrap();
            model.add_texture_cube(&cube_map_path, "jpg").unwrap();
            init_matrix = model.to_center_matrix();
        },
        3 => {
            eye = Point3::new(0.0_f32, 0.25_f32, 0.75_f32);
            add_angle = rad(0.01_f32);
            model = ModelObj::load(std::path::Path::new("ring/ring.obj")).unwrap();
            model.add_texture_cube(&cube_map_path, "jpg").unwrap();
            init_matrix = model.to_center_matrix();
        },
        4 => {
            eye = Point3::new(0.0_f32, 0.2_f32, 0.8_f32);
            add_angle = rad(0.01_f32);
            model = ModelObj::load(std::path::Path::new("skull/skull.obj")).unwrap();
            model.add_texture_cube(&cube_map_path, "jpg").unwrap();
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
        scene.draw(&model, Matrix4::from(Matrix3::from_angle_y(angle)).mul(&init_matrix), &mut shader).present();
    }
}
