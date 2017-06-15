use std::fmt::{self, Formatter, Display};

#[derive(PartialEq, Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum Command {
    CLS,
    RET,
    SYS(u16),
    JMP(u16),
    CALL(u16),
    SE(u8, u8),
    SNE(u8, u8),
    CPSE(u8, u8),
    LD(u8, u8),
    ADDI(u8, u8),
    CP(u8, u8),
    OR(u8, u8),
    AND(u8, u8),
    XOR(u8, u8),
    ADD(u8, u8),
    SUB(u8, u8),
    SHR(u8),
    SUBN(u8, u8),
    SHL(u8),
    SNER(u8, u8),
    LDI(u16),
    RJMP(u16),
    RND(u8, u8),
    DRW(u8, u8, u8),
    SKPK(u8),
    SKPNK(u8),
    LDDT(u8),
    WLDK(u8),
    SDTR(u8),
    SSTR(u8),
    ADDIR(u8),
    LDSPR(u8),
    BCD(u8),
    STOR(u8),
    READ(u8),
    ERR,
    // Error

}

pub struct Opcode {
    opcode: u16,
    command: Command
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        let mut opcode = Opcode {
            opcode: opcode,
            command: Command::ERR
        };
        opcode.decode();
        opcode
    }

    pub fn as_asm(&self) -> Command {
        self.command
    }

    fn decode(&mut self) {
        let nibbles = self.as_nibbles();
        self.command = match nibbles.0 {
            0x0 => self.retrieve_0_opcodes(nibbles),
            0x1 => Command::JMP(self.as_masked(0x0FFF)),
            0x2 => Command::CALL(self.as_masked(0x0FFF)),
            0x3 => Command::SE(nibbles.1, self.as_masked(0x00FF) as u8),
            0x4 => Command::SNE(nibbles.1, self.as_masked(0x00FF) as u8),
            0x5 => self.retrieve_5_opcodes(nibbles),
            0x6 => Command::LD(nibbles.1, self.as_masked(0x00FF) as u8),
            0x7 => Command::ADDI(nibbles.1, self.as_masked(0x00FF) as u8),
            0x8 => self.retrieve_8_opcodes(nibbles),
            0x9 => self.retrieve_9_opcodes(nibbles),
            0xA => Command::LDI(self.as_masked(0x0FFF)),
            0xB => Command::RJMP(self.as_masked(0x0FFF)),
            0xC => Command::RND(nibbles.1, self.as_masked(0x00FF) as u8),
            0xD => Command::DRW(nibbles.1, nibbles.2, nibbles.3),
            0xE => self.retrieve_e_opcodes(nibbles),
            0xF => self.retrieve_f_opcodes(nibbles),
            _ => Command::ERR
        }
    }

    fn retrieve_0_opcodes(&self, nibbles: (u8, u8, u8, u8)) -> Command {
        match nibbles.3 {
            0x0 => Command::CLS,
            0xE => Command::RET,
            _ => Command::SYS(self.as_masked(self.opcode))
        }
    }

    fn retrieve_5_opcodes(&self, nibbles: (u8, u8, u8, u8)) -> Command {
        match nibbles.3 {
            0x0 => Command::CPSE(nibbles.1, nibbles.2),
            _ => Command::ERR
        }
    }

    fn retrieve_8_opcodes(&self, nibbles: (u8, u8, u8, u8)) -> Command {
        match nibbles.3 {
            0x0 => Command::CP(nibbles.1, nibbles.2),
            0x1 => Command::OR(nibbles.1, nibbles.2),
            0x2 => Command::AND(nibbles.1, nibbles.2),
            0x3 => Command::XOR(nibbles.1, nibbles.2),
            0x4 => Command::ADD(nibbles.1, nibbles.2),
            0x5 => Command::SUB(nibbles.1, nibbles.2),
            0x6 => Command::SHR(nibbles.1),
            0x7 => Command::SUBN(nibbles.1, nibbles.2),
            0xE => Command::SHL(nibbles.1),
            _ => Command::ERR
        }
    }

    fn retrieve_9_opcodes(&self, nibbles: (u8, u8, u8, u8)) -> Command {
        match nibbles.3 {
            0x0 => Command::SNER(nibbles.1, nibbles.2),
            _ => Command::ERR
        }
    }

    fn retrieve_e_opcodes(&self, nibbles: (u8, u8, u8, u8)) -> Command {
        match nibbles.2 {
            0x9 => match nibbles.3 {
                0xE => Command::SKPK(nibbles.1),
                _ => Command::ERR
            },
            0xA => match nibbles.3 {
                0x1 => Command::SKPNK(nibbles.1),
                _ => Command::ERR
            },
            _ => Command::ERR
        }
    }

    fn retrieve_f_opcodes(&self, nibbles: (u8, u8, u8, u8)) -> Command {
        match nibbles.2 {
            0x0 => match nibbles.3 {
                0x7 => Command::LDDT(nibbles.1),
                0xA => Command::WLDK(nibbles.1),
                _ => Command::ERR
            },
            0x1 => match nibbles.3 {
                0x5 => Command::SDTR(nibbles.1),
                0x8 => Command::SSTR(nibbles.1),
                0xE => Command::ADDIR(nibbles.1),
                _ => Command::ERR
            },
            0x2 => match nibbles.3 {
                0x9 => Command::LDSPR(nibbles.1),
                _ => Command::ERR
            },
            0x3 => match nibbles.3 {
                0x3 => Command::BCD(nibbles.1),
                _ => Command::ERR
            },
            0x5 => match nibbles.3 {
                0x5 => Command::STOR(nibbles.1),
                _ => Command::ERR
            },
            0x6 => match nibbles.3 {
                0x5 => Command::READ(nibbles.1),
                _ => Command::ERR
            },
            _ => Command::ERR
        }
    }

    fn as_nibbles(&self) -> (u8, u8, u8, u8) {
        ((self.opcode >> 12) as u8 & 0xF, (self.opcode >> 8) as u8 & 0xF, (self.opcode >> 4) as u8 & 0xF, (self.opcode & 0xF) as u8)
    }

    fn as_masked(&self, bitmask: u16) -> u16 {
        self.opcode & bitmask
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
    use super::Command;

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

    #[test]
    fn identify_cls() {
        let under_test = Opcode::new(0x00D0);
        assert!(under_test.as_asm() == Command::CLS)
    }

    #[test]
    fn identify_ret() {
        let under_test = Opcode::new(0x00EE);
        assert!(under_test.as_asm() == Command::RET)
    }

    #[test]
    fn identify_sys() {
        let under_test = Opcode::new(0x0123);
        assert!(under_test.as_asm() == Command::SYS(0x123))
    }

    #[test]
    fn identify_jmp() {
        let under_test = Opcode::new(0x1123);
        assert!(under_test.as_asm() == Command::JMP(0x123))
    }

    #[test]
    fn identify_call() {
        let under_test = Opcode::new(0x2123);
        assert!(under_test.as_asm() == Command::CALL(0x123))
    }

    #[test]
    fn identify_se() {
        let under_test = Opcode::new(0x3123);
        assert!(under_test.as_asm() == Command::SE(0x1, 0x23))
    }

    #[test]
    fn identify_sne() {
        let under_test = Opcode::new(0x4123);
        assert!(under_test.as_asm() == Command::SNE(0x1, 0x23))
    }

    #[test]
    fn identify_cpse() {
        let under_test = Opcode::new(0x5120);
        assert!(under_test.as_asm() == Command::CPSE(0x1, 0x2))
    }

    #[test]
    fn identify_invalid_5xxx() {
        let under_test = Opcode::new(0x5123);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_ld() {
        let under_test = Opcode::new(0x6123);
        assert!(under_test.as_asm() == Command::LD(0x1, 0x23))
    }

    #[test]
    fn identify_addi() {
        let under_test = Opcode::new(0x7123);
        assert!(under_test.as_asm() == Command::ADDI(0x1, 0x23))
    }

    #[test]
    fn identify_cp() {
        let under_test = Opcode::new(0x8120);
        assert!(under_test.as_asm() == Command::CP(0x1, 0x2))
    }

    #[test]
    fn identify_or() {
        let under_test = Opcode::new(0x8121);
        assert!(under_test.as_asm() == Command::OR(0x1, 0x2))
    }

    #[test]
    fn identify_and() {
        let under_test = Opcode::new(0x8122);
        assert!(under_test.as_asm() == Command::AND(0x1, 0x2))
    }

    #[test]
    fn identify_xor() {
        let under_test = Opcode::new(0x8123);
        assert!(under_test.as_asm() == Command::XOR(0x1, 0x2))
    }

    #[test]
    fn identify_add() {
        let under_test = Opcode::new(0x8124);
        assert!(under_test.as_asm() == Command::ADD(0x1, 0x2))
    }

    #[test]
    fn identify_sub() {
        let under_test = Opcode::new(0x8125);
        assert!(under_test.as_asm() == Command::SUB(0x1, 0x2))
    }

    #[test]
    fn identify_shr() {
        let under_test = Opcode::new(0x8126);
        assert!(under_test.as_asm() == Command::SHR(0x1))
    }

    #[test]
    fn identify_subn() {
        let under_test = Opcode::new(0x8127);
        assert!(under_test.as_asm() == Command::SUBN(0x1, 0x2))
    }

    #[test]
    fn identify_shl() {
        let under_test = Opcode::new(0x812E);
        assert!(under_test.as_asm() == Command::SHL(0x1))
    }

    #[test]
    fn identify_invalid_8xxx() {
        let under_test = Opcode::new(0x8128);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_sner() {
        let under_test = Opcode::new(0x9120);
        assert!(under_test.as_asm() == Command::SNER(0x1, 0x2))
    }

    #[test]
    fn identify_invalid_9xxx() {
        let under_test = Opcode::new(0x9123);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_ldi() {
        let under_test = Opcode::new(0xA123);
        assert!(under_test.as_asm() == Command::LDI(0x123))
    }

    #[test]
    fn identify_rjmp() {
        let under_test = Opcode::new(0xB123);
        assert!(under_test.as_asm() == Command::RJMP(0x123))
    }

    #[test]
    fn identify_rnd() {
        let under_test = Opcode::new(0xC123);
        assert!(under_test.as_asm() == Command::RND(0x1, 0x23))
    }

    #[test]
    fn identify_drw() {
        let under_test = Opcode::new(0xD123);
        assert!(under_test.as_asm() == Command::DRW(0x1, 0x2, 0x3))
    }

    #[test]
    fn identify_skpk() {
        let under_test = Opcode::new(0xE19E);
        assert!(under_test.as_asm() == Command::SKPK(0x1))
    }

    #[test]
    fn identify_skpnk() {
        let under_test = Opcode::new(0xE1A1);
        assert!(under_test.as_asm() == Command::SKPNK(0x1))
    }

    #[test]
    fn identify_invalid_exxx() {
        let under_test = Opcode::new(0xE1B3);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_invalid_ex9x() {
        let under_test = Opcode::new(0xE193);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_invalid_exax() {
        let under_test = Opcode::new(0xE1A3);
        assert!(under_test.as_asm() == Command::ERR)
    }


    #[test]
    fn identify_lddt() {
        let under_test = Opcode::new(0xF107);
        assert!(under_test.as_asm() == Command::LDDT(0x1))
    }

    #[test]
    fn identify_wldk() {
        let under_test = Opcode::new(0xF10A);
        assert!(under_test.as_asm() == Command::WLDK(0x1))
    }

    #[test]
    fn identify_sdtr() {
        let under_test = Opcode::new(0xF115);
        assert!(under_test.as_asm() == Command::SDTR(0x1))
    }

    #[test]
    fn identify_sstr() {
        let under_test = Opcode::new(0xF118);
        assert!(under_test.as_asm() == Command::SSTR(0x1))
    }

    #[test]
    fn identify_addir() {
        let under_test = Opcode::new(0xF11E);
        assert!(under_test.as_asm() == Command::ADDIR(0x1))
    }

    #[test]
    fn identify_ldspr() {
        let under_test = Opcode::new(0xF129);
        assert!(under_test.as_asm() == Command::LDSPR(0x1))
    }

    #[test]
    fn identify_bcd() {
        let under_test = Opcode::new(0xF133);
        assert!(under_test.as_asm() == Command::BCD(0x1))
    }

    #[test]
    fn identify_stor() {
        let under_test = Opcode::new(0xF155);
        assert!(under_test.as_asm() == Command::STOR(0x1))
    }

    #[test]
    fn identify_read() {
        let under_test = Opcode::new(0xF165);
        assert!(under_test.as_asm() == Command::READ(0x1))
    }

    #[test]
    fn identify_invalid_fxxx() {
        let under_test = Opcode::new(0xF177);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_invalid_fx0x() {
        let under_test = Opcode::new(0xF108);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_invalid_fx1x() {
        let under_test = Opcode::new(0xF116);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_invalid_fx2x() {
        let under_test = Opcode::new(0xF123);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_invalid_fx3x() {
        let under_test = Opcode::new(0xF134);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_invalid_fx5x() {
        let under_test = Opcode::new(0xF156);
        assert!(under_test.as_asm() == Command::ERR)
    }

    #[test]
    fn identify_invalid_fx6x() {
        let under_test = Opcode::new(0xF166);
        assert!(under_test.as_asm() == Command::ERR)
    }
}