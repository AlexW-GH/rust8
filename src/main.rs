mod emulator;

#[macro_use]
extern crate log;
extern crate log4rs;

use self::emulator::chip8::Chip8;
use self::emulator::window::App;
use self::emulator::Emulator;


fn main() {
    log4rs::init_file("src/config/log4rs.yml", Default::default()).unwrap();
    let mut emulator: Box<Chip8> = Box::new(Default::default());
    emulator.test_setup();
    let mut app: App = App::new(emulator);
    app.run();
}

struct Testemu {
    height: usize,
    width: usize,
    count: usize,
    screen: Vec<bool>
}

impl Default for Testemu {
    fn default() -> Self {
        Self { height: 32, width: 64, count: 0, screen: vec![false; 32 * 64] }
    }
}