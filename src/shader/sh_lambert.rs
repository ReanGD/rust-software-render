use cgmath::{Vector, EuclideanVector, Vector2, Vector3, Vector4, Matrix};
use shader::base::*;


impl Shader {
    pub fn set_lambert(&mut self) {
        self.vertex_func = [Shader::vertex_lambert_color, Shader::vertex_lambert_texture];
        self.pixel_func = [Shader::pixel_lambert_color, Shader::pixel_lambert_texture];
    }

    // out:
    // 0 - f32 cos_nl
    fn vertex_lambert_color(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
        let cos_nl = norm.dot(&self.read_vec4(IN_VS_VEC_NEG_LIGHT));

        self.out_f32(cos_nl);
        pos
    }

    // in:
    // 0 - f32 cos_nl
    fn pixel_lambert_color(&self) -> Vector3<f32> {
        let cos_nl = self.in_pixel_data[0];
        let ambient = self.ambient.mul_s(self.ambient_intensity);
        let diffuse = self.diffuse.mul_s(cos_nl.max(0.0_f32));

        ambient + diffuse
    }

    // out:
    // 0 - Vector2 tex
    // 2 - f32 cos_nl
    fn vertex_lambert_texture(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
        let tex = self.read_vec2(IN_VS_VEC_TEX);
        let cos_nl = norm.dot(&self.read_vec4(IN_VS_VEC_NEG_LIGHT));

        self.out_vec2(&tex);
        self.out_f32(cos_nl);
        pos
    }

    // in:
    // 0 - Vector2 tex
    // 2 - f32 cos_nl
    fn pixel_lambert_texture(&self) -> Vector3<f32> {
        let tex = Vector2::new(self.in_pixel_data[0], self.in_pixel_data[1]);
        let cos_nl = self.in_pixel_data[2];

        let color = match self.texture {
            Some(ref t) => t.tex_2d(tex),
            None => panic!("texture is none"),
        };

        let ambient = color.mul_s(self.ambient_intensity);
        let diffuse = color.mul_s(cos_nl.max(0.0_f32));

        ambient + diffuse
    }
}
