use std;
use cgmath::{Vector3, Vector4, Matrix4};
use texture::Texture;

pub const MATRIX_PROJ_VIEW_WORLD: usize = 0;
pub const MATRIX_WORLD: usize = 1;

pub const IN_VS_VEC_POS: usize = 0;
pub const IN_VS_VEC_NORM: usize = 4;
pub const IN_VS_VEC_TEX: usize = 8;
pub const IN_VS_VEC_NEG_LIGHT: usize = 10;
pub const IN_VS_VEC_EYE_POS: usize = 14;

pub const MAX_OUT_VALUES: usize = 16;

pub struct Shader {
    pub matrix_arr: [Matrix4<f32>; 2], // see MATRIX_*
    pub in_vertex_data: Vec<f32>,      // see IN_VS_*
    pub out_vertex_data: [f32; MAX_OUT_VALUES],
    pub in_pixel_data: [f32; MAX_OUT_VALUES],
    pub texture: Option<std::rc::Rc<Texture>>,
    pub texture_lod: usize,
    pub ambient: Vector3<f32>,           // {r, g, b}
    pub diffuse: Vector3<f32>,           // {r, g, b}
    pub specular: Vector3<f32>,          // {r, g, b}
    pub ambient_intensity: f32,          // [0; 1]
    pub vertex_out_len: usize,
    pub vertex_func: [fn(&mut Shader) -> Vector4<f32>; 2],
    pub pixel_func: [fn(&Shader) -> Vector3<f32>; 2],
}

impl Shader {
    // out:
    // 0 - Vector3 normal
    // #[allow(dead_code)]
    // pub fn vertex_normals(&mut self) -> Vector4<f32> {
    //     let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
    //     let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();

    //     self.out_vec3_from4(&norm);
    //     pos
    // }

    // // in:
    // // 0 - Vector3 normal
    // #[allow(dead_code)]
    // pub fn pixel_normals(&self) -> Vector3<f32> {
    //     let color = Vector3::new(self.in_pixel_data[0], self.in_pixel_data[1], self.in_pixel_data[2]).add_s(1.0_f32).mul_s(128.0_f32);

    //     color
    // }

    // // out:
    // // 0 - Vector2 tex
    // #[allow(dead_code)]
    // pub fn vertex_tex(&mut self) -> Vector4<f32> {
    //     let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
    //     // let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
    //     let tex = self.read_vec2(IN_VS_VEC_TEX);

    //     self.out_vec2(&tex);
    //     pos
    // }

    // // in:
    // // 0 - Vector2 tex
    // #[allow(dead_code)]
    // pub fn pixel_tex(&self) -> Vector3<f32> {
    //     let tex = Vector2::<f32>::new(self.in_pixel_data[0], self.in_pixel_data[1]);
    //     let lod = self.texture_lod;

    //     let size_x = self.texture.levels[self.texture_lod].size_x;
    //     let size_y = self.texture.levels[self.texture_lod].size_y;

    //     let x = std::cmp::max((tex.x * (size_x as f32)) as i32, 0) as usize % size_x;
    //     let y = std::cmp::max((tex.y * (size_y as f32)) as i32, 0) as usize % size_y;
    //     let color = self.texture.levels[lod].data[y * size_x + x];

    //     color
    // }

    // // out:
    // // 0 - Vector3 view
    // // 3 - Vector3 norm
    // #[allow(dead_code)]
    // pub fn vertex_phong_blinn(&mut self) -> Vector4<f32> {
    //     let pos_pvw = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
    //     let pos_w = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
    //     let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
    //     let view = self.read_vec4(IN_VS_VEC_EYE_POS).sub_v(&pos_w).normalize();

    //     self.out_vec3_from4(&view);
    //     self.out_vec3_from4(&norm);
    //     pos_pvw
    // }

    // // in:
    // // 0 - Vector3 view
    // // 3 - Vector3 norm
    // #[allow(dead_code)]
    // pub fn pixel_phong_blinn(&self) -> Vector3<f32> {
    //     let view = Vector3::new(self.in_pixel_data[0], self.in_pixel_data[1], self.in_pixel_data[2]).normalize();
    //     let norm = Vector3::new(self.in_pixel_data[3], self.in_pixel_data[4], self.in_pixel_data[5]).normalize();
    //     let light = self.read_vec3(IN_VS_VEC_NEG_LIGHT);
    //     let half = view.add_v(&light).normalize();
    //     let cos_nh = norm.dot(&half).max(0.0_f32);
    //     let cos_nl = norm.dot(&light).max(0.0_f32);

    //     const POWER: i32 = 5;

    //     let ambient = self.ambient.mul_s(self.ambient_intensity);
    //     let diffuse = self.diffuse.mul_s(cos_nl);
    //     let specular = self.specular.mul_s(cos_nh.powi(POWER));

    //     ambient + diffuse + specular
    // }

    // // out:
    // // 0 - Vector3 view
    // // 3 - Vector3 norm
    // #[allow(dead_code)]
    // pub fn vertex_cook_torrance(&mut self) -> Vector4<f32> {
    //     let pos_pvw = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
    //     let pos_w = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
    //     let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
    //     let view = self.read_vec4(IN_VS_VEC_EYE_POS).sub_v(&pos_w).normalize();

    //     self.out_vec3_from4(&view);
    //     self.out_vec3_from4(&norm);
    //     pos_pvw
    // }

    // // in:
    // // 0 - Vector3 view
    // // 3 - Vector3 norm
    // #[allow(dead_code)]
    // pub fn pixel_cook_torrance(&self) -> Vector3<f32> {
    //     let view = Vector3::new(self.in_pixel_data[0], self.in_pixel_data[1], self.in_pixel_data[2]).normalize();
    //     let norm = Vector3::new(self.in_pixel_data[3], self.in_pixel_data[4], self.in_pixel_data[5]).normalize();
    //     let light = self.read_vec3(IN_VS_VEC_NEG_LIGHT);
    //     let half = view.add_v(&light).normalize();

    //     const ROUGHNESS: f32 = 0.3_f32;
    //     const ROUGHNESS_SQ: f32 = ROUGHNESS * ROUGHNESS;

    //     let cos_hn = half.dot(&norm).max(0.0000001_f32);
    //     let cos_hn_sq = cos_hn * cos_hn;
    //     let cos_vn = view.dot(&norm).max(0.0_f32);
    //     let cos_ln = light.dot(&norm).max(0.0_f32);
    //     let cos_vh = view.dot(&half).max(0.0_f32);

    //     let geometric = 1.0_f32.min((2.0_f32 * cos_hn * cos_vn.min(cos_ln)) / cos_vh);
    //     let frenel = 1.0_f32 / (1.0_f32 + cos_vn);
    //     let pow_val = (cos_hn_sq - 1.0_f32) / (ROUGHNESS_SQ * cos_hn_sq);
    //     let d = pow_val.exp() / (4.0_f32 * ROUGHNESS_SQ * cos_hn_sq * cos_hn_sq);
    //     let k = (geometric * frenel * d) / (cos_vn * cos_ln + 0.0000001_f32);

    //     let ambient = self.ambient.mul_s(self.ambient_intensity);
    //     let specular = self.specular.mul_s(k);
    //     let diffuse_specular = self.diffuse.add_v(&specular).mul_s(cos_ln.max(0.0_f32));

    //     ambient + diffuse_specular
    // }
}
