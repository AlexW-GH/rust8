#[derive(Default)]
pub struct Input {
    pressed_keys: u16,
}

impl Input {
    pub fn is_pressed(&self, key: u8) -> bool {
        (self.pressed_keys & (0b1 << key)) > 0
    }

    pub fn press_key(&mut self, key: u16) {
        self.pressed_keys |= key
    }

    pub fn release_key(&mut self, key: u16) {
        self.pressed_keys &= !key
    }

    pub fn get_any_pressed_key(&self) -> Option<u8> {
        for i in 0..16 {
            if (self.pressed_keys & (1 << i)) > 0 {
                return Some(i)
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn is_pressed_no_key() {
        let under_test: Input = Input { pressed_keys: 0b0000_0000_0000_0000 };
        let mut result: bool = false;
        for i in 0..16 {
            result |= under_test.is_pressed(i);
        }
        assert!(!result);
    }

    #[test]
    fn is_pressed_key_0() {
        let under_test: Input = Input { pressed_keys: 0b0000_0000_0000_0001 };
        let result: bool = under_test.is_pressed(0x0);
        assert!(result);
    }

    #[test]
    fn is_pressed_key_15() {
        let under_test: Input = Input { pressed_keys: 0b1000_0000_0000_0000 };
        let result: bool = under_test.is_pressed(0xF);
        assert!(result);
    }

    #[test]
    fn press_key_0() {
        let mut under_test: Input = Input { pressed_keys: 0b0000_0000_0000_0000 };
        under_test.press_key(0b0000_0000_0000_0001);
        let result: bool = under_test.is_pressed(0x0);
        assert!(result);
    }

    #[test]
    fn press_key_15() {
        let mut under_test: Input = Input { pressed_keys: 0b0000_0000_0000_0000 };
        under_test.press_key(0b1000_0000_0000_0000);
        let result: bool = under_test.is_pressed(0xF);
        assert!(result);
    }

    #[test]
    fn release_key_0() {
        let mut under_test: Input = Input { pressed_keys: 0b0000_0000_0000_0001 };
        under_test.release_key(0b0000_0000_0000_0001);
        let result: bool = !under_test.is_pressed(0x0);
        assert!(result);
    }

    #[test]
    fn release_key_15() {
        let mut under_test: Input = Input { pressed_keys: 0b1000_0000_0000_0000 };
        under_test.release_key(0b1000_0000_0000_0000);
        let result: bool = !under_test.is_pressed(0xF);
        assert!(result);
    }
}

