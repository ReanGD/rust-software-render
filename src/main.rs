extern crate rand;
extern crate sdl2;
extern crate cgmath;

mod device;
mod rasterization;
mod tests;

use cgmath::Point2;
use rand::distributions::{IndependentSample, Range};

use device::Device;
use rasterization::triangle;

pub fn main() {
    let mut device = Device::new("rust software render", 800, 600);

    let color_range = Range::new(0, std::u32::MAX);
    let x_range = Range::new(-50f32, device.x_size as f32 + 50f32);
    let y_range = Range::new(-50f32, device.y_size as f32 + 50f32);
    let mut rng = rand::thread_rng();

    let cnt = 16000;
    let mut points: Vec<Point2<f32>> = vec![];
    let mut colors: Vec<u32> = vec![];
    for _ in 0..cnt {
        points.push(Point2::new(x_range.ind_sample(&mut rng), y_range.ind_sample(&mut rng)));
        points.push(Point2::new(x_range.ind_sample(&mut rng), y_range.ind_sample(&mut rng)));
        points.push(Point2::new(x_range.ind_sample(&mut rng), y_range.ind_sample(&mut rng)));
        colors.push(color_range.ind_sample(&mut rng));
    }
    
    while device.keyboard() {
        device.clear(0xFFFFFF);
        for i in 0..cnt {
            triangle(&mut device.cbuffer,
                     device.x_size,
                     device.y_size,
                     points[i*3 + 0], points[i*3 + 1], points[i*3 + 2], colors[i]);
        }
        device.present();
        device.draw_fps();
    }
}
