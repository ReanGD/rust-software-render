use sdl2_sys;
use libc::c_char;

extern "C" {
    pub fn IMG_Load(file: *const c_char) -> *mut sdl2_sys::surface::SDL_Surface;
}
