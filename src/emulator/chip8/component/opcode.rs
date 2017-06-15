use std::fmt::{self, Formatter, Display};

#[derive(PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Command {
    CLS,
    //00E0
    RET,
    //00EE
    SYS,
    //0nnn
    JMP,
    //1nnn
    CALL,
    //2nnn
    SE,
    //3xkk
    SNE,
    //4xkk
    CPSE,
    //5xy0
    LD,
    //6xkk
    ADDI,
    //7xkk
    CP,
    //8xy0
    OR,
    //8xy1
    AND,
    //8xy2
    XOR,
    //8xy3
    ADD,
    //8xy4
    SUB,
    //8xy5
    SHR,
    //8xy6
    SUBN,
    //8xy7
    SHL,
    //8xyE
    SNER,
    //9xy0
    LDI,
    //Annn
    JMPI,
    //Bnnn
    RND,
    //Cxkk
    DRW,
    //Dxyn
    SKPK,
    //Ex9E
    SKPNK,
    //ExA1
    LDDT,
    //Fx07
    WLDK,
    //Fx0A
    SDTR,
    //Fx15
    SSTR,
    //Fx18
    ADDIR,
    //Fx1E
    LDSPR,
    //Fx29
    BCD,
    //Fx33
    STOR,
    //Fx55
    READ,
    //Fx65
    ERR,
    // Error
}

pub struct Opcode {
    opcode: u16,
    nibbles: (u8, u8, u8, u8),
    command: Command
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        let mut opcode = Opcode {
            opcode: opcode,
            nibbles: ((opcode >> 12) as u8 & 0xF, (opcode >> 8) as u8 & 0xF, (opcode >> 4) as u8 & 0xF, (opcode & 0xF) as u8),
            command: Command::ERR
        };
        opcode.decode();
        opcode
    }

    pub fn as_nibbles(&self) -> (u8, u8, u8, u8) {
        self.nibbles
    }

    pub fn as_masked(&self, bitmask: u16) -> u16 {
        self.as_int() & bitmask
    }

    pub fn as_int(&self) -> u16 {
        self.opcode
    }

    pub fn as_asm(&self) -> Command {
        self.command
    }

    fn decode(&mut self) {
        self.command = match self.nibbles.0 {
            0x0 => Opcode::retrieve_0_opcodes(&self.nibbles),
            0x1 => Command::JMP,
            0x2 => Command::CALL,
            0x3 => Command::SE,
            0x4 => Command::SNE,
            0x5 => Opcode::retrieve_5_opcodes(&self.nibbles),
            0x6 => Command::LD,
            0x7 => Command::ADDI,
            0x8 => Opcode::retrieve_8_opcodes(&self.nibbles),
            0x9 => Command::SNER,
            0xA => Command::LDI,
            0xB => Command::JMPI,
            0xC => Command::RND,
            0xD => Command::DRW,
            0xE => Opcode::retrieve_e_opcodes(&self.nibbles),
            0xF => Opcode::retrieve_f_opcodes(&self.nibbles),
            _ => Command::ERR
        }
    }

    fn retrieve_0_opcodes(nibbles: &(u8, u8, u8, u8)) -> Command {
        match nibbles.3 {
            0x0 => Command::CLS,
            0xE => Command::RET,
            _ => Command::ERR
        }
    }

    fn retrieve_5_opcodes(nibbles: &(u8, u8, u8, u8)) -> Command {
        match nibbles.3 {
            0x0 => Command::CPSE,
            _ => Command::ERR
        }
    }

    fn retrieve_8_opcodes(nibbles: &(u8, u8, u8, u8)) -> Command {
        match nibbles.3 {
            0x0 => Command::CP,
            0x1 => Command::OR,
            0x2 => Command::AND,
            0x3 => Command::XOR,
            0x4 => Command::ADD,
            0x5 => Command::SUB,
            0x6 => Command::SHR,
            0x7 => Command::SUBN,
            0xE => Command::SHL,
            _ => Command::ERR
        }
    }

    fn retrieve_e_opcodes(nibbles: &(u8, u8, u8, u8)) -> Command {
        match nibbles.2 {
            0x9 => match nibbles.3 {
                0xE => Command::SKPK,
                _ => Command::ERR
            },
            0xA => match nibbles.3 {
                0x1 => Command::SKPNK,
                _ => Command::ERR
            },
            _ => Command::ERR
        }
    }

    fn retrieve_f_opcodes(nibbles: &(u8, u8, u8, u8)) -> Command {
        match nibbles.2 {
            0x0 => match nibbles.3 {
                0x7 => Command::LDDT,
                0xA => Command::WLDK,
                _ => Command::ERR
            },
            0x1 => match nibbles.3 {
                0x5 => Command::SDTR,
                0x8 => Command::SSTR,
                0xE => Command::ADDIR,
                _ => Command::ERR
            },
            0x2 => match nibbles.3 {
                0x9 => Command::LDSPR,
                _ => Command::ERR
            },
            0x3 => match nibbles.3 {
                0x3 => Command::BCD,
                _ => Command::ERR
            },
            0x5 => match nibbles.3 {
                0x5 => Command::STOR,
                _ => Command::ERR
            },
            0x6 => match nibbles.3 {
                0x5 => Command::READ,
                _ => Command::ERR
            },
            _ => Command::ERR
        }
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