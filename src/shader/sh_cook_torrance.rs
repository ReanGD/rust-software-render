use cgmath::{Vector, EuclideanVector, Vector2, Vector3, Vector4, Matrix};
use shader::base::*;


impl Shader {
    pub fn shader_cook_torrance() -> ShadersType {
        ([Shader::vertex_cook_torrance_color, Shader::vertex_cook_torrance_texture],
         [Shader::pixel_cook_torrance_color, Shader::pixel_cook_torrance_texture])
    }

    // out:
    // 0 - Vector3 view
    // 3 - Vector3 norm
    fn vertex_cook_torrance_color(&mut self) -> Vector4<f32> {
        let pos_pvw = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let pos_w = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM).normalize()).normalize();
        let view = self.read_vec4(IN_VS_VEC_EYE_POS).sub_v(&pos_w).normalize();

        self.out_vec3_from4(&view);
        self.out_vec3_from4(&norm);
        pos_pvw
    }

    // in:
    // 0 - Vector3 view
    // 3 - Vector3 norm
    fn pixel_cook_torrance_color(&self) -> Vector3<f32> {
        let view = Vector3::new(self.in_pixel_data[0], self.in_pixel_data[1], self.in_pixel_data[2]).normalize();
        let norm = Vector3::new(self.in_pixel_data[3], self.in_pixel_data[4], self.in_pixel_data[5]).normalize();
        let light = self.read_vec3(IN_VS_VEC_NEG_LIGHT);
        let half = view.add_v(&light).normalize();

        const ROUGHNESS: f32 = 0.3_f32;
        const ROUGHNESS_SQ: f32 = ROUGHNESS * ROUGHNESS;

        let cos_hn = half.dot(&norm).max(0.0000001_f32);
        let cos_hn_sq = cos_hn * cos_hn;
        let cos_vn = view.dot(&norm).max(0.0_f32);
        let cos_ln = light.dot(&norm).max(0.0_f32);
        let cos_vh = view.dot(&half).max(0.0_f32);

        let geometric = 1.0_f32.min((2.0_f32 * cos_hn * cos_vn.min(cos_ln)) / cos_vh);
        let frenel = 1.0_f32 / (1.0_f32 + cos_vn);
        let pow_val = (cos_hn_sq - 1.0_f32) / (ROUGHNESS_SQ * cos_hn_sq);
        let d = pow_val.exp() / (4.0_f32 * ROUGHNESS_SQ * cos_hn_sq * cos_hn_sq);
        let k = (geometric * frenel * d) / (cos_vn * cos_ln + 0.0000001_f32);

        let ambient = self.ambient.mul_s(self.ambient_intensity);
        let specular = self.specular.mul_s(k);
        let diffuse_specular = self.diffuse.add_v(&specular).mul_s(cos_ln.max(0.0_f32));

        ambient + diffuse_specular
    }

    // out:
    // 0 - Vector2 tex
    // 2 - Vector3 view
    // 5 - Vector3 norm
    fn vertex_cook_torrance_texture(&mut self) -> Vector4<f32> {
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
    fn pixel_cook_torrance_texture(&self) -> Vector3<f32> {
        let tex = Vector2::new(self.in_pixel_data[0], self.in_pixel_data[1]);
        let view = Vector3::new(self.in_pixel_data[2], self.in_pixel_data[3], self.in_pixel_data[4]).normalize();
        let norm = Vector3::new(self.in_pixel_data[5], self.in_pixel_data[6], self.in_pixel_data[7]).normalize();
        let light = self.read_vec3(IN_VS_VEC_NEG_LIGHT);
        let half = view.add_v(&light).normalize();

        const ROUGHNESS: f32 = 0.3_f32;
        const ROUGHNESS_SQ: f32 = ROUGHNESS * ROUGHNESS;

        let cos_hn = half.dot(&norm).max(0.0000001_f32);
        let cos_hn_sq = cos_hn * cos_hn;
        let cos_vn = view.dot(&norm).max(0.0_f32);
        let cos_ln = light.dot(&norm).max(0.0_f32);
        let cos_vh = view.dot(&half).max(0.0_f32);

        let geometric = 1.0_f32.min((2.0_f32 * cos_hn * cos_vn.min(cos_ln)) / cos_vh);
        let frenel = 1.0_f32 / (1.0_f32 + cos_vn);
        let pow_val = (cos_hn_sq - 1.0_f32) / (ROUGHNESS_SQ * cos_hn_sq);
        let d = pow_val.exp() / (4.0_f32 * ROUGHNESS_SQ * cos_hn_sq * cos_hn_sq);
        let k = (geometric * frenel * d) / (cos_vn * cos_ln + 0.0000001_f32);

        let color = match self.texture {
            Some(ref t) => t.tex_2d_bilinear(tex),
            None => panic!("texture is none"),
        };

        let ambient = color.mul_s(self.ambient_intensity);
        let specular = self.specular.mul_s(k);
        let diffuse_specular = color.add_v(&specular).mul_s(cos_ln.max(0.0_f32));

        ambient + diffuse_specular
    }
}
