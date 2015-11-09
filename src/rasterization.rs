use std::mem;
use std::cmp;
use std::boxed::Box;
use cgmath::*;
use shader::{Shader, MAX_OUT_VALUES};
use memory::vector3_to_u32;

const EPSILON: f32 = 0.0001_f32;

#[inline]
fn sort_by_y<'a>(points: &'a[Point3<f32>; 3], vertex_data: &'a[[f32;MAX_OUT_VALUES]; 3]) ->
    (&'a Point3<f32>, &'a Point3<f32>, &'a Point3<f32>, &'a [f32;MAX_OUT_VALUES], &'a [f32;MAX_OUT_VALUES], &'a [f32;MAX_OUT_VALUES]) {
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

        (a, b, c, va, vb, vc)
    }

#[inline]
fn half_triangle(cbuffer: &mut Vec<u32>,
                 zbuffer: &mut Vec<f32>,
                 x_size: usize,
                 shader: &mut Shader,
                 point_base: &Point3<f32>,
                 vd_base: &[f32;MAX_OUT_VALUES],
                 y_begin: usize,
                 y_end: usize,
                 step0: &[f32; MAX_OUT_VALUES + 2],
                 step1: &[f32; MAX_OUT_VALUES + 2]
                 ) {
    let mut vdata0_step = [0.0_f32; MAX_OUT_VALUES];
    let mut vdata0      = [0.0_f32; MAX_OUT_VALUES];
    let mut dvdata_step = [0.0_f32; MAX_OUT_VALUES];
    let mut dvdata      = [0.0_f32; MAX_OUT_VALUES];
    let mut vdata_step  = [0.0_f32; MAX_OUT_VALUES];
    let mut vdata       = [0.0_f32; MAX_OUT_VALUES];

    let y_step = y_begin as f32 + 0.5_f32 - point_base.y;
    let x0_step = step0[0];
    let x1_step = step1[0];
    let z0_step = step0[1];
    let z1_step = step1[1];

    let mut x1 = point_base.x + y_step * x0_step + 0.5_f32 - EPSILON;
    let mut z1 = point_base.z + y_step * z0_step; // inverse z
    let mut x2 = point_base.x + y_step * x1_step + 0.5_f32 - EPSILON;

    let dx_step = x0_step - x1_step;
    let dz_step = z0_step - z1_step;

    let mut dx = y_step * dx_step;
    let mut dz = y_step * dz_step;

    for ind in 0..shader.vertex_out_len {
        vdata0_step[ind] = step0[ind + 2];
        let vdata1_step = step1[ind + 2];

        vdata0[ind] = vd_base[ind] + y_step * vdata0_step[ind];
        dvdata_step[ind] = vdata0_step[ind] - vdata1_step;
        dvdata[ind] = y_step * dvdata_step[ind];
    }

    let pixel_func_base = match shader.texture {
        None => shader.pixel_func[0],
        Some(_) => shader.pixel_func[1],
    };

    let pixel_func: Box<Fn(&Shader) -> Vector3<f32>> = match shader.texture_cube {
        None => Box::new(pixel_func_base),
        Some(_) => Box::new(|shader| shader.pixel_cubemap(pixel_func_base(shader))),
    };

    let mut offset = y_begin * x_size;
    for _ in y_begin..y_end {
        let x1_int = cmp::min(cmp::max(x1 as i32, 0) as usize, x_size - 1);
        let x2_int = cmp::min(cmp::max(x2 as i32, 0) as usize, x_size);
        if x2_int > x1_int {
            let z_step = dz / dx;
            let mut z = z1 + z_step * (x1_int as f32 - x1 - EPSILON); // inverse z
            for ind in 0..shader.vertex_out_len {
                vdata_step[ind] = dvdata[ind] / dx;
                vdata[ind] = vdata0[ind] + vdata_step[ind] * (x1_int as f32 - x1 - EPSILON);
            }

            for x in x1_int..x2_int {
                z += z_step;
                for ind in 0..shader.vertex_out_len {
                    vdata[ind] += vdata_step[ind];
                }
                if zbuffer[offset + x] < z {
                    for ind in 0..shader.vertex_out_len {
                        shader.in_pixel_data[ind] = vdata[ind] / z;
                    }
                    cbuffer[offset + x] = vector3_to_u32(&pixel_func(shader));
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
        for ind in 0..shader.vertex_out_len {
            vdata0[ind] += vdata0_step[ind];
            dvdata[ind] += dvdata_step[ind];
        }
    }
}

#[inline]
pub fn triangle(cbuffer: &mut Vec<u32>,
                zbuffer: &mut Vec<f32>,
                x_size: usize,
                y_size: usize,
                points: [Point3<f32>; 3],
                vertex_data: [[f32;MAX_OUT_VALUES]; 3],
                shader: &mut Shader) {
    // a.y > b.y > c.y
    let (a, b, c, va, vb, vc) = sort_by_y(&points, &vertex_data);

    // is visible
    const MIN_Y: f32 = 0.5_f32;
    const MIN_X: f32 = 0.5_f32;
    let max_y = y_size as f32 - 0.5_f32;
    let max_x = x_size as f32 - 0.5_f32;
    if (a.y < MIN_Y) ||
        (c.y > max_y) ||
        (a.x < MIN_X && b.x < MIN_X && c.x < MIN_X) ||
        (a.x > max_x && b.x > max_x && c.x > max_x) {
            return;
        }

    // steps for line
    let mut step_ab = [0.0_f32; MAX_OUT_VALUES + 2];
    let inv_dy_ab = if a.y - b.y > EPSILON {
        1.0_f32 / (a.y - b.y)
    } else {
        1.0_f32 / EPSILON
    };
    let mut step_ac = [0.0_f32; MAX_OUT_VALUES + 2];
    let inv_dy_ac = if a.y - c.y > EPSILON {
        1.0_f32 / (a.y - c.y)
    } else {
        1.0_f32 / EPSILON
    };
    let mut step_bc = [0.0_f32; MAX_OUT_VALUES + 2];
    let inv_dy_bc = if b.y - c.y > EPSILON {
        1.0_f32 / (b.y - c.y)
    } else {
        1.0_f32 / EPSILON
    };

    step_ab[0] = (a.x - b.x) * inv_dy_ab;
    step_ab[1] = (a.z - b.z) * inv_dy_ab;

    step_ac[0] = (a.x - c.x) * inv_dy_ac;
    step_ac[1] = (a.z - c.z) * inv_dy_ac;

    step_bc[0] = (b.x - c.x) * inv_dy_bc;
    step_bc[1] = (b.z - c.z) * inv_dy_bc;

    for i in 0..shader.vertex_out_len {
        step_ab[i + 2] = (va[i] - vb[i]) * inv_dy_ab;
        step_ac[i + 2] = (va[i] - vc[i]) * inv_dy_ac;
        step_bc[i + 2] = (vb[i] - vc[i]) * inv_dy_bc;
    }

    // y-ranges: [y0; y1) + [y1; y2)
    let y0 = cmp::min(cmp::max((c.y + 0.5_f32) as i32, 0) as usize, y_size - 1);
    let y1 = cmp::min(cmp::max((b.y + 0.5_f32) as i32, 0) as usize, y_size);
    let y2 = cmp::min(cmp::max((a.y + 0.5_f32) as i32, 0) as usize, y_size);

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

    if y0 < y1 {
        half_triangle(cbuffer,
                      zbuffer,
                      x_size,
                      shader,
                      c, vc, y0, y1,
                      step0.0,
                      step0.1
                      );
    }
    if y1 < y2 {
        half_triangle(cbuffer,
                      zbuffer,
                      x_size,
                      shader,
                      a, va, y1, y2,
                      step1.0,
                      step1.1
                      );
    }
}
