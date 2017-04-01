extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{GlGraphics, OpenGL};

use emulator::Emulator;

const UPDATE_LIMIT: f64 = 1.0 / 60.0;


pub struct App {
    gl: GlGraphics,
    window: Window,
    emulator: Box<Emulator>,
    update_time: f64,
}

impl App {
    pub fn new(selected_emulator: Box<Emulator>) -> App {
        let opengl = OpenGL::V3_2;

        let window_gl: Window = WindowSettings::new(
            selected_emulator.get_name(),
            [200, 200]
        )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        App {
            gl: GlGraphics::new(opengl),
            window: window_gl,
            emulator: selected_emulator,
            update_time: 0.0,
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(event) = events.next(&mut self.window) {
            if let Some(render) = event.render_args() {
                self.render(&render);
            }

            if let Some(Button::Keyboard(key)) = event.press_args() {
                self.handle_key_press(&key);
            }

            if let Some(Button::Keyboard(key)) = event.release_args() {
                self.handle_key_release(&key);
            }

            if let Some(update) = event.update_args() {
                self.update(&update);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        let (screen_width, screen_height) = self.emulator.retrieve_screen_size();
        let (pixel_width, pixel_height) = ((args.width / screen_width as u32) as u32, (args.height / screen_height as u32) as u32);
        let center_width: f64 = ((args.width - ((screen_width as u32) * pixel_width)) / 2) as f64;
        let center_height: f64 = ((args.height - ((screen_height as u32) * pixel_height)) / 2) as f64;

        let pixel = rectangle::rectangle_by_corners(0.0, 0.0, pixel_width as f64, pixel_height as f64);
        let iterator = self.emulator.retrieve_screen_pixels().into_iter();
        self.gl.draw(args.viewport(), |c, gl| {

            clear(color::hex("888888"), gl);

            for (index, value) in iterator.enumerate() {
                let draw_color = if *value { color::WHITE } else { color::BLACK };
                let pos_x = (((index % screen_width) as u32) * pixel_width) as f64;
                let pos_y = (((index / screen_width) as f64).floor() * pixel_height as f64) as f64;

                let transform = c.transform.trans(pos_x + center_width, pos_y + center_height);

                rectangle(draw_color, pixel, transform, gl);
            }
        });
    }

    fn handle_key_press(&mut self, key: &Key) {
        if let Some(key_value) = self.handle_key(key) {
            self.emulator.press_key(0b1 << key_value)
        }
    }

    fn handle_key_release(&mut self, key: &Key) {
        if let Some(key_value) = self.handle_key(key) {
            self.emulator.release_key(0b1 << key_value)
        }
    }

    fn handle_key(&self, key: &Key) -> Option<u16> {
        match *key {
            Key::Space => Option::Some(0x0),
            Key::Y => Option::Some(0x1),
            Key::X => Option::Some(0x2),
            Key::C => Option::Some(0x3),
            Key::A => Option::Some(0x4),
            Key::S => Option::Some(0x5),
            Key::D => Option::Some(0x6),
            Key::Q => Option::Some(0x7),
            Key::W => Option::Some(0x8),
            Key::E => Option::Some(0x9),
            Key::D1 => Option::Some(0xA),
            Key::D2 => Option::Some(0xB),
            Key::D3 => Option::Some(0xC),
            Key::V => Option::Some(0xD),
            Key::F => Option::Some(0xE),
            Key::R => Option::Some(0xF),
            _ => Option::None,
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.update_time += args.dt;
        if self.update_time > UPDATE_LIMIT {
            self.emulator.update();
            self.update_time -= UPDATE_LIMIT;
        }

    }
}