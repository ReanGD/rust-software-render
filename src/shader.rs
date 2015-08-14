use cgmath::*;

pub const MATRIX_PROJ_VIEW_WORLD: usize = 0;
pub const MATRIX_WORLD: usize = 1;

pub const IN_VS_VEC_POS: usize = 0;
pub const IN_VS_VEC_NORM: usize = 4;
pub const IN_VS_VEC_NEG_LIGHT: usize = 8;

pub struct Shader {
    pub matrix_arr: [Matrix4<f32>; 2], // see MATRIX_*
    pub in_vertex_data: Vec<f32>,      // see IN_VS_*
    pub out_vertex_data: Vec<f32>,
    pub in_pixel_data: Vec<f32>,
    pub color: Vector3<f32>,           // {r, g, b}
    pub ambient_intensity: f32,        // [0; 1]
}

impl Shader {
    pub fn new(color: Vector3<f32>, ambient_intensity: f32) -> Shader {
        Shader {
            matrix_arr: [Matrix4::<f32>::zero(); 2],
            in_vertex_data: vec![0.0_f32; 12],
            out_vertex_data: vec![0.0_f32; 12],
            in_pixel_data: vec![0.0_f32; 12],
            color: color,
            ambient_intensity: ambient_intensity,
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

    // 0 - f32 cos_nl
    pub fn vertex(&mut self) -> (Vector4<f32>, usize) {
        let mut sm: usize = 0;
        let pos = self.matrix_arr[MATRIX_PROJ_VIEW_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_POS));
        let norm = self.matrix_arr[MATRIX_WORLD].mul_v(&self.read_vec4(IN_VS_VEC_NORM)).normalize();

        self.out_vertex_data[sm] = norm.dot(&self.read_vec4(IN_VS_VEC_NEG_LIGHT));
        sm += 1;

        (pos, sm)
    }

    // 0 - f32 cos_nl
    pub fn pixel(&self) -> Vector3<f32> {
        let cos_nl = self.in_pixel_data[0];
        let color = self.color.mul_s(cos_nl.max(self.ambient_intensity));

        color
    }
}
