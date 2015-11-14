extern crate sdl2;
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
    let eye;
    let center;
    let path;
    let color;
    let ind = 0;
    let mut init_matrix = Matrix4::<f32>::one();
    match ind {
        0 => {
            eye = Point3::new(0.0_f32, 0.0_f32, -1.1_f32);
	        center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
            color = Vector3::<f32>::new(0xff as f32, 0xd7 as f32, 0x00 as f32);
            path = "ring.3ds";
        },
        1 => {
            eye = Point3::new(0.0_f32, 0.0_f32, -10.1_f32);
	        center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
            color = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            path = "tux.3ds";
        },
        2 => {
            eye = Point3::new(0.0_f32, -2.0_f32, -4.1_f32);
	        center = Point3::new(0.0_f32, -2.0_f32, 0.0_f32);
            color = Vector3::<f32>::new(0xbb as f32, 0xbb as f32, 0xbb as f32);
            init_matrix = Matrix4::from(Matrix3::from_angle_x(rad(-1.8_f32)));
            path = "monster.3ds";
        },
        _ => return
    };
	let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);

    let mut angle = rad(0.0_f32);
    let add_angle = rad(2.0_f32 * std::f32::consts::PI / 180.0_f32);

    let mut scene = Scene::new(800, 600);
    scene.proj(deg(100.0_f32), 0.1_f32, 100.0_f32)
        .view(eye, center, up)
        .light(Vector3::new(0.0_f32, 1.0_f32, -1.0_f32));

    let model = Loader3ds::load(&path).unwrap();
    let mut shader = Shader::new(color, 0.3_f32);
    while scene.start(0xFFFFFF) {
        angle = angle + add_angle;
        scene.draw(&model, Matrix4::from(Matrix3::from_angle_y(angle)).mul_m(&init_matrix), &mut shader).present();
    }
}
