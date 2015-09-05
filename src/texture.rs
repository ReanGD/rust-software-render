use std;
use sdl2;
use cgmath::*;
use sdl2::surface;
use utils::get_full_path;
use dll_import::IMG_Load;

pub struct Surface {
    pub size_x: usize,
    pub size_y: usize,
    pub data: Vec<Vector3<f32>>,
}

pub struct Texture {
    pub levels: Vec<Surface>,
}

impl Surface {
    fn new(size_x: usize, size_y: usize) -> Surface {
        Surface {
            size_x: size_x,
            size_y: size_y,
            data: Vec::<Vector3<f32>>::with_capacity(size_x * size_y),
        }
    }
}

impl Texture {
    pub fn from_dir(dir: &std::path::PathBuf, filename: &str) -> Result<Texture, String> {
        let mut path = dir.clone();
        path.push(filename);

        let fullpath = match path.as_path().to_str() {
            Some(path) => path.to_string(),
            None => return Err(format!("error get full path for filename {}", filename))
        };

        Ok(try!(Texture::new(&fullpath, filename)))
    }

    #[allow(dead_code)]
    pub fn from_def(filename: &str) -> Result<Texture, String> {
        let fullpath = try!(get_full_path(filename));

        Ok(try!(Texture::new(&fullpath, filename)))
    }

    pub fn new(fullpath: &str, filename: &str) -> Result<Texture, String> {
        let surface = try!(Texture::load_surface(fullpath, filename));

        let size_x = surface.as_ref().width() as usize;
        let size_y = surface.as_ref().height() as usize;
        let data_u8: &[u8] = match surface.as_ref().without_lock() {
            Some(v) => v,
            None => return Err(format!("can't lock surface for texture {}", filename))
        };

        let mut lvl0 = Surface::new(size_x, size_y);
        for ind in 0..size_y * size_x {
            lvl0.data.push(Vector3::new(
                data_u8[ind * 4 + 2] as f32,
                data_u8[ind * 4 + 1] as f32,
                data_u8[ind * 4 + 0] as f32));
        }

        let mut this = Texture {
            levels: vec![lvl0],
        };
        this.gen_mipmap();

        Ok(this)
    }

    fn load_surface<'a>(fullpath: &str, filename: &str) -> Result<surface::Surface<'a>, String> {
        let standart =
            match surface::Surface::new(1, 1, sdl2::pixels::PixelFormatEnum::ARGB8888) {
                Ok(v) => v,
                Err(e) => return Err(format!("can't create standart surface for texture {}, error = {}",
                                             filename, e))
            };

        let surface_load =
            unsafe {
                let raw = IMG_Load(std::ffi::CString::new(fullpath).unwrap().as_ptr());
                if (raw as *mut ()).is_null() {
                    return Err(format!("can't load texture {}, error = {}", filename, sdl2::get_error()));
                } else {
                    surface::Surface::from_ll(raw)
                }
            };

        match surface_load.as_ref().convert(&standart.as_ref().pixel_format()) {
            Ok(v) => Ok(v),
            Err(e) => return Err(format!("can't convert surface for texture {}, error = {}",
                                         filename, e))
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

            self.levels.push(s);
            size_x = next_size_x;
            size_y = next_size_y;
            surface_ind += 1;
        }
    }
}
