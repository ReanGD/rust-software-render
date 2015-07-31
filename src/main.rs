extern crate sdl2;

mod device;
mod vector;
mod rasterization;

use device::Device;
use vector::Vec2;
use rasterization::triangle;


pub fn main() {
    let mut device = Device::new("rust software render", 800, 600);

    while device.keyboard() {
        device.clear(0xFFFFFF);
        // for j in (1..100) {
        //     for i in (1..60) {
        //         triangle(&mut device.cbuffer,
        //                  device.x_size,
        //                  device.y_size,
        //                  Vec2::new(790.0_f32, i as f32 * 10.0_f32),
        //                  Vec2::new( 10.0_f32, i as f32 * 10.0_f32 + 5.0_f32),
        //                  Vec2::new(790.0_f32, i as f32 * 10.0_f32 + 10.0_f32));
        //     }
        //     for i in (1..60) {
        //         triangle(&mut device.cbuffer,
        //                  device.x_size,
        //                  device.y_size,
        //                  Vec2::new( 10.0_f32, i as f32 * 10.0_f32 + 5.0_f32),
        //                  Vec2::new(790.0_f32, i as f32 * 10.0_f32 + 10.0_f32),
        //                  Vec2::new( 10.0_f32, i as f32 * 10.0_f32 + 15.0_f32));
        //     }
        // }
        triangle(&mut device.cbuffer,
                 device.x_size,
                 device.y_size,
                 Vec2::new(100.0_f32, 300.0_f32),
                 Vec2::new(400.0_f32, 100.0_f32),
                 Vec2::new(500.0_f32, 200.0_f32));
        device.draw();
        device.draw_fps();
    }
}
