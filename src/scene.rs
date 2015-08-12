use cgmath::*;
use device::Device;
use mesh::Mesh;

pub struct Scene {
    device: Device,
    mat_proj: Matrix4<f32>,
    mat_view: Matrix4<f32>,
    cnt_triangle: u32,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
        Scene {
            device: Device::new("rust software render", width, height),
            mat_proj: Matrix4::<f32>::zero(),
            mat_view: Matrix4::<f32>::zero(),
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
        self
    }

    pub fn start(&mut self, color: u32) -> bool {
        if self.device.keyboard() {
            self.device.clear(color);
            self.cnt_triangle = 0;
            true
        }else {
            false
        }
    }

    pub fn draw(&mut self, mesh: &Mesh, mat_world: Matrix4<f32>) -> &mut Scene {
        let mat_proj_view_world = self.mat_proj * self.mat_view * mat_world;
        self.cnt_triangle += mesh.draw(&mat_proj_view_world, &mat_world, &mut self.device);

        self
    }

    pub fn present(&mut self) {
        self.device.present();
        self.device.update_fps(self.cnt_triangle);
    }
}
