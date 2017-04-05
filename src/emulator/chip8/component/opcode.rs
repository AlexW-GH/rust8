use std::fmt::{self, Formatter, Display};

pub struct Opcode {
    opcode: u16,
    nibbles: (u8, u8, u8, u8),
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        Opcode {
            opcode: opcode,
            nibbles: ((opcode >> 12) as u8 & 0xF, (opcode >> 8) as u8 & 0xF, (opcode >> 4) as u8 & 0xF, (opcode & 0xF) as u8)
        }
    }

    pub fn as_nibbles(&self) -> &(u8, u8, u8, u8) {
        &self.nibbles
    }

    pub fn as_masked(&self, bitmask: u16) -> u16 {
        self.as_int() & bitmask
    }

    pub fn as_int(&self) -> u16 {
        self.opcode
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let opnibbles = self.as_nibbles();
        write!(f, "0x{:X}{:X}{:X}{:X}", opnibbles.0, opnibbles.1, opnibbles.2, opnibbles.3)
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[test]
    fn as_nibbles_min() {
        let under_test = Opcode::new(0x0000);
        let result = under_test.as_nibbles();
        assert!(result.0 == 0x0);
        assert!(result.1 == 0x0);
        assert!(result.2 == 0x0);
        assert!(result.3 == 0x0);
    }

    #[test]
    fn as_nibbles_max() {
        let under_test = Opcode::new(0xFFFF);
        let result = under_test.as_nibbles();
        assert!(result.0 == 0xF);
        assert!(result.1 == 0xF);
        assert!(result.2 == 0xF);
        assert!(result.3 == 0xF);
    }

    #[test]
    fn as_nibbles_different() {
        let under_test = Opcode::new(0x8A3C);
        let result = under_test.as_nibbles();
        assert!(result.0 == 0x8);
        assert!(result.1 == 0xA);
        assert!(result.2 == 0x3);
        assert!(result.3 == 0xC);
    }

    #[test]
    fn as_masked_all() {
        let under_test = Opcode::new(0x4567);
        let result = under_test.as_masked(0xFFFF);
        assert!(result == 0x4567);
    }

    #[test]
    fn as_masked_none() {
        let under_test = Opcode::new(0x4567);
        let result = under_test.as_masked(0x0000);
        assert!(result == 0x0000);
    }

    #[test]
    fn as_masked_mixed() {
        let under_test = Opcode::new(0b1011_0111_1000_1101);
        let result = under_test.as_masked(0b1010_1010_1010_1010);
        assert!(result == 0b1010_0010_1000_1000);
    }
}