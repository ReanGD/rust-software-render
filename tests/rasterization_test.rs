// extern crate vector;
// extern crate rasterization;
// extern crate rust-software-render;

// #[cfg(test)]
// mod vector;

// use vector::Vec2;
// use rasterization::triangle;

// #[test]
// fn it_work() {

//     let x_size: usize = 7;
//     let y_size: usize = 5;
//     let mut buffer: Vec<u32> = vec![0; x_size * y_size];
//     triangle(&mut buffer, x_size, y_size, Vec2(1,4), Vec2(2,1), Vec2(3,6));
//     let mut buffer_except = vec![
//         0,0,0,0,0,0,0,0,
//         0,0,1,0,0,0,0,0,
//         0,1,1,1,1,0,0,0,
//         0,1,1,0,0,0,0,0];
//     for i in (0..x_size * y_size) {
//         assert_eq!(buffer[i]==0,buffer_except[i]==0);
//     }
// }
