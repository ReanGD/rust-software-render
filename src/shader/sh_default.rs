use cgmath::{Vector3, Vector4};
use shader::base::*;
use std::ops::Mul;

impl Shader {
    pub fn shader_default() -> ShadersType {
        ([Shader::vertex_default_color, Shader::vertex_default_color],
         [Shader::pixel_default_color, Shader::pixel_default_color])
    }

    // out:
    fn vertex_default_color(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul(self.read_vec4(IN_VS_VEC_POS));

        pos
    }

    // in:
    fn pixel_default_color(&self) -> Vector3<f32> {
        Vector3::new(255.0_f32, 255.0_f32, 255.0_f32)
    }
}
