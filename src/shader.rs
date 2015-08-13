use cgmath::*;

pub const MATRIX_PROJ_VIEW_WORLD: usize = 0;
pub const MATRIX_WORLD: usize = 1;

pub const VEC_POS: usize = 0;
pub const VEC_NORM: usize = 4;
pub const VEC_NEG_LIGHT: usize = 8;

pub struct Shader {
    pub matrix_arr: [Matrix4<f32>; 2],
    pub in_vertex_data: Vec<f32>,
    pub ambient_color: Vector3<f32>, // {r, g, b}
    pub diffuse_color: Vector3<f32>, // {r, g, b}
    pub ambient_intensity: f32,      // [0; 1]
    pub cos_nl: f32,                 // [0; 1]
}


impl Shader {
    pub fn new(ambient_color: Vector3<f32>, diffuse_color: Vector3<f32>, ambient_intensity: f32, cos_nl: f32) -> Shader {
        Shader {
            matrix_arr: [Matrix4::<f32>::zero(); 2],
            in_vertex_data: vec![0.0_f32; 12],
            ambient_color: ambient_color,
            diffuse_color: diffuse_color,
            ambient_intensity: ambient_intensity,
            cos_nl: cos_nl,
        }
    }

    pub fn set_matrix(&mut self, ind: usize, matrix: Matrix4<f32>) -> &mut Shader {
        self.matrix_arr[ind] = matrix;
        self
    }

    pub fn set_vec4(&mut self, sm: usize, vector: Vector4<f32>) -> &mut Shader {
        self.in_vertex_data[sm + 0] = vector.x;
        self.in_vertex_data[sm + 1] = vector.y;
        self.in_vertex_data[sm + 2] = vector.z;
        self.in_vertex_data[sm + 3] = vector.w;

        self
    }

    fn read_vec4(&self, sm: usize) -> Vector4<f32> {
        // Vector4::from_fixed_ref(self.in_vertex_data.slice(sm, sm+4))
        Vector4::new(self.in_vertex_data[sm + 0],
                     self.in_vertex_data[sm + 1],
                     self.in_vertex_data[sm + 2],
                     self.in_vertex_data[sm + 3])
    }

    pub fn vertex(&mut self) -> Vector4<f32> {
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(VEC_NORM)).normalize();
        self.cos_nl = norm.dot(&self.read_vec4(VEC_NEG_LIGHT)).max(0.0_f32);

        pos
    }

    pub fn pixel(&self) -> Vector3<f32> {
        let color = self.ambient_color
            .mul_s(self.ambient_intensity)
            .add_v(&self.diffuse_color.mul_s(self.cos_nl));

        color
    }
}
