use cgmath::*;

pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub ambient_intensity: f32,
}

impl Material {
    pub fn new(ambient: Vector3<f32>, diffuse: Vector3<f32>, specular: Vector3<f32>, ambient_intensity: f32) -> Material {
        Material {
            ambient: ambient,
            diffuse: diffuse,
            specular: specular,
            ambient_intensity: ambient_intensity,
        }
    }

    fn calc_ambient_intensity(ambient: &Vector3<f32>, diffuse: &Vector3<f32>) -> f32 {
        (0.212671_f32 * ambient.x + 0.715160_f32 * ambient.y + 0.072169_f32 * ambient.z) /
            (0.212671_f32 * diffuse.x + 0.715160_f32 * diffuse.y + 0.072169_f32 * diffuse.z)
    }

    pub fn gold() -> Material {
        let ambient = Vector3::new(63.059_f32, 50.783_f32, 18.998_f32);
        let diffuse = Vector3::new(191.668_f32, 154.652_f32, 57.752_f32);
        let specular = Vector3::new(160.212_f32, 141.73_f32, 93.347_f32);
        Material::new(ambient, diffuse, specular, Material::calc_ambient_intensity(&ambient, &diffuse))
    }

    #[allow(dead_code)]
    pub fn silver() -> Material {
        let ambient = Vector3::new(49.024_f32, 49.024_f32, 49.024_f32);
        let diffuse = Vector3::new(129.423_f32, 129.423_f32, 129.423_f32);
        let specular = Vector3::new(129.61_f32, 129.61_f32, 129.61_f32);
        Material::new(ambient, diffuse, specular, Material::calc_ambient_intensity(&ambient, &diffuse))
    }

    pub fn monster_skin() -> Material {
        let ambient = Vector3::new(249.0_f32, 202.0_f32, 104.0_f32);
        let diffuse = Vector3::new(249.0_f32, 202.0_f32, 104.0_f32);
        let specular = Vector3::new(249.0_f32, 202.0_f32, 104.0_f32);
        Material::new(ambient, diffuse, specular, Material::calc_ambient_intensity(&ambient, &diffuse))
    }
}
