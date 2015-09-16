extern crate obj;
extern crate genmesh;
extern crate sdl2;
extern crate sdl2_image;
extern crate time;
extern crate cgmath;
extern crate libc;
extern crate sdl2_sys;

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
mod dll_import;
// mod generator;
mod rasterization;

use cgmath::*;
use scene::Scene;
use shader::Shader;
// use generator::{generate_plane, generate_sphere};
use importobj::ModelObj;

pub fn main() {
    let eye;
    let center;
    let model;
    let ind = 3;

    let add_angle = rad(2.0_f32 * std::f32::consts::PI / 180.0_f32);
    let light = Vector3::new(1.0_f32, -1.0_f32, 1.0_f32);
    match ind {
        // 0 => {
        //     eye = Point3::new(0.0_f32, 0.0_f32, -0.5_f32);
        //     center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
        //     add_angle = rad(0.0_f32);
        //     init_matrix = Matrix4::from(Matrix3::from_angle_x(rad(std::f32::consts::PI * 0.25_f32)));
        //     model = generate_plane().unwrap();
        // },
        // 1 => {
        //     eye = Point3::new(0.0_f32, 0.0_f32, -2.0_f32);
        //     center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
        //     model = generate_sphere(50).unwrap();
        // },
        3 => {
            eye = Point3::new(0.0_f32, 0.3_f32, -0.6_f32);
            center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
            // model = ModelObj::load("cube.obj").unwrap();
            model = ModelObj::load(std::path::Path::new("monster1/monster.obj")).unwrap();
            // model = ModelObj::load("nokia/nokia.obj").unwrap();
            // model = ModelObj::load("droid/attack_droid.obj").unwrap();
        },

        _ => return
    };
    let init_matrix = model.to_center_matrix();
    let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);

    let mut angle = rad(std::f32::consts::PI * 0.5_f32);

    let mut scene = Scene::new(800, 600);
    scene.proj(deg(100.0_f32), 0.1_f32, 100.0_f32)
        .view(eye, center, up)
        .light(light)
        .ambient_intensity(1.0_f32);

    let mut shader = Shader::new();
    let sh_ind = 0;
    match sh_ind {
        0 => shader.set_normal(),
        1 => shader.set_lambert(),
        _ => return,
    };

    while scene.start(0xAAAAAA) {
        angle = angle + add_angle;
        scene.draw(&model, Matrix4::from(Matrix3::from_angle_y(angle)) * init_matrix, &mut shader).present();
    }
}
