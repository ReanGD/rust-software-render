#[cfg(test)]
mod rasterization {
    use cgmath::Point2;
    use rasterization::triangle;

    fn triangle_test(a: Point2<f32>, b: Point2<f32>, c: Point2<f32>, buffer_except: Vec<u32>) {
        let x_size: usize = 7;
        let y_size: usize = 5;
        let mut buffer: Vec<u32> = vec![0; x_size * y_size];
        triangle(&mut buffer, x_size, y_size, a, b, c, 1);
        
        println!("");
        println!("real: ");
        for y in (0..y_size) {
            print!("   ");
            for x in (0..x_size) {
                print!("{}",if buffer[(y_size - y - 1) * x_size + x]==0 {0} else {1});
            }
            println!("");
        }
        println!("except: ");
        for y in (0..y_size) {
            print!("   ");
            for x in (0..x_size) {
                print!("{}",if buffer_except[y * x_size + x]==0 {0} else {1});
            }
            println!("");
        }
        for y in 0..y_size {
            for x in 0..x_size {
                let val_real = buffer[y * x_size + x];
                let val_except = buffer_except[(y_size - y - 1) * x_size + x];
                debug_assert!((val_real==0) == (val_except==0),
                              "real = {}, except = {}, x = {} y = {}",
                              val_real, val_except, x, y);
            }
        }
    }

    #[test]
    fn triangle_00() {
        triangle_test(Point2::new(1.0,4.0), Point2::new(2.0,1.0), Point2::new(6.0,3.0),
                      vec![0,0,0,0,0,0,0,
                           0,1,1,0,0,0,0,
                           0,1,1,1,1,0,0,
                           0,0,1,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_01() {
        triangle_test(Point2::new(1.4,1.6), Point2::new(2.4,1.6), Point2::new(2.4,2.6),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_02() {
        triangle_test(Point2::new(3.5,1.5), Point2::new(4.5,1.5), Point2::new(4.5,2.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }
    
    #[test]
    fn triangle_03() {
        triangle_test(Point2::new(2.5,2.5), Point2::new(2.5,2.5), Point2::new(2.5,2.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }
    
    #[test]
    fn triangle_04() {
        triangle_test(Point2::new(0.0,0.0), Point2::new(4.0,0.0), Point2::new(6.0,2.0),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,1,0,0,
                           0,1,1,1,0,0,0]);
    }
    
    #[test]
    fn triangle_05() {
        triangle_test(Point2::new(1.0,2.0), Point2::new(4.0,1.0), Point2::new(3.0,4.0),
                      vec![0,0,0,0,0,0,0,
                           0,0,1,0,0,0,0,
                           0,1,1,0,0,0,0,
                           0,0,1,1,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_06() {
        triangle_test(Point2::new(5.5,2.5), Point2::new(4.0,1.0), Point2::new(3.0,4.0),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,1,0,0,0,
                           0,0,0,1,1,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_07() {
        triangle_test(Point2::new(1.5,0.5), Point2::new(2.5,0.5), Point2::new(1.5,-2.0),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,1,0,0,0,0,0]);
    }

    #[test]
    fn triangle_08() {
        triangle_test(Point2::new(0.7,3.5), Point2::new(2.5,0.7), Point2::new(4.6,3.5),
                      vec![0,0,0,0,0,0,0,
                           0,1,1,1,1,0,0,
                           0,1,1,1,0,0,0,
                           0,0,1,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_09() {
        triangle_test(Point2::new(0.7,1.5), Point2::new(2.7,3.2), Point2::new(4.7,1.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,1,1,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_10() {
        triangle_test(Point2::new(0.5,0.5), Point2::new(0.5,2.5), Point2::new(1.5,1.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           1,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_11() {
        triangle_test(Point2::new(0.5,0.5), Point2::new(0.5,2.5), Point2::new(2.5,2.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           1,1,0,0,0,0,0,
                           1,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_12() {
        triangle_test(Point2::new(0.5,0.5), Point2::new(2.5,2.5), Point2::new(2.5,0.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,1,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_13() {
        triangle_test(Point2::new(0.5,3.5), Point2::new(1.5,2.5), Point2::new(1.5,0.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_14() {
        triangle_test(Point2::new(0.5,3.5), Point2::new(1.5,2.5), Point2::new(2.0,5.0),
                      vec![0,1,0,0,0,0,0,
                           1,1,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_15() {
        triangle_test(Point2::new(0.5,0.5), Point2::new(2.5,0.5), Point2::new(0.5,2.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           1,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_16() {
        triangle_test(Point2::new(2.5,2.5), Point2::new(2.5,0.5), Point2::new(0.5,2.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           1,1,0,0,0,0,0,
                           0,1,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_17() {
        triangle_test(Point2::new(5.0,2.0), Point2::new(5.0,-2.0), Point2::new(10.0,2.0),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,1,1,
                           0,0,0,0,0,1,1]);
    }

    #[test]
    fn triangle_18() {
        triangle_test(Point2::new(5.0,3.0), Point2::new(5.0,10.0), Point2::new(10.0,3.0),
                      vec![0,0,0,0,0,1,1,
                           0,0,0,0,0,1,1,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }
}
