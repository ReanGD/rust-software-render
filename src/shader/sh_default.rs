use cgmath::{Vector3, Vector4, Matrix};
use shader::base::*;

impl Shader {
    // out:
    pub fn vertex_default(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));

        pos
    }

    // in:
    pub fn pixel_default(&self) -> Vector3<f32> {
        Vector3::new(255.0_f32, 255.0_f32, 255.0_f32)
    }
}
