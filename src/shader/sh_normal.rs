use cgmath::{Vector, EuclideanVector, Vector3, Vector4, Matrix};
use shader::base::*;


impl Shader {
    pub fn shader_normal() -> ShadersType {
        ([Shader::vertex_normal_color, Shader::vertex_normal_color],
         [Shader::pixel_normal_color, Shader::pixel_normal_color])
    }

    // out:
    // 0 - Vector3 normal
    fn vertex_normal_color(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();

        self.out_vec3_from4(&norm);
        pos
    }

    // in:
    // 0 - Vector3 normal
    fn pixel_normal_color(&self) -> Vector3<f32> {
        let color = Vector3::new(self.in_pixel_data[0],
                                 self.in_pixel_data[1],
                                 self.in_pixel_data[2]).add_s(1.0_f32).mul_s(128.0_f32);

        color
    }
}
