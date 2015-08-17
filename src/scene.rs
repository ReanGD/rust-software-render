use cgmath::*;
use mesh::Model;
use device::Device;
use shader::*;

pub struct Scene {
    device: Device,
    mat_proj: Matrix4<f32>,
    mat_view: Matrix4<f32>,
    vec_neg_light: Vector4<f32>,
    vec_eye_pos: Vector4<f32>,
    cnt_triangle: u32,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
        Scene {
            device: Device::new("rust software render", width, height),
            mat_proj: Matrix4::<f32>::zero(),
            mat_view: Matrix4::<f32>::zero(),
            vec_neg_light: Vector4::<f32>::zero(),
            vec_eye_pos: Vector4::<f32>::zero(),
            cnt_triangle: 0,
        }
    }

    pub fn proj<A: Angle<f32>>(&mut self, fovy: A, near: f32, far: f32) -> &mut Scene {
        let aspect = self.device.x_size as f32 / self.device.y_size as f32;
        self.mat_proj = perspective(fovy, aspect, near, far);
        self
    }

    pub fn view(&mut self, eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>) -> &mut Scene {
        self.mat_view = Matrix4::<f32>::look_at(&eye, &center, &up);
        self.vec_eye_pos = Vector4::new(eye.x, eye.y, eye.z, 1.0_f32);
        
        self
    }

    pub fn light(&mut self, vec: Vector3<f32>) -> &mut Scene {
        self.vec_neg_light = Vector4::new(vec.x, vec.y, vec.z, 0.0_f32).normalize();
        self.vec_neg_light.neg_self();

        self
    }

    pub fn start(&mut self, color: u32) -> bool {
        if self.device.keyboard() {
            self.device.clear(color);
            self.cnt_triangle = 0;
            true
        } else {
            false
        }
    }

    pub fn draw(&mut self, mesh: &Model, mat_world: Matrix4<f32>, shader: &mut Shader) -> &mut Scene {
        shader.set_matrix(MATRIX_PROJ_VIEW_WORLD, self.mat_proj * self.mat_view * mat_world);
        shader.set_matrix(MATRIX_WORLD, mat_world);
        shader.set_vec4(IN_VS_VEC_NEG_LIGHT, self.vec_neg_light);
        shader.set_vec4(IN_VS_VEC_EYE_POS, self.vec_eye_pos);

        self.cnt_triangle += mesh.draw(shader, &mut self.device);

        self
    }

    pub fn present(&mut self) {
        self.device.present();
        self.device.update_fps(self.cnt_triangle);
    }
}
