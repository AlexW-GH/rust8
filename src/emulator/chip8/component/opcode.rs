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