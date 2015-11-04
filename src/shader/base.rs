use std;
use cgmath::{Vector3, Vector4, Matrix4};
use texture::{Surface, TextureCube};

pub const MATRIX_PROJ_VIEW_WORLD: usize = 0;
pub const MATRIX_WORLD: usize = 1;

pub const IN_VS_VEC_POS: usize = 0;
pub const IN_VS_VEC_NORM: usize = 4;
pub const IN_VS_VEC_TEX: usize = 8;
pub const IN_VS_VEC_NEG_LIGHT: usize = 10;
pub const IN_VS_VEC_EYE_POS: usize = 14;

pub const MAX_OUT_VALUES: usize = 16;

pub type ShadersVertex = [fn(&mut Shader) -> Vector4<f32>; 2];
pub type ShadersPixel = [fn(&Shader) -> Vector3<f32>; 2];
pub type ShadersType = (ShadersVertex, ShadersPixel);

pub struct Shader {
    pub matrix_arr: [Matrix4<f32>; 2], // see MATRIX_*
    pub in_vertex_data: Vec<f32>,      // see IN_VS_*
    pub out_vertex_data: [f32; MAX_OUT_VALUES],
    pub in_pixel_data: [f32; MAX_OUT_VALUES],
    pub texture: Option<std::rc::Rc<Surface>>,
    pub texture_cube: Option<std::rc::Rc<TextureCube>>,
    pub ambient: Vector3<f32>,           // {r, g, b}
    pub diffuse: Vector3<f32>,           // {r, g, b}
    pub specular: Vector3<f32>,          // {r, g, b}
    pub ambient_intensity: f32,          // [0; 1]
    pub vertex_out_len: usize,
    pub vertex_out2_base: usize,
    pub vertex_func: ShadersVertex,
    pub pixel_func: ShadersPixel,
}

#[allow(dead_code)]
pub enum ShaderType {
    Default,
    Normal,
    Lambert,
    PhongBlinn,
    CookTorrance,
}
