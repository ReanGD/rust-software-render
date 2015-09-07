use cgmath::{Vector2, Vector3, Vector4, Matrix4};
use shader::base::*;
use material::Material;

impl Shader {
    pub fn new() -> Shader {
        Shader {
            matrix_arr: [Matrix4::<f32>::zero(); 2],
            in_vertex_data: vec![0.0_f32; 18],
            out_vertex_data: [0.0_f32; MAX_OUT_VALUES],
            in_pixel_data: [0.0_f32; MAX_OUT_VALUES],
            texture: None,
            texture_lod: 0,
            ambient: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            diffuse: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            specular: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            ambient_intensity: 0.0_f32,
            vertex_out_len: 0,
            vertex_func: [Shader::vertex_default, Shader::vertex_default],
            pixel_func: [Shader::pixel_default, Shader::pixel_default],
        }
    }

    pub fn reset(&mut self, position: Vector4<f32>, normal: Vector4<f32>, tex: Vector2<f32>) {
        self.vertex_out_len = 0;
        self.set_vec4(IN_VS_VEC_POS, position);
        self.set_vec4(IN_VS_VEC_NORM, normal);
        self.set_vec2(IN_VS_VEC_TEX, tex);
    }

    pub fn set_material(&mut self, material: &Material) {
        self.texture = match material.texture {
            Some(ref v) => Some(v.clone()),
            None => None
        };
        self.texture_lod = 0;
        self.ambient = material.ambient;
        self.diffuse = material.diffuse;
        self.specular = material.specular;
        self.ambient_intensity = material.ambient_intensity;
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
