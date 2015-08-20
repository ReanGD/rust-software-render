use std;
use cgmath::*;
use material::Material;
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
    pub texture: Texture,
    pub ambient: Vector3<f32>,           // {r, g, b}
    pub diffuse: Vector3<f32>,           // {r, g, b}
    pub specular: Vector3<f32>,          // {r, g, b}
    pub ambient_intensity: f32,          // [0; 1]
    pub vertex_out_len: usize,
    pub vertex_func: fn(&mut Shader) -> Vector4<f32>,
    pub pixel_func: fn(&Shader) -> Vector3<f32>,
}

impl Shader {
    pub fn new(material: &Material, texture: Texture) -> Shader {
        Shader {
            matrix_arr: [Matrix4::<f32>::zero(); 2],
            in_vertex_data: vec![0.0_f32; 18],
            out_vertex_data: [0.0_f32; MAX_OUT_VALUES],
            in_pixel_data: [0.0_f32; MAX_OUT_VALUES],
            texture: texture,
            ambient: material.ambient,
            diffuse: material.diffuse,
            specular: material.specular,
            ambient_intensity: material.ambient_intensity,
            vertex_out_len: 0,
            vertex_func: Shader::vertex_lambert,
            pixel_func: Shader::pixel_lambert,
        }
    }

    pub fn reset(&mut self, position: Vector4<f32>, normal: Vector4<f32>, tex: Vector2<f32>) {
        self.vertex_out_len = 0;
        self.set_vec4(IN_VS_VEC_POS, position);
        self.set_vec4(IN_VS_VEC_NORM, normal);
        self.set_vec2(IN_VS_VEC_TEX, tex);
    }

    pub fn set_shaders(&mut self,
                       vertex_func: fn(&mut Shader) -> Vector4<f32>,
                       pixel_func: fn(&Shader) -> Vector3<f32>) {
        self.vertex_func = vertex_func;
        self.pixel_func = pixel_func;
    }

    pub fn set_matrix(&mut self, ind: usize, matrix: Matrix4<f32>) {
        self.matrix_arr[ind] = matrix;
    }

    pub fn set_vec4(&mut self, sm: usize, vector: Vector4<f32>) {
        self.in_vertex_data[sm + 0] = vector.x;
        self.in_vertex_data[sm + 1] = vector.y;
        self.in_vertex_data[sm + 2] = vector.z;
        self.in_vertex_data[sm + 3] = vector.w;
    }
    
    pub fn set_vec2(&mut self, sm: usize, vector: Vector2<f32>) {
        self.in_vertex_data[sm + 0] = vector.x;
        self.in_vertex_data[sm + 1] = vector.y;
    }

    fn read_vec4(&self, sm: usize) -> Vector4<f32> {
        Vector4::new(self.in_vertex_data[sm + 0],
                     self.in_vertex_data[sm + 1],
                     self.in_vertex_data[sm + 2],
                     self.in_vertex_data[sm + 3])
    }

    fn read_vec3(&self, sm: usize) -> Vector3<f32> {
        Vector3::new(self.in_vertex_data[sm + 0],
                     self.in_vertex_data[sm + 1],
                     self.in_vertex_data[sm + 2])
    }

    fn read_vec2(&self, sm: usize) -> Vector2<f32> {
        Vector2::new(self.in_vertex_data[sm + 0],
                     self.in_vertex_data[sm + 1])
    }

    fn out_vec3_from4(&mut self, val: &Vector4<f32>) {
        self.out_vertex_data[self.vertex_out_len + 0] = val.x;
        self.out_vertex_data[self.vertex_out_len + 1] = val.y;
        self.out_vertex_data[self.vertex_out_len + 2] = val.z;
        self.vertex_out_len += 3;
    }

    fn out_vec2(&mut self, val: &Vector2<f32>) {
        self.out_vertex_data[self.vertex_out_len + 0] = val.x;
        self.out_vertex_data[self.vertex_out_len + 1] = val.y;
        self.vertex_out_len += 2;
    }
    
    fn out_f32(&mut self, val: f32) {
        self.out_vertex_data[self.vertex_out_len] = val;
        self.vertex_out_len += 1;
    }

    // out:
    // 0 - Vector3 normal
    #[allow(dead_code)]
    pub fn vertex_normals(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();

        self.out_vec3_from4(&norm);
        pos
    }

    // in:
    // 0 - Vector3 normal
    #[allow(dead_code)]
    pub fn pixel_normals(&self) -> Vector3<f32> {
        let color = Vector3::new(self.in_pixel_data[0], self.in_pixel_data[1], self.in_pixel_data[2]).add_s(1.0_f32).mul_s(128.0_f32);

        color
    }

    // out:
    // 0 - f32 cos_nl
    #[allow(dead_code)]
    pub fn vertex_lambert(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
        let cos_nl = norm.dot(&self.read_vec4(IN_VS_VEC_NEG_LIGHT));

        self.out_f32(cos_nl);
        pos
    }

    // in:
    // 0 - f32 cos_nl
    #[allow(dead_code)]
    pub fn pixel_lambert(&self) -> Vector3<f32> {
        let cos_nl = self.in_pixel_data[0];
        let ambient = self.ambient.mul_s(self.ambient_intensity);
        let diffuse = self.diffuse.mul_s(cos_nl.max(0.0_f32));

        ambient + diffuse
    }

    // out:
    // 0 - Vector2 tex
    #[allow(dead_code)]
    pub fn vertex_tex(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        // let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
        let mut tex = self.read_vec2(IN_VS_VEC_TEX);
        tex.x *= (self.texture.size_x - 1) as f32;
        tex.y *= (self.texture.size_y - 1) as f32;

        self.out_vec2(&tex);
        pos
    }

    // in:
    // 0 - Vector2 tex
    #[allow(dead_code)]
    pub fn pixel_tex(&self) -> Vector3<f32> {
        let tex = Vector2::<f32>::new(self.in_pixel_data[0], self.in_pixel_data[1]);
        let x = std::cmp::max(tex.x as i32, 0) as usize % self.texture.size_x;
        let y = std::cmp::max(tex.y as i32, 0) as usize % self.texture.size_y;
        let color = self.texture.data[y * self.texture.size_x + x];

        color
    }

    // out:
    // 0 - Vector3 view
    // 3 - Vector3 norm
    #[allow(dead_code)]
    pub fn vertex_phong_blinn(&mut self) -> Vector4<f32> {
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
    pub fn pixel_phong_blinn(&self) -> Vector3<f32> {
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
    // 0 - Vector3 view
    // 3 - Vector3 norm
    #[allow(dead_code)]
    pub fn vertex_cook_torrance(&mut self) -> Vector4<f32> {
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
    pub fn pixel_cook_torrance(&self) -> Vector3<f32> {
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
}
