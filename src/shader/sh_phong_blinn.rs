use cgmath::{Vector, EuclideanVector, Vector2, Vector3, Vector4, Matrix};
use shader::base::*;


impl Shader {
    pub fn shader_phong_blinn() -> ShadersType {
        ([Shader::vertex_phong_blinn_color, Shader::vertex_phong_blinn_texture],
         [Shader::pixel_phong_blinn_color, Shader::pixel_phong_blinn_texture])
    }

    // out:
    // 0 - Vector3 view
    // 3 - Vector3 norm
    fn vertex_phong_blinn_color(&mut self) -> Vector4<f32> {
        let pos_pvw = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let pos_w = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
        let view = self.read_vec4(IN_VS_VEC_EYE_POS).sub_v(&pos_w).normalize();

        self.out_vec3_from4(&view);
        self.out_vec3_from4(&norm);
        pos_pvw
    }

    // in:
    // 0 - Vector3 view
    // 3 - Vector3 norm
    #[allow(dead_code)]
    fn pixel_phong_blinn_color(&self) -> Vector3<f32> {
        let view = Vector3::new(self.in_pixel_data[0], self.in_pixel_data[1], self.in_pixel_data[2]).normalize();
        let norm = Vector3::new(self.in_pixel_data[3], self.in_pixel_data[4], self.in_pixel_data[5]).normalize();
        let light = self.read_vec3(IN_VS_VEC_NEG_LIGHT);
        let half = view.add_v(&light).normalize();
        let cos_nh = norm.dot(&half).max(0.0_f32);
        let cos_nl = norm.dot(&light).max(0.0_f32);

        const POWER: i32 = 5;

        let ambient = self.ambient.mul_s(self.ambient_intensity);
        let diffuse = self.diffuse.mul_s(cos_nl);
        let specular = self.specular.mul_s(cos_nh.powi(POWER));

        ambient + diffuse + specular
    }

    // out:
    // 0 - Vector2 tex
    // 2 - Vector3 view
    // 5 - Vector3 norm
    fn vertex_phong_blinn_texture(&mut self) -> Vector4<f32> {
        let pos_pvw = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let pos_w = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let mut norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
        norm.neg_self();
        let view = self.read_vec4(IN_VS_VEC_EYE_POS).sub_v(&pos_w).normalize();
        let tex = self.read_vec2(IN_VS_VEC_TEX);

        self.out_vec2(&tex);
        self.out_vec3_from4(&view);
        self.out_vec3_from4(&norm);
        pos_pvw
    }

    // in:
    // 0 - Vector2 tex
    // 2 - Vector3 view
    // 5 - Vector3 norm
    fn pixel_phong_blinn_texture(&self) -> Vector3<f32> {
        let tex = Vector2::new(self.in_pixel_data[0], self.in_pixel_data[1]);
        let view = Vector3::new(self.in_pixel_data[2], self.in_pixel_data[3], self.in_pixel_data[4]).normalize();
        let norm = Vector3::new(self.in_pixel_data[5], self.in_pixel_data[6], self.in_pixel_data[7]).normalize();
        let light = self.read_vec3(IN_VS_VEC_NEG_LIGHT);
        let half = view.add_v(&light).normalize();
        let cos_nh = norm.dot(&half).max(0.0_f32);
        let cos_nl = norm.dot(&light).max(0.0_f32);

        const POWER: i32 = 5;

        let color = match self.texture {
            Some(ref t) => t.tex_2d(tex),
            None => panic!("texture is none"),
        };

        let ambient = color.mul_s(self.ambient_intensity);
        let diffuse = color.mul_s(cos_nl);
        let specular = self.specular.mul_s(cos_nh.powi(POWER));

        ambient + diffuse + specular
    }
}
