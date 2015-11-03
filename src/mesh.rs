use std;
use cgmath::*;
use shader::*;
use device::Device;
use material::Material;
use rasterization::triangle;
use texture::TextureCube;

#[derive(Copy,Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex: Vector2<f32>,
}

pub struct Mesh {
    pub index_buffer: Vec<u32>,
    pub material_id: usize,
}

pub struct Model {
    pub vertex_buffer: Vec<Vertex>,
    pub material_list: Vec<Material>,
    pub mesh_list: Vec<Mesh>,
    min: Vector3<f32>,
    max: Vector3<f32>,
    normalize: bool,
}

impl Vertex {
    pub fn new(position: &Vector3<f32>, tex: &Vector2<f32>, normal: &Vector3<f32>) -> Vertex {
        Vertex {
            position: position.clone(),
            normal: normal.clone(),
            tex: tex.clone(),
        }
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            index_buffer: Vec::<u32>::new(),
            material_id: 0,
        }
    }

    fn draw(&self, shader: &mut Shader,
            material: &Material,
            vertex_buffer: &Vec<Vertex>,
            device: &mut Device) -> u32 {
        shader.set_material(material);
        let vertex_func = match material.texture {
            None => shader.vertex_func[0],
            Some(_) => shader.vertex_func[1],
        };
        let cnt_triangle = self.index_buffer.len() / 3;
        for indexes in self.index_buffer.chunks(3) {
            let mut points_2d: [Point3<f32>; 3] = [Point3::<f32>::new(0.0, 0.0, 0.0); 3];
            let mut tex_coord: [Vector2<f32>; 3] = [Vector2::<f32>::new(0.0, 0.0); 3];
            let mut vertex_out = [[0.0_f32;MAX_OUT_VALUES];3];
            let mut vertex_out_len = 0;
            for i in 0..3 {
                let p = vertex_buffer[indexes[i] as usize];
                let v = p.position;
                let n = p.normal;
                let t = p.tex;

                shader.reset(Vector4::<f32>::new(v.x, v.y, v.z, 1.0_f32), Vector4::<f32>::new(n.x, n.y, n.z, 1.0_f32), t);
                let p_screen = vertex_func(shader);
                vertex_out_len = shader.vertex_out_len;
                let inverse_w = 1.0_f32 / p_screen.w;

                for ind in 0..vertex_out_len {
                    vertex_out[i][ind] = shader.out_vertex_data[ind] * inverse_w;
                }

                tex_coord[i] = t;
                points_2d[i] = Point3::new(
                    (p_screen.x * inverse_w + 1.0_f32) * device.x_size as f32 * 0.5_f32,
                    (p_screen.y * inverse_w + 1.0_f32) * device.y_size as f32 * 0.5_f32,
                    inverse_w);
            }

            let col0 = Vector3::new(points_2d[0].x, points_2d[1].x, points_2d[2].x);
            let col1 = Vector3::new(points_2d[0].y, points_2d[1].y, points_2d[2].y);
            let col2 = Vector3::new(1.0_f32,     1.0_f32,     1.0_f32    );
            if Matrix3::from_cols(col0, col1, col2).determinant() < 0.0_f32 {
                continue;
            }

            // calc mip level:
            shader.texture = match material.texture {
                Some(ref texture) => {
                    let ba_pixel = Vector2::new(points_2d[1].x, points_2d[1].y)
                        .sub_v(&Vector2::new(points_2d[0].x, points_2d[0].y));
                    let ca_pixel = Vector2::new(points_2d[2].x, points_2d[2].y)
                        .sub_v(&Vector2::new(points_2d[0].x, points_2d[0].y));

                    let tex_size = texture.size;
                    let ba_texel = tex_coord[1].sub_v(&tex_coord[0]).mul_v(&tex_size);
                    let ca_texel = tex_coord[2].sub_v(&tex_coord[0]).mul_v(&tex_size);
                    // cross product in 2d = 2 * square of triangle
                    let sq_pixel = ba_pixel.x * ca_pixel.y - ba_pixel.y * ca_pixel.x;
                    let sq_texel = ba_texel.x * ca_texel.y - ba_texel.y * ca_texel.x;
                    let lod = (sq_texel / sq_pixel).abs().sqrt().max(1.0_f32).log2() as usize;
                    Some(texture.get_surface(lod))
                },
                None => None,
            };

            triangle(&mut device.cbuffer,
                     &mut device.zbuffer,
                     device.x_size,
                     device.y_size,
                     points_2d, vertex_out, vertex_out_len, shader);
        }

        cnt_triangle as u32
    }
}

impl Model {
    pub fn new() -> Model {
        Model {
            vertex_buffer: Vec::<Vertex>::new(),
            material_list: Vec::<Material>::new(),
            mesh_list: Vec::<Mesh>::new(),
            min: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            max: Vector3::new(0.0_f32, 0.0_f32, 0.0_f32),
            normalize: false,
        }
    }

    pub fn with_normalize(min: Vector3<f32>, max: Vector3<f32>) -> Model {
        Model {
            vertex_buffer: Vec::<Vertex>::new(),
            material_list: Vec::<Material>::new(),
            mesh_list: Vec::<Mesh>::new(),
            min: min,
            max: max,
            normalize: true,
        }
    }

    pub fn add_texture_cube(&mut self, dir_path: &std::path::Path, image_extension: &str) -> Result<(), String> {
        let texture = std::rc::Rc::new(try!(TextureCube::new(dir_path, image_extension)));
        for material in &mut self.material_list {
            material.add_texture_cube(texture.clone());
        }

        Ok(())
    }

    pub fn draw(&self, shader: &mut Shader, device: &mut Device) -> u32 {
        let mut triangle_cnt: u32 = 0;
        for mesh in &self.mesh_list {
            triangle_cnt += mesh.draw(shader,
                                      &self.material_list[mesh.material_id],
                                      &self.vertex_buffer,
                                      device);
        }

        triangle_cnt
    }

    pub fn to_center_matrix(&self) -> Matrix4<f32> {
        if self.normalize {
            let size = self.max.sub_v(&self.min);
            let scale = Vector3::from_value(1.0_f32/(size.x.max(size.y).max(size.z)));
            let center = self.min.add_v(&size.mul_s(0.5_f32));
            let mat_move = Matrix4::from_translation(&center.mul_s(-1.0_f32));
            let mat_scale = Matrix4::from(Matrix3::from_diagonal(&scale));
            mat_scale * mat_move
        } else {
            Matrix4::identity()
        }
    }
}
