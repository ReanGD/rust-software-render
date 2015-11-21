use cgmath::{Vector, Vector2, Vector3, EuclideanVector, Matrix};
use shader::base::*;

impl Shader {
    // out:
    // 0 - Vector3 reflection
    pub fn vertex_cubemap(&mut self) {
        let eye = self.matrix_arr[MATRIX_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS)).normalize();
        let norm = self.matrix_arr[MATRIX_VIEW_WORLD]
            .mul_v(&self.read_vec4(IN_VS_VEC_NORM).normalize()).normalize();
        let reflection = eye.sub_v(&norm.mul_s(norm.dot(&eye) * 2.0_f32));

        self.out_vec3_from4(&reflection)
    }

    // in:
    // 0 - Vector3 reflection
    pub fn pixel_cubemap(&self, base_color: Vector3<f32>) -> Vector3<f32> {
        let lod = 0;
        let offset = self.vertex_out2_base;
        let reflection = Vector3::new(self.in_pixel_data[offset + 0],
                                      self.in_pixel_data[offset + 1],
                                      self.in_pixel_data[offset + 2]).normalize();

        let irx = reflection.x;
        let iry = reflection.y;
        let irz = reflection.z;
        let urx = irx.abs();
        let ury = iry.abs();
        let urz = irz.abs();

        let (index, sc, tc, ma) =
            // x plane
            if (urx > urz) && (urx > ury) {
                if irx.is_sign_positive() {
                    (0, -irz,  iry, urx)
                } else {
                    (1,  irz,  iry, urx)
                }
                // y plane
            } else if ury > urz {
                if iry > 0.0_f32 {
                    (2,  irx, -irz, ury)
                } else {
                    (3,  irx,  irz, ury)
                }
                // z plane
            } else {
                if irz.is_sign_positive() {
                    (4,  irx,  iry, urz)
                } else {
                    (5, -irx,  iry, urz)
                }
            };

        let coord = Vector2::new(( sc/ma + 1.0_f32 ) / 2.0_f32,
                                 ( tc/ma + 1.0_f32 ) / 2.0_f32);

        match self.texture_cube {
            Some(ref t) => t
                .get_texture(index)
                .get_surface(lod)
                .tex_2d_bilinear(coord)
                .lerp(&base_color, 0.3_f32),
            None => panic!("texture_cube is not set"),
        }
    }
}
