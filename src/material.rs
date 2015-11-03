use std;
use cgmath::*;
use std::rc::Rc;
use texture::{Texture, TextureCube};

#[derive(Clone)]
pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub texture: Option<Rc<Texture>>,
    pub texture_cube: Option<Rc<TextureCube>>,
}

impl Material {
    pub fn new() -> Material {
        Material {
            ambient: Vector3::<f32>::zero(),
            diffuse: Vector3::<f32>::zero(),
            specular: Vector3::<f32>::zero(),
            texture: None,
            texture_cube: None,
        }
    }

    pub fn create_texture(&mut self, path: &std::path::Path) -> Result<(), String> {
        self.texture = Some(Rc::new(try!(Texture::new(path))));

        Ok(())
    }

    pub fn add_texture_cube(&mut self, texture: Rc<TextureCube>) {
        self.texture_cube = Some(texture);
        }
}
