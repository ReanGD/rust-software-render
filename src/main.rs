extern crate sdl2;
extern crate rand;
extern crate time;
extern crate cgmath;

mod device;
mod mesh;
mod rasterization;
mod tests;

use cgmath::*;

use device::Device;
use mesh::generate_cube;

// fn typed_to_bytes<T>(slice: &[T]) -> &[u8] {
//     unsafe {
//         std::slice::from_raw_parts(slice.as_ptr() as *const u8,
//                                    slice.len() * std::mem::size_of::<T>())
//     }
// }

// fn bytes_to_typed<T>(slice: &mut [u8]) -> &mut [T] {
//     unsafe {
//     std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T,
//                                    slice.len() / std::mem::size_of::<T>())
//             }
// }
    // let a: Vec<f32> = vec![0.55_f32];
    // for it in &a {
    //     println!("{}", it);
    // }

    // let b = typed_to_bytes(&a);
    // for it in b {
    //     println!("0x{:x}",it);
    // }

    // let mut aa: Vec<u8> = vec![0xcd, 0xcc, 0x0c, 0x3f];
    // let c = bytes_to_typed::<f32>(&mut aa);
    // for it in c {
    //     println!("{}",*it);
    // }

pub fn main() {    
    let mut device = Device::new("rust software render", 800, 600);

    let eye = Point3::new(0.0_f32, 0.0_f32, -2.0_f32);
	let center = Point3::new(0.0_f32, 0.0_f32, 0.0_f32);
	let up = Vector3::new(0.0_f32, 1.0_f32, 0.0_f32);
    let fovy = deg(130.0_f32);
    let aspect = 800.0_f32/600.0_f32;
    let near = 0.1_f32;
    let far = 100.0_f32;

    let mat_view = Matrix4::<f32>::look_at(&eye, &center, &up);
    let mat_proj = perspective(fovy, aspect, near, far);

    let mut angle = rad(0.0_f32);
    let add_angle = rad(2.0_f32 * std::f32::consts::PI / 180.0_f32);

    let mesh = generate_cube();
    while device.keyboard() {
        device.clear(0xFFFFFF);

        angle = angle + add_angle;
        let mat_world = Matrix4::from(Matrix3::from_angle_y(angle));
        let mat = mat_proj * mat_view * mat_world;
        mesh.draw(&mat, &mut device);
        device.present();
        device.draw_fps();
    }
}
