#[derive(Default)]
pub struct Input {
    pressed_keys: [bool; 0xF],
}

impl Input {
    pub fn is_pressed(&self, key: u8) -> bool {
        self.pressed_keys[key as usize]
    }

    pub fn get_key(&self) -> u8 {
        error!("get_key not yet implemented!");
        0 //TODO: implement me!
    }
}

