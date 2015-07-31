use std::mem;
use std::cmp;
use vector::Vec2;

pub fn triangle(buffer: &mut Vec<u32>, x_size: usize, y_size: usize, p_a: Vec2, p_b: Vec2, p_c: Vec2) {
    let mut a = &p_a;
    let mut b = &p_b;
    let mut c = &p_c;

    // a.y > b.y > c.y
    if a.y < b.y {
        mem::swap(&mut a, &mut b);
    }
    if a.y < c.y {
        mem::swap(&mut a, &mut c);
    }
    if b.y < c.y {
        mem::swap(&mut b, &mut c);
    }

    // is visible
    let min_y = 0.5_f32;
    let min_x = 0.5_f32;
    let max_y = y_size as f32 - 0.5_f32;
    let max_x = x_size as f32 - 0.5_f32;
    if (a.y < min_y) ||
        (c.y > max_y) ||
        (a.x < min_x && b.x < min_x && c.x < min_x) ||
        (a.x > max_x && b.x > max_x && c.x > max_x) {
            return;
        }
    
    // steps for line
    let epsilon = 0.0001_f32;
    let step_ab = if a.y - b.y > epsilon {
        (a.x - b.x) / (a.y - b.y)
    } else {
        0.0_f32
    };
    let step_ac = if a.y - c.y > epsilon {
        (a.x - c.x) / (a.y - c.y)
    } else {
        0.0_f32
    };
    let step_bc = if b.y - c.y > epsilon {
        (b.x - c.x) / (b.y - c.y)
    } else {
        0.0_f32
    };

    // y-ranges: [y0; y1) + [y1; y2)
    let y0 = cmp::min(cmp::max((c.y + 0.5_f32) as i32, 0) as usize, y_size - 1);
    let y1 = cmp::min(cmp::max((b.y + 0.5_f32) as i32, 0) as usize, y_size);
    let y2 = cmp::min(cmp::max((a.y + 0.5_f32) as i32, 0) as usize, y_size);
    let point_base = [c, a];
    let y_begin = [y0, y1];
    let y_end = [y1, y2];

    let step0 = if step_bc > step_ac {
        (step_ac, step_bc)
    } else {
        (step_bc, step_ac)
    };
    let step1 = if step_ab > step_ac {
        (step_ab, step_ac)
    } else {
        (step_ac, step_ab)
    };
    let steps = [step0, step1];

    for i in (0..2) {
        if y_begin[i] < y_end[i] {
            let y_step = y_begin[i] as f32 + 0.5_f32 - point_base[i].y;
            let (x1_step, x2_step) = steps[i];
            let mut x1 = point_base[i].x + y_step * x1_step + (0.5_f32 - epsilon);
            let mut x2 = point_base[i].x + y_step * x2_step - (0.5_f32 + epsilon);

            println!("");
            println!("  step: x={}-{};", x1_step, x2_step);
            for y in y_begin[i]..y_end[i] {
                let x1_int = cmp::min(cmp::max(x1 as i32, 0) as usize, x_size);
                let x2_int = cmp::min(cmp::max(x2 as i32, 0) as usize, x_size) + 1;
                println!("{}: y={}; x={}-{};", i, y, x1_int, x2_int);
                println!("  real: x={}-{};", x1, x2);
                for x in (x1_int..x2_int) {
                    buffer[y * x_size + x] = 0x00FF00;
                }
                
                x1 += x1_step;
                x2 += x2_step;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use vector::Vec2;
    use rasterization::triangle;

    fn triangle_test(a: Vec2, b: Vec2, c: Vec2, buffer_except: Vec<u32>) {
        let x_size: usize = 7;
        let y_size: usize = 5;
        let mut buffer: Vec<u32> = vec![0; x_size * y_size];
        triangle(&mut buffer, x_size, y_size, a, b, c);
        
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
    fn triangle_0() {
        triangle_test(Vec2::new(1.0,4.0), Vec2::new(2.0,1.0), Vec2::new(6.0,3.0),
                      vec![0,0,0,0,0,0,0,
                           0,1,1,0,0,0,0,
                           0,1,1,1,1,0,0,
                           0,0,1,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_1() {
        triangle_test(Vec2::new(1.4,1.6), Vec2::new(2.4,1.6), Vec2::new(2.4,2.6),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }

    #[test]
    fn triangle_2() {
        triangle_test(Vec2::new(3.5,1.5), Vec2::new(4.5,1.5), Vec2::new(4.5,2.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }
    
    #[test]
    fn triangle_3() {
        triangle_test(Vec2::new(2.5,2.5), Vec2::new(2.5,2.5), Vec2::new(2.5,2.5),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0]);
    }
    
    #[test]
    fn triangle_4() {
        triangle_test(Vec2::new(0.0,0.0), Vec2::new(4.0,0.0), Vec2::new(6.0,2.0),
                      vec![0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,0,0,0,
                           0,0,0,0,1,0,0,
                           0,1,1,1,0,0,0]);
    }
}
