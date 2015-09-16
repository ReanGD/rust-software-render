use cgmath::{Vector, EuclideanVector, Vector3, Vector4, Matrix};
use shader::base::*;


impl Shader {
    pub fn set_normal(&mut self) {
        self.vertex_func = [Shader::vertex_normal, Shader::vertex_normal];
        self.pixel_func = [Shader::pixel_normal, Shader::pixel_normal];
    }

    // out:
    // 0 - Vector3 normal
    fn vertex_normal(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();

        self.out_vec3_from4(&norm);
        pos
    }

    // in:
    // 0 - Vector3 normal
    fn pixel_normal(&self) -> Vector3<f32> {
        let color = Vector3::new(self.in_pixel_data[0],
                                 self.in_pixel_data[1],
                                 self.in_pixel_data[2]).add_s(1.0_f32).mul_s(128.0_f32);

        color
    }
}
