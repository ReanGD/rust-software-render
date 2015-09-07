use cgmath::{Vector2, Vector4};
use shader::base::Shader;

impl Shader {
    pub fn read_vec2(&self, sm: usize) -> Vector2<f32> {
        Vector2::new(self.in_vertex_data[sm + 0],
                     self.in_vertex_data[sm + 1])
    }

    pub fn read_vec4(&self, sm: usize) -> Vector4<f32> {
        Vector4::new(self.in_vertex_data[sm + 0],
                     self.in_vertex_data[sm + 1],
                     self.in_vertex_data[sm + 2],
                     self.in_vertex_data[sm + 3])
    }

    pub fn out_vec2(&mut self, val: &Vector2<f32>) {
        self.out_vertex_data[self.vertex_out_len + 0] = val.x;
        self.out_vertex_data[self.vertex_out_len + 1] = val.y;
        self.vertex_out_len += 2;
    }

    pub fn out_f32(&mut self, val: f32) {
        self.out_vertex_data[self.vertex_out_len] = val;
        self.vertex_out_len += 1;
    }

    // pub fn read_vec3(&self, sm: usize) -> Vector3<f32> {
    //     Vector3::new(self.in_vertex_data[sm + 0],
    //                  self.in_vertex_data[sm + 1],
    //                  self.in_vertex_data[sm + 2])
    // }


    // pub fn out_vec3_from4(&mut self, val: &Vector4<f32>) {
    //     self.out_vertex_data[self.vertex_out_len + 0] = val.x;
    //     self.out_vertex_data[self.vertex_out_len + 1] = val.y;
    //     self.out_vertex_data[self.vertex_out_len + 2] = val.z;
    //     self.vertex_out_len += 3;
    // }
}
