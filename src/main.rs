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
    let ind = 7;
    // let mut material = Material::gold();

    let init_matrix = Matrix4::<f32>::identity();
    let mut add_angle = rad(2.0_f32 * std::f32::consts::PI / 180.0_f32);
    // let add_angle = rad(0.0_f32);
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
        // 2 => {
        //     eye = Point3::new(0.0_f32, 0.7_f32, -1.1_f32);
	    //     center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
        //     // material = Material::silver();
        //     add_angle = rad(0.0_f32);
        //     model = Model3ds::load("ring.3ds").unwrap();
        // },
        // 3 => {
        //     eye = Point3::new(0.0_f32, 0.0_f32, -10.1_f32);
	    //     center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
        //     model = Model3ds::load("tux.3ds").unwrap();
        // },
        // 4 => {
        //     eye = Point3::new(0.0_f32, -2.0_f32, -4.1_f32);
	    //     center = Point3::new(0.0_f32, -2.0_f32, 0.0_f32);
        //     // material = Material::monster_skin();
        //     init_matrix = Matrix4::from(Matrix3::from_angle_x(rad(-1.8_f32)));
        //     model = Model3ds::load("monster.3ds").unwrap();
        // },
        // 5 => {
        //     eye = Point3::new(0.0_f32, 0.0_f32, -1.1_f32);
	    //     center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
        //     model = Model3ds::load("yoda/yoda2.3ds").unwrap();
        // },
        // 6 => {
        //     eye = Point3::new(0.0_f32, 0.0_f32, -1.1_f32);
	    //     center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
        //     model = Model3ds::load("nokia/nokia-n82-highres.3DS").unwrap();
        // },
        7 => {
            eye = Point3::new(0.0_f32, 40.0_f32, -50.5_f32);
	        center = Point3::new(0.0_f32, 2.0_f32, 0.0_f32);
            // model = ModelObj::load("cube.obj").unwrap();
            // model = ModelObj::load("monster1/monster.obj").unwrap();
            // model = ModelObj::load("nokia/nokia.obj").unwrap();
            model = ModelObj::load("droid/attack_droid.obj").unwrap();
        },


        _ => return
    };
	let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);

    let mut angle = rad(std::f32::consts::PI * 0.5_f32);

    let mut scene = Scene::new(800, 600);
    scene.proj(deg(100.0_f32), 0.1_f32, 100.0_f32)
        .view(eye, center, up)
        .light(light);

    // let mut texture = Texture::new();
    // texture.load_from_def("gamma_dalai_lama_gray.jpg").unwrap();
    // texture.load_from_def("green_monster.png").unwrap();
    // texture.load_from_def("chess.png").unwrap();

    let mut shader = Shader::new();
    shader.set_lambert();
    // shader.set_shaders(Shader::vertex_cook_torrance, Shader::pixel_cook_torrance);
    // shader.set_shaders(Shader::vertex_phong_blinn, Shader::pixel_phong_blinn);
    // shader.set_shaders(Shader::vertex_normals, Shader::pixel_normals);
    while scene.start(0xAAAAAA) {
        angle = angle + add_angle;
        scene.draw(&model, Matrix4::from(Matrix3::from_angle_y(angle)) * init_matrix, &mut shader).present();
    }
}
