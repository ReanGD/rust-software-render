use cgmath::*;

pub const MATRIX_PROJ_VIEW_WORLD: usize = 0;
pub const MATRIX_WORLD: usize = 1;

pub const IN_VS_VEC_POS: usize = 0;
pub const IN_VS_VEC_NORM: usize = 4;
pub const IN_VS_VEC_NEG_LIGHT: usize = 8;
pub const IN_VS_VEC_EYE_POS: usize = 12;

pub struct Shader {
    pub matrix_arr: [Matrix4<f32>; 2], // see MATRIX_*
    pub in_vertex_data: Vec<f32>,      // see IN_VS_*
    pub out_vertex_data: Vec<f32>,
    pub in_pixel_data: Vec<f32>,
    pub color: Vector3<f32>,           // {r, g, b}
    pub ambient_intensity: f32,        // [0; 1]
    pub vertex_out_len: usize,
    pub vertex_func: fn(&mut Shader) -> Vector4<f32>,
    pub pixel_func: fn(&Shader) -> Vector3<f32>,
}

impl Shader {
    pub fn new(color: Vector3<f32>, ambient_intensity: f32) -> Shader {
        Shader {
            matrix_arr: [Matrix4::<f32>::zero(); 2],
            in_vertex_data: vec![0.0_f32; 16],
            out_vertex_data: vec![0.0_f32; 16],
            in_pixel_data: vec![0.0_f32; 12],
            color: color,
            ambient_intensity: ambient_intensity,
            vertex_out_len: 0,
            vertex_func: Shader::vertex_def,
            pixel_func: Shader::pixel_def,
        }
    }

    pub fn reset(&mut self, position: Vector4<f32>, normal: Vector4<f32>) {
        self.vertex_out_len = 0;
        self.set_vec4(IN_VS_VEC_POS, position);
        self.set_vec4(IN_VS_VEC_NORM, normal);
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

    fn read_vec4(&self, sm: usize) -> Vector4<f32> {
        // Vector4::from_fixed_ref(self.in_vertex_data.slice(sm, sm+4))
        Vector4::new(self.in_vertex_data[sm + 0],
                     self.in_vertex_data[sm + 1],
                     self.in_vertex_data[sm + 2],
                     self.in_vertex_data[sm + 3])
    }

    fn out_vec3(&mut self, val: &Vector3<f32>) {
        self.out_vertex_data[self.vertex_out_len + 0] = val.x;
        self.out_vertex_data[self.vertex_out_len + 1] = val.y;
        self.out_vertex_data[self.vertex_out_len + 2] = val.z;
        self.vertex_out_len += 3;
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

        self.out_vec3(&Vector3::new(norm.x, norm.y, norm.z));
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
        let color = self.color.mul_s(cos_nl.max(0.0_f32));

        color
    }

    // out:
    // 0 - Vector3 eye_vec
    // 3 - Vector3 norm
    #[allow(dead_code)]
    pub fn vertex_phong_blinn(&mut self) -> Vector4<f32> {
        let pos_pvw = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let pos_w = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
        let eye_vec = self.read_vec4(IN_VS_VEC_EYE_POS).sub_v(&pos_w).normalize();

        self.out_vec3(&Vector3::new(eye_vec.x, eye_vec.y, eye_vec.z));
        self.out_vec3(&Vector3::new(norm.x, norm.y, norm.z));
        pos_pvw
    }

    // in:
    // 0 - Vector3 eye_vec
    // 3 - Vector3 norm
    #[allow(dead_code)]
    pub fn pixel_phong_blinn(&self) -> Vector3<f32> {
        let eye_vec = Vector3::new(self.in_pixel_data[0], self.in_pixel_data[1], self.in_pixel_data[2]).normalize();
        let norm = Vector3::new(self.in_pixel_data[3], self.in_pixel_data[4], self.in_pixel_data[5]).normalize();
        let light4 = self.read_vec4(IN_VS_VEC_NEG_LIGHT);
        let light = Vector3::new(light4.x, light4.y, light4.z);
        let half = eye_vec.add_v(&light).normalize();
        let cos_nh = norm.dot(&half);
        let cos_nl = norm.dot(&light);

        let spec_power = 50.0_f32;

        let diff = self.color.mul_s(cos_nl.max(0.0_f32));
        let spec = Vector3::new(255.0_f32, 255.0_f32, 255.0_f32).mul_s(cos_nh.max(0.0_f32).powf(spec_power) * 20.0_f32);

        diff + spec
    }

    // out:
    // 0 - f32 cos_nl
    pub fn vertex_def(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();
        let cos_nl = norm.dot(&self.read_vec4(IN_VS_VEC_NEG_LIGHT));

        self.out_f32(cos_nl);
        pos
    }

    // in:
    // 0 - f32 cos_nl
    pub fn pixel_def(&self) -> Vector3<f32> {
        let cos_nl = self.in_pixel_data[0];
        let color = self.color.mul_s(cos_nl.max(self.ambient_intensity));

        color
    }
}
