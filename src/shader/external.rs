use cgmath::{Vector2, Vector3, Vector4, Matrix, Matrix4};
use shader::base::*;
use material::Material;

impl Shader {
    pub fn new(shader_type: ShaderType) -> Shader {
        let (vertex_func, pixel_func) = match shader_type {
            ShaderType::Default => Shader::shader_default(),
            ShaderType::Normal => Shader::shader_normal(),
            ShaderType::Lambert => Shader::shader_lambert(),
            ShaderType::PhongBlinn => Shader::shader_phong_blinn(),
            ShaderType::CookTorrance => Shader::shader_cook_torrance(),
        };

        Shader {
            matrix_arr: [Matrix4::<f32>::zero(); 2],
            in_vertex_data: vec![0.0_f32; 18],
            out_vertex_data: [0.0_f32; MAX_OUT_VALUES],
            in_pixel_data: [0.0_f32; MAX_OUT_VALUES],
            texture: None,
            ambient: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            diffuse: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            specular: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            ambient_intensity: 0.0_f32,
            vertex_out_len: 0,
            vertex_func: vertex_func,
            pixel_func: pixel_func,
        }
    }

    pub fn reset(&mut self, position: Vector4<f32>, normal: Vector4<f32>, tex: Vector2<f32>) {
        self.vertex_out_len = 0;
        self.set_vec4(IN_VS_VEC_POS, position);
        self.set_vec4(IN_VS_VEC_NORM, normal);
        self.set_vec2(IN_VS_VEC_TEX, tex);
    }

    pub fn set_material(&mut self, material: &Material) {
        self.ambient = material.ambient;
        self.diffuse = material.diffuse;
        self.specular = material.specular;
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
}
