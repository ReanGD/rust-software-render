use std;
use sdl2;
use cgmath::*;
use sdl2::surface::Surface;
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
    pub fn new(filename: &str) -> Result<Texture, String> {
        let fullpath = try!(get_full_path(filename));
        let surface_load: Surface;
        unsafe {
            let raw = IMG_Load(std::ffi::CString::new(fullpath).unwrap().as_ptr());
            if (raw as *mut ()).is_null() {
                return Err(sdl2::get_error());
            } else {
                surface_load = Surface::from_ll(raw);
            }
        }
        let standart = try!(Surface::new(1, 1, sdl2::pixels::PixelFormatEnum::ARGB8888));
        let mut surface: Surface = try!(surface_load.as_ref().convert(&standart.as_ref().get_pixel_format()));

        let size_x = surface.as_ref().get_width() as usize;
        let size_y = surface.as_ref().get_height() as usize;
        let mut data = Vec::<Vector3<f32>>::with_capacity(size_x * size_y);
        
        let data_u8: &[u8] = match surface.as_mut().without_lock() {
            Some(v) => v,
            None => return Err(format!("can't lock texture {}", filename))
        };

        for x in 0..size_x {
            for y in 0..size_y {
                data.push(Vector3::new(
                    data_u8[(y * size_x + size_x - x - 1) * 4 + 0] as f32,
                    data_u8[(y * size_x + size_x - x - 1) * 4 + 1] as f32,
                    data_u8[(y * size_x + size_x - x - 1) * 4 + 2] as f32));
            }
        }
        
        Ok(Texture {
            size_x: size_x as usize,
            size_y: size_y as usize,
            data: data,
        })
    }
}
