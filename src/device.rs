use std;
use sdl2;
use time;
use sdl2_image;

struct Fps {
    pub fps: f32,
    pub mega_tps: u32,
    counter_max: u32,
    counter: u32,
    triangle_counter: u32,
    time_start: u64,
}

impl Fps {
    pub fn new(counter_max: u32) -> Fps {
        Fps {
            fps: 0.0_f32,
            mega_tps: 0,
            counter_max: counter_max,
            counter: 0,
            triangle_counter: 0,
            time_start: 0,
        }
    }

    pub fn start(&mut self) {
        self.time_start = time::precise_time_ns();
        self.counter = 0;
        self.triangle_counter = 0;
        self.fps = 0.0_f32;
        self.mega_tps = 0;
    }

    pub fn update(&mut self, cnt_triangle: u32) -> bool {
        self.counter += 1;
        self.triangle_counter += cnt_triangle;
        if self.counter == self.counter_max {
            let cur_time = time::precise_time_ns();
            let dt = ((cur_time - self.time_start) / 1000000) as f32;
            self.time_start = cur_time;
            self.fps = (self.counter_max * 1000) as f32 / dt;
            self.mega_tps = self.triangle_counter / (dt as u32);
            self.counter = 0;
            self.triangle_counter = 0;
            true
        } else {
            false
        }
    }
}

pub struct Device {
    events: sdl2::EventPump,
    renderer: sdl2::render::Renderer<'static>,
    texture: sdl2::render::Texture,
    fps: Fps,
    pub cbuffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    pub y_size: usize,
    pub x_size: usize,
}

impl Device {
    pub fn new(title: &str, width: u32, height: u32) -> Device {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG | sdl2_image::INIT_TIF);
        let events = context.event_pump().unwrap();

        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();

        let format = sdl2::pixels::PixelFormatEnum::ARGB8888;
        let texture = renderer.create_texture_streaming(format, (width, height)).unwrap();
        let size = (width as usize)*(height as usize);
        let cbuffer = vec![0; size];
        let zbuffer = vec![0.0_f32; size];

        let mut fps = Fps::new(10);
        fps.start();

        Device {
            events: events,
            renderer: renderer,
            texture: texture,
            fps: fps,
            cbuffer: cbuffer,
            zbuffer: zbuffer,
            y_size: height as usize,
            x_size: width as usize,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        let mut window = self.renderer.window_mut().unwrap();
        window.set_title(&title);
    }

    pub fn update_fps(&mut self, cnt_triangle: u32) {
        if self.fps.update(cnt_triangle) {
            let title = format!("fps={:.1}, tps={} 000", self.fps.fps, self.fps.mega_tps as u32);
            self.set_title(&title);
        }
    }

    pub fn present(&mut self) {
        let cbuffer = &self.cbuffer;
        let y_size = self.y_size;
        let x_size = self.x_size;
        self.texture.with_lock(None, |buffer: &mut [u8], _: usize| {
            unsafe {
                std::ptr::copy_nonoverlapping::<u8>(cbuffer.as_ptr() as *const u8, buffer.as_mut_ptr(), buffer.len());
            }
        }).unwrap();
        // self.renderer.clear();
        let rect = sdl2::rect::Rect::new_unwrap(0, 0, x_size as u32, y_size as u32);
        self.renderer.copy_ex(&self.texture, None, Some(rect), 0.0_f64, None, (false, true));
        self.renderer.present();
    }

    pub fn clear(&mut self, color: u32) {
        for val in &mut self.cbuffer {
            *val = color;
        }
        for val in &mut self.zbuffer {
            *val = 0.0_f32;
        }
    }

    pub fn keyboard(&mut self) -> bool {
        let mut is_continue = true;

        for event in self.events.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    is_continue = false
                },
                _ => {}
            }
        }
        
        is_continue
    }

    pub fn exit(&self) {
        sdl2_image::quit();
    }
    
}
