pub mod chip8;

pub trait Emulator {
    fn retrieve_screen_pixels(&self) -> &[bool];
    fn retrieve_screen_size(&self) -> (usize, usize);
    fn update(&mut self);
    fn get_name(&self) -> &str;
    fn load(&mut self, game_data: Vec<u8>);
    fn press_key(&mut self, key: u16);
    fn release_key(&mut self, key: u16);
    fn needs_redraw(&mut self) -> bool;
}