extern crate sdl2;
extern crate rand;
extern crate time;
extern crate cgmath;

mod device;
mod mesh;
mod mesh3ds;
mod rasterization;
mod tests;

use cgmath::*;

use device::Device;
use mesh3ds::Loader3ds;


pub fn main() {    
    let mut device = Device::new("rust software render", 800, 600);

    let eye = Point3::new(0.0_f32, 0.0_f32, -1.1_f32);
	let center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
    // let eye = Point3::new(0.0_f32, 3.0_f32, -7.1_f32);
	// let center = Point3::new(0.0_f32, 3.0_f32, 0.0_f32);
	let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);
    let fovy = deg(100.0_f32);
    let aspect = 800.0_f32/600.0_f32;
    let near = 0.1_f32;
    let far = 100.0_f32;

    let mat_view = Matrix4::<f32>::look_at(&eye, &center, &up);
    let mat_proj = perspective(fovy, aspect, near, far);

    let mut angle = rad(0.0_f32);
    let add_angle = rad(2.0_f32 * std::f32::consts::PI / 180.0_f32);

    let path = "../media/ring.3ds";
    // let path = "../media/tux.3ds";
    // let path = "../media/yoda.3ds";
    // let path = "../media/cube.3ds";
    // let path = "../media/model.3ds";
    let mesh = Loader3ds::load(&path).unwrap();
    while device.keyboard() {
        device.clear(0xFFFFFF);

        angle = angle + add_angle;
        let mat_world = Matrix4::from(Matrix3::from_angle_y(angle));
        let mat_proj_view_world = mat_proj * mat_view * mat_world;
        let cnt_triangle = mesh.draw(&mat_proj_view_world, &mat_world, &mut device);
        device.present();
        device.update_fps(cnt_triangle);
    }
}
