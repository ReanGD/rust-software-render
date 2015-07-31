use sdl2;

struct Fps {
    pub fps: f32,
    counter_max: u32,
    counter: u32,
    tick_start: u32,
}

impl Fps {
    pub fn new(counter_max: u32) -> Fps {
        Fps {
            fps: 0.0_f32,
            counter_max: counter_max,
            counter: 0,
            tick_start: 0,
        }
    }

    pub fn start(&mut self) {
        self.tick_start = sdl2::timer::get_ticks();
        self.counter = 0;
        self.fps = 0.0_f32;
    }

    pub fn update(&mut self) -> bool {
        self.counter += 1;
        if self.counter == self.counter_max {
            let cur_tick = sdl2::timer::get_ticks();
            let dt = (cur_tick - self.tick_start) as f32;
            self.tick_start = cur_tick;
            self.fps = (self.counter_max * 1000) as f32 / dt;
            self.counter = 0;
            true
        } else {
            false
        }
    }
}

pub struct Device {
    context: sdl2::Sdl,
    renderer: sdl2::render::Renderer<'static>,
    texture: sdl2::render::Texture,
    fps: Fps,
    pub cbuffer: Vec<u32>,
    pub y_size: usize,
    pub x_size: usize,
}

impl Device {
    pub fn new(title: &str, width: u32, height: u32) -> Device {
        let context = sdl2::init().video().unwrap();

        let window = context.window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();

        let format = sdl2::pixels::PixelFormatEnum::RGBX8888;
        let texture = renderer.create_texture_streaming(format, (width, height)).unwrap();
        let size = (width as usize)*(height as usize);
        let cbuffer = vec![0; size];

        let mut fps = Fps::new(5);
        fps.start();

        Device {
            context: context,
            renderer: renderer,
            texture: texture,
            fps: fps,
            cbuffer: cbuffer,
            y_size: height as usize,
            x_size: width as usize,
        }
    }

    pub fn set_title(&mut self, title: &str) {
        let mut props = self.renderer.window_properties(&self.context).unwrap();
        props.set_title(&title);
    }

    pub fn draw_fps(&mut self) {
        if self.fps.update() {
            let fps = format!("fps={:.1}", self.fps.fps);
            self.set_title(&fps);
        }
    }

    pub fn draw(&mut self) {
        let cbuffer = &self.cbuffer;
        let y_size = self.y_size;
        let x_size = self.x_size;
        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in (0..y_size) {
                for x in (0..x_size) {
                    let offset = y*pitch + x*4;
                    let c = cbuffer[(y_size - y - 1)*x_size + x];
                    
                    buffer[offset + 0] = 0            as u8;
                    buffer[offset + 1] = c            as u8;
                    buffer[offset + 2] = (c >> (8*1)) as u8;
                    buffer[offset + 3] = (c >> (8*2)) as u8;
                }
            }
        }).unwrap();
        self.renderer.clear();
        let rect = sdl2::rect::Rect::new_unwrap(0, 0, x_size as u32, y_size as u32);
        self.renderer.copy(&self.texture, None, Some(rect));
        self.renderer.present();
    }

    pub fn clear(&mut self, color: u32) {
        for y in (0..self.y_size) {
            for x in (0..self.x_size) {
                self.cbuffer[y*self.x_size+x] = color;
            }
        }
    }

    pub fn keyboard(&mut self) -> bool {
        let mut is_continue = true;
        for event in self.context.event_pump().poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    is_continue = false
                },
                _ => {}
            }
        }
        is_continue
    }
}
