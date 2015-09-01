use std;
use cgmath::*;
use std::rc::Rc;
use texture::Texture;

#[derive(Clone)]
pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub ambient_intensity: f32,
    pub texture: Option<Rc<Texture>>,
}

impl Material {
    pub fn new() -> Material {
        Material {
            ambient: Vector3::<f32>::zero(),
            diffuse: Vector3::<f32>::zero(),
            specular: Vector3::<f32>::zero(),
            ambient_intensity: 0.0_f32,
            texture: None,
        }
    }

    pub fn texture_from_dir(&mut self, dir: &std::path::PathBuf, filename: &str) -> Result<(), String> {
        self.texture = Some(Rc::new(try!(Texture::from_dir(dir, filename))));

        Ok(())
    }

    #[allow(dead_code)]
    pub fn texture_from_def(&mut self, filename: &str) -> Result<(), String> {
        self.texture = Some(Rc::new(try!(Texture::from_def(filename))));

        Ok(())
    }


    pub fn calc_ambient_intensity(&mut self) {
        const K_VEC: Vector3<f32> = Vector3{x: 0.212671_f32, y: 0.715160_f32, z: 0.072169_f32};
        let ambient = self.ambient.mul_v(&K_VEC);
        let diffuse = self.diffuse.mul_v(&K_VEC);
        self.ambient_intensity = (ambient.x + ambient.y + ambient.z) / (diffuse.x + diffuse.y + diffuse.z);
    }

    #[allow(dead_code)]
    pub fn gold(&mut self) {
        self.ambient = Vector3::new(63.059_f32, 50.783_f32, 18.998_f32);
        self.diffuse = Vector3::new(191.668_f32, 154.652_f32, 57.752_f32);
        self.specular = Vector3::new(160.212_f32, 141.73_f32, 93.347_f32);
        self.calc_ambient_intensity();
    }

    #[allow(dead_code)]
    pub fn silver(&mut self) {
        self.ambient = Vector3::new(49.024_f32, 49.024_f32, 49.024_f32);
        self.diffuse = Vector3::new(129.423_f32, 129.423_f32, 129.423_f32);
        self.specular = Vector3::new(129.61_f32, 129.61_f32, 129.61_f32);
        self.calc_ambient_intensity();
    }

    #[allow(dead_code)]
    pub fn monster_skin(&mut self) {
        self.ambient = Vector3::new(249.0_f32, 202.0_f32, 104.0_f32);
        self.diffuse = Vector3::new(249.0_f32, 202.0_f32, 104.0_f32);
        self.specular = Vector3::new(249.0_f32, 202.0_f32, 104.0_f32);
        self.calc_ambient_intensity();
    }
}
