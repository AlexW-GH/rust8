mod emulator;

#[macro_use]
extern crate log;
extern crate log4rs;

use self::emulator::chip8::Chip8;
use self::emulator::window::App;

fn main() {
    log4rs::init_file("src/config/log4rs.yml", Default::default()).unwrap();
    let mut emulator: Box<Chip8> = Box::new(Chip8::new());
    emulator.test_setup();
    let mut app: App = App::new(emulator);
    app.run();
}