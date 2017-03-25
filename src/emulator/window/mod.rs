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
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
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

    fn update(&mut self, args: &UpdateArgs) {
        self.update_time += args.dt;
        if self.update_time > UPDATE_LIMIT {
            self.emulator.update();
            self.update_time -= UPDATE_LIMIT;
        }

    }
}