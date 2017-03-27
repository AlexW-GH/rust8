use std::fmt::{self, Formatter, Display};

pub struct Opcode(pub u16);

impl Opcode {
    pub fn as_tuple(&self) -> (u8, u8, u8, u8) {
        ((self.as_int() >> 12) as u8 & 0xF, (self.as_int() >> 8) as u8 & 0xF, (self.as_int() >> 4) as u8 & 0xF, (self.as_int() & 0xF) as u8)
    }

    pub fn as_masked(&self, bitmask: u16) -> u16 {
        self.as_int() & bitmask
    }

    pub fn as_int(&self) -> u16 {
        self.0
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let opcode = self.as_tuple();
        write!(f, "0x{:X}{:X}{:X}{:X}", opcode.0, opcode.1, opcode.2, opcode.3)
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[test]
    fn as_tuple_min() {
        let under_test = Opcode(0x0000);
        let result = under_test.as_tuple();
        assert!(result.0 == 0x0);
        assert!(result.1 == 0x0);
        assert!(result.2 == 0x0);
        assert!(result.3 == 0x0);
    }

    #[test]
    fn as_tuple_max() {
        let under_test = Opcode(0xFFFF);
        let result = under_test.as_tuple();
        assert!(result.0 == 0xF);
        assert!(result.1 == 0xF);
        assert!(result.2 == 0xF);
        assert!(result.3 == 0xF);
    }

    #[test]
    fn as_tuple_different() {
        let under_test = Opcode(0x8A3C);
        let result = under_test.as_tuple();
        assert!(result.0 == 0x8);
        assert!(result.1 == 0xA);
        assert!(result.2 == 0x3);
        assert!(result.3 == 0xC);
    }

    #[test]
    fn as_masked_all() {
        let under_test = Opcode(0x4567);
        let result = under_test.as_masked(0xFFFF);
        assert!(result == 0x4567);
    }

    #[test]
    fn as_masked_none() {
        let under_test = Opcode(0x4567);
        let result = under_test.as_masked(0x0000);
        assert!(result == 0x0000);
    }

    #[test]
    fn as_masked_mixed() {
        let under_test = Opcode(0b1011_0111_1000_1101);
        let result = under_test.as_masked(0b1010_1010_1010_1010);
        assert!(result == 0b1010_0010_1000_1000);
    }
}