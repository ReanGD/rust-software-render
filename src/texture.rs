use std;
use sdl2;
use cgmath::*;
use std::rc::Rc;
use std::path::Path;
use sdl2_image::LoadSurface;

pub struct Surface {
    pub size_x: usize,
    pub size_y: usize,
    pub data: Vec<Vector3<f32>>,
}

pub struct Texture {
    levels: Vec<Rc<Surface>>,
    pub size: Vector2<f32>,
}

impl Surface {
    fn new(size_x: usize, size_y: usize) -> Surface {
        Surface {
            size_x: size_x,
            size_y: size_y,
            data: Vec::<Vector3<f32>>::with_capacity(size_x * size_y),
        }
    }

    #[allow(dead_code)]
    pub fn tex_2d(&self, tex: Vector2<f32>) -> Vector3<f32>
    {
        let x = (tex.x * (self.size_x as f32)) as usize % self.size_x;
        let y = (tex.y * (self.size_y as f32)) as usize % self.size_y;
        self.data[y * self.size_x + x]
    }

    #[allow(dead_code)]
    pub fn tex_2d_bilinear(&self, tex: Vector2<f32>) -> Vector3<f32>
    {
        let x = tex.x * (self.size_x as f32);
        let y = tex.y * (self.size_y as f32);

        let mut x_int = x as usize;
        let mut y_int = y as usize;

        let dx = x - (x_int as f32);
        let dy = y - (y_int as f32);

        x_int %= self.size_x;
        y_int %= self.size_y;

        let add_x = if x_int + 1 == self.size_x {0} else {1};
        let add_y = if y_int + 1 == self.size_y {0} else {self.size_x};

        let ind = y_int * self.size_x + x_int;

        self.data[ind].mul_s(1.0_f32 - dx).
            add_v(&self.data[ind + add_x].mul_s(dx)).
            mul_s(1.0_f32 - dy).
            add_v(&self.data[ind + add_y].mul_s(1.0_f32 - dx).
                  add_v(&self.data[ind + add_x + add_y].mul_s(dx)).
                  mul_s(dy))
    }
}

impl Texture {
    pub fn new(path: &Path) -> Result<Texture, String> {
        let surface = try!(Texture::load_surface(path));

        let size_x = surface.as_ref().width() as usize;
        let size_y = surface.as_ref().height() as usize;
        let data_u8: &[u8] = match surface.as_ref().without_lock() {
            Some(v) => v,
            None => return Err(format!("can't lock surface for texture {}", path.display()))
        };

        let mut lvl0 = Surface::new(size_x, size_y);
        for y in 0..size_y {
            for x in 0..size_x {
                let ind = ((size_y - y - 1) * size_x + x) * 4;
                lvl0.data.push(Vector3::new(
                    data_u8[ind + 2] as f32,
                    data_u8[ind + 1] as f32,
                    data_u8[ind + 0] as f32));
            }}

        let mut this = Texture {
            levels: vec![Rc::new(lvl0)],
            size: Vector2::new(size_x as f32, size_y as f32),
        };
        this.gen_mipmap();

        Ok(this)
    }

    pub fn get_surface(&self, mip_lvl: usize) -> Rc<Surface> {
        self.levels[std::cmp::min(mip_lvl, self.levels.len() - 1)].clone()
    }

    fn load_surface<'a>(path: &Path) -> Result<sdl2::surface::Surface<'a>, String> {
        println!("load texture: \"{}\"", path.display());

        let standart =
            match sdl2::surface::Surface::new(1, 1, sdl2::pixels::PixelFormatEnum::ARGB8888) {
                Ok(v) => v,
                Err(e) => return Err(format!("can't create standart surface for texture \"{}\", error = \"{}\"",
                                             path.display(), e))
            };

        let surface_load = match sdl2::surface::Surface::from_file(path) {
            Ok(v) => v,
            Err(e) => return Err(format!("can't load texture \"{}\", error = \"{}\"",
                                         path.display(), e))
        };

        match surface_load.as_ref().convert(&standart.as_ref().pixel_format()) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("can't convert surface for texture \"{}\", error = \"{}\"",
                                         path.display(), e))
        }
    }

    #[inline]
    fn add_color(clr: &mut Vector3<f32>, src: &Surface, x: i32, y: i32, k: u32) -> u32 {
        if x >= 0 && y >= 0 && x < (src.size_x as i32) && y < (src.size_y as i32) {
            let ind = (y as usize) * src.size_x + x as usize;
            clr.add_self_v(&src.data[ind].mul_s(k as f32));
            k
        } else {
            0
        }
    }

    fn gen_mipmap(&mut self) {
        let mut surface_ind = 0;
        let mut size_x = self.levels[surface_ind].size_x as i32;
        let mut size_y = self.levels[surface_ind].size_y as i32;

        while std::cmp::min(size_x, size_y) != 1 {
            let next_size_x = size_x >> 1;
            let next_size_y = size_y >> 1;
            let mut s = Surface::new(next_size_x as usize, next_size_y as usize);
            {
                let src = &(self.levels[surface_ind]);
                let mut dst = &mut s.data;

                // koeff table
                // 1 2 1
                // 2 4 2
                // 1 2 1
                for y in 0..next_size_y {
                    for x in 0..next_size_x {
                        let mut clr = Vector3::zero();
                        let mut cnt: u32 = 0;
                        {
                            let mut f = |add_x: i32, add_y: i32, k: u32| {
                                Texture::add_color(&mut clr, src, x * 2 + add_x, y * 2 + add_y, k)
                            };

                            cnt += f(-1, -1, 1);
                            cnt += f( 0, -1, 2);
                            cnt += f( 1, -1, 1);
                            cnt += f(-1,  0, 2);
                            cnt += f( 0,  0, 4);
                            cnt += f( 1,  0, 2);
                            cnt += f(-1,  1, 1);
                            cnt += f( 0,  1, 2);
                            cnt += f( 1,  1, 1);
                        }
                        dst.push(clr.div_s(cnt as f32));
                    }
                }
            }

            self.levels.push(Rc::new(s));
            size_x = next_size_x;
            size_y = next_size_y;
            surface_ind += 1;
        }
    }
}
