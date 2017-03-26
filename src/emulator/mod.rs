pub mod window;
pub mod chip8;
pub mod romloader;

pub trait Emulator {
    fn retrieve_screen_pixels(&self) -> &[bool];
    fn retrieve_screen_size(&self) -> (usize, usize);
    fn update(&mut self);
    fn get_name(&self) -> &str;
    fn load(&mut self, game_data: Vec<u8>);
}