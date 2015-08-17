use std::mem;
use std::cmp;
use cgmath::*;
use shader::{Shader, MAX_OUT_VALUES};

// 598/1195/1206
pub fn triangle(cbuffer: &mut Vec<u32>,
                zbuffer: &mut Vec<f32>,
                x_size: usize,
                y_size: usize,
                points: [Point3<f32>; 3],
                vertex_data: [[f32;MAX_OUT_VALUES]; 3],
                vertex_data_cnt: usize,
                shader: &mut Shader) {
    let mut a = &points[0];
    let mut b = &points[1];
    let mut c = &points[2];
    let mut va = &vertex_data[0];
    let mut vb = &vertex_data[1];
    let mut vc = &vertex_data[2];

    // a.y > b.y > c.y
    if a.y < b.y {
        mem::swap(&mut a, &mut b);
        mem::swap(&mut va, &mut vb);
    }
    if a.y < c.y {
        mem::swap(&mut a, &mut c);
        mem::swap(&mut va, &mut vc);
    }
    if b.y < c.y {
        mem::swap(&mut b, &mut c);
        mem::swap(&mut vb, &mut vc);
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

    let mut step_ab = [0.0_f32; MAX_OUT_VALUES + 2];
    let inv_dy_ab = if a.y - b.y > epsilon {
        1.0_f32 / (a.y - b.y)
    } else {
        1.0_f32 / epsilon
    };
    let mut step_ac = [0.0_f32; MAX_OUT_VALUES + 2];
    let inv_dy_ac = if a.y - c.y > epsilon {
        1.0_f32 / (a.y - c.y)
    } else {
        1.0_f32 / epsilon
    };
    let mut step_bc = [0.0_f32; MAX_OUT_VALUES + 2];
    let inv_dy_bc = if b.y - c.y > epsilon {
        1.0_f32 / (b.y - c.y)
    } else {
        1.0_f32 / epsilon
    };

    step_ab[0] = (a.x - b.x) * inv_dy_ab;
    step_ab[1] = (a.z - b.z) * inv_dy_ab;

    step_ac[0] = (a.x - c.x) * inv_dy_ac;
    step_ac[1] = (a.z - c.z) * inv_dy_ac;

    step_bc[0] = (b.x - c.x) * inv_dy_bc;
    step_bc[1] = (b.z - c.z) * inv_dy_bc;

    for i in 0..vertex_data_cnt {
        step_ab[i + 2] = (va[i] - vb[i]) * inv_dy_ab;
        step_ac[i + 2] = (va[i] - vc[i]) * inv_dy_ac;
        step_bc[i + 2] = (vb[i] - vc[i]) * inv_dy_bc;
    }
    

    // y-ranges: [y0; y1) + [y1; y2)
    let y0 = cmp::min(cmp::max((c.y + 0.5_f32) as i32, 0) as usize, y_size - 1);
    let y1 = cmp::min(cmp::max((b.y + 0.5_f32) as i32, 0) as usize, y_size);
    let y2 = cmp::min(cmp::max((a.y + 0.5_f32) as i32, 0) as usize, y_size);
    let point_base = [c, a];
    let vd_base = [vc, va];
    let y_begin = [y0, y1];
    let y_end = [y1, y2];

    let step0 = if step_bc[0] > step_ac[0] {
        (&step_ac, &step_bc)
    } else {
        (&step_bc, &step_ac)
    };
    let step1 = if step_ab[0] > step_ac[0] {
        (&step_ab, &step_ac)
    } else {
        (&step_ac, &step_ab)
    };
    let steps = [step0, step1];
    let pixel_func = shader.pixel_func;

    
    let mut vdata0_step = [0.0_f32; MAX_OUT_VALUES];
    let mut vdata0      = [0.0_f32; MAX_OUT_VALUES];
    let mut dvdata_step = [0.0_f32; MAX_OUT_VALUES];
    let mut dvdata      = [0.0_f32; MAX_OUT_VALUES];
    let mut vdata_step  = [0.0_f32; MAX_OUT_VALUES];
    let mut vdata       = [0.0_f32; MAX_OUT_VALUES];

    for i in (0..2) {
        if y_begin[i] < y_end[i] {
            let y_step = y_begin[i] as f32 + 0.5_f32 - point_base[i].y;
            let x0_step = steps[i].0[0];
            let x1_step = steps[i].1[0];
            let z0_step = steps[i].0[1];
            let z1_step = steps[i].1[1];

            let mut x1 = point_base[i].x + y_step * x0_step + 0.5_f32 - epsilon;
            let mut z1 = point_base[i].z + y_step * z0_step; // inverse z
            let mut x2 = point_base[i].x + y_step * x1_step + 0.5_f32 - epsilon;

            let dx_step = x0_step - x1_step;
            let dz_step = z0_step - z1_step;

            let mut dx = y_step * dx_step;
            let mut dz = y_step * dz_step;

            for ind in 0..vertex_data_cnt {
                vdata0_step[ind] = steps[i].0[ind + 2];
                let vdata1_step = steps[i].1[ind + 2];

                vdata0[ind] = vd_base[i][ind] + y_step * vdata0_step[ind];
                dvdata_step[ind] = vdata0_step[ind] - vdata1_step;
                dvdata[ind] = y_step * dvdata_step[ind];
            }

            let mut offset = y_begin[i] * x_size;
            for _ in y_begin[i]..y_end[i] {
                let x1_int = cmp::min(cmp::max(x1 as i32, 0) as usize, x_size - 1);
                let x2_int = cmp::min(cmp::max(x2 as i32, 0) as usize, x_size);
                if x2_int > x1_int {
                    let z_step = dz / dx;
                    let mut z = z1 + z_step * (x1_int as f32 - x1 - epsilon); // inverse z
                    for ind in 0..vertex_data_cnt {
                        vdata_step[ind] = dvdata[ind] / dx;
                        vdata[ind] = vdata0[ind] + vdata_step[ind] * (x1_int as f32 - x1 - epsilon);
                    }

                    for x in x1_int..x2_int {
                        z += z_step;
                        for ind in 0..vertex_data_cnt {
                            vdata[ind] = vdata[ind] + vdata_step[ind];
                        }
                        if zbuffer[offset + x] < z {
                            for ind in 0..vertex_data_cnt {
                                shader.in_pixel_data[ind] = vdata[ind] / z;
                            }
                            let clr = pixel_func(shader);
                            cbuffer[offset + x] = ((cmp::min(cmp::max((clr.x as i32), 0), 0xFF) << 16) +
                                          (cmp::min(cmp::max((clr.y as i32), 0), 0xFF) << 8) +
                                          cmp::min(cmp::max((clr.z as i32), 0), 0xFF)) as u32;
                            zbuffer[offset + x] = z;
                        }
                    }
                }
                offset += x_size;
                x1 += x0_step;
                x2 += x1_step;
                dx += dx_step;
                z1 += z0_step;
                dz += dz_step;
                for ind in 0..vertex_data_cnt {
                    vdata0[ind] = vdata0[ind] + vdata0_step[ind];
                    dvdata[ind] = dvdata[ind] + dvdata_step[ind];
                }
            }
        }
    }
}
