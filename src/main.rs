extern crate sdl2;
extern crate cgmath;

mod device;
mod rasterization;
mod tests;

use device::Device;
use cgmath::Point2;
use rasterization::triangle;


pub fn main() {
    let mut device = Device::new("rust software render", 800, 600);

    while device.keyboard() {
        device.clear(0xFFFFFF);
        triangle(&mut device.cbuffer,
                 device.x_size,
                 device.y_size,
                 Point2::new(100.0_f32, 300.0_f32),
                 Point2::new(400.0_f32, 100.0_f32),
                 Point2::new(500.0_f32, 200.0_f32));
        device.present();
        device.draw_fps();
    }
}
