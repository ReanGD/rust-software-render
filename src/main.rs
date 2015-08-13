extern crate sdl2;
extern crate rand;
extern crate time;
extern crate cgmath;

mod mesh;
mod scene;
mod tests;
mod device;
mod memory;
mod shader;
mod import3ds;
mod rasterization;

use cgmath::*;
use scene::Scene;
use shader::Shader;
use import3ds::Loader3ds;


pub fn main() {    
    let eye = Point3::new(0.0_f32, 0.0_f32, -1.1_f32);
	let center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
    // let eye = Point3::new(0.0_f32, -2.0_f32, -4.1_f32);
	// let center = Point3::new(0.0_f32, -2.0_f32, 0.0_f32);
	let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);

    let mut angle = rad(0.0_f32);
    let add_angle = rad(2.0_f32 * std::f32::consts::PI / 180.0_f32);

    let mut scene = Scene::new(800, 600);
    scene.proj(deg(100.0_f32), 0.1_f32, 100.0_f32)
        .view(eye, center, up)
        .light(Vector3::new(0.0_f32, 1.0_f32, -1.0_f32));

    let path = "ring.3ds";
    // let path = "monster.3ds";
    // let path = "tux.3ds";
    // let path = "yoda.3ds";
    // let path = "cube.3ds";
    // let path = "model.3ds";

    let model = Loader3ds::load(&path).unwrap();
    let ambient = Vector3::<f32>::new(0xff as f32, 0xd7 as f32, 0x00 as f32);
    let diffuse = Vector3::<f32>::new(0xff as f32, 0xd7 as f32, 0x00 as f32);
    // let ambient = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
    // let diffuse = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
    let mut shader = Shader::new(ambient, diffuse, 0.2_f32, 0.0_f32);
    // let to_begin = Matrix4::from(Matrix3::from_angle_x(rad(-1.8_f32)));
    while scene.start(0xFFFFFF) {
        angle = angle + add_angle;
        scene.draw(&model, Matrix4::from(Matrix3::from_angle_y(angle)), &mut shader).present();
        // scene.draw(&model, Matrix4::from(Matrix3::from_angle_y(angle)) * to_begin, &mut shader).present();
    }
}
