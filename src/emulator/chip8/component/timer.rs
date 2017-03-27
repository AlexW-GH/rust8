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

    pub fn tick_down(&mut self) {
        if self.clock > 0 {
            self.clock -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;

    #[test]
    fn tick_down_success() {
        let mut under_test: Timer = Timer { clock: 5 };
        under_test.tick_down();
        assert!(under_test.get_value() == 4);
    }

    #[test]
    fn tick_down_no_tick_at_zero() {
        let mut under_test: Timer = Timer { clock: 0 };
        under_test.tick_down();
        assert!(under_test.get_value() == 0);
    }
}
