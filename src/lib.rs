pub mod emulator;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rand;

pub use emulator::Emulator;
use emulator::chip8::Chip8;

pub fn create_chip8() -> Box<Chip8> {
    Box::new(Chip8::new())
}