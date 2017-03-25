#[derive(Default)]
pub struct Timer {
    clock: u8
}

impl Timer {
    pub fn get_value(&self) -> u8 {
        self.clock
    }

    pub fn set_value(&mut self, value: u8) {
        self.clock = value
    }
}
