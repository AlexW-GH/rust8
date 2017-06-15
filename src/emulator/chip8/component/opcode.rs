use std::fmt::{self, Formatter, Display};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ASM {
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
    ERR
}

impl Display for ASM {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq)]
pub struct Opcode {
    opcode: u16,
    assembler: ASM
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        let mut opcode = Opcode {
            opcode: opcode,
            assembler: ASM::ERR
        };
        opcode.decode();
        opcode
    }

    pub fn from_asm(command: ASM) -> Opcode {
        match command {
            ASM::CLS =>
                Opcode { opcode: 0x00E0, assembler: command },
            ASM::RET =>
                Opcode { opcode: 0x00EE, assembler: command },
            ASM::SYS(address) =>
                Opcode { opcode: 0x0000 | address, assembler: command },
            ASM::JMP(address) =>
                Opcode { opcode: 0x1000 | address, assembler: command },
            ASM::CALL(address) =>
                Opcode { opcode: 0x2000 | address, assembler: command },
            ASM::SE(register, value) =>
                Opcode { opcode: 0x3000 | ((register as u16) << 8) + value as u16, assembler: command },
            ASM::SNE(register, value) =>
                Opcode { opcode: 0x4000 | ((register as u16) << 8) + value as u16, assembler: command },
            ASM::CPSE(register1, register2) =>
                Opcode { opcode: 0x5000 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::LD(register, value) =>
                Opcode { opcode: 0x6000 | ((register as u16) << 8) + value as u16, assembler: command },
            ASM::ADDI(register, value) =>
                Opcode { opcode: 0x7000 | ((register as u16) << 8) + value as u16, assembler: command },
            ASM::CP(register1, register2) =>
                Opcode { opcode: 0x8000 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::OR(register1, register2) =>
                Opcode { opcode: 0x8001 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::AND(register1, register2) =>
                Opcode { opcode: 0x8002 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::XOR(register1, register2) =>
                Opcode { opcode: 0x8003 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::ADD(register1, register2) =>
                Opcode { opcode: 0x8004 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::SUB(register1, register2) =>
                Opcode { opcode: 0x8005 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::SHR(register) =>
                Opcode { opcode: 0x8006 | ((register as u16) << 8), assembler: command },
            ASM::SUBN(register1, register2) =>
                Opcode { opcode: 0x8007 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::SHL(register) =>
                Opcode { opcode: 0x800E | ((register as u16) << 8), assembler: command },
            ASM::SNER(register1, register2) =>
                Opcode { opcode: 0x9000 | (((register1 as u16) << 8) + (register2 << 4) as u16), assembler: command },
            ASM::LDI(address) =>
                Opcode { opcode: 0xA000 | address, assembler: command },
            ASM::RJMP(address) =>
                Opcode { opcode: 0xB000 | address, assembler: command },
            ASM::RND(register, value) =>
                Opcode { opcode: 0xC000 | ((register as u16) << 8) + value as u16, assembler: command },
            ASM::DRW(register_x, register_y, register_h) =>
                Opcode { opcode: 0xD000 | (((register_x as u16) << 8) + ((register_y as u16) << 4) + register_h as u16), assembler: command },
            ASM::SKPK(register) =>
                Opcode { opcode: 0xE09E | ((register as u16) << 8), assembler: command },
            ASM::SKPNK(register) =>
                Opcode { opcode: 0xE0A1 | ((register as u16) << 8), assembler: command },
            ASM::LDDT(register) =>
                Opcode { opcode: 0xF007 | ((register as u16) << 8), assembler: command },
            ASM::WLDK(register) =>
                Opcode { opcode: 0xF00A | ((register as u16) << 8), assembler: command },
            ASM::SDTR(register) =>
                Opcode { opcode: 0xF015 | ((register as u16) << 8), assembler: command },
            ASM::SSTR(register) =>
                Opcode { opcode: 0xF018 | ((register as u16) << 8), assembler: command },
            ASM::ADDIR(register) =>
                Opcode { opcode: 0xF01E | ((register as u16) << 8), assembler: command },
            ASM::LDSPR(register) =>
                Opcode { opcode: 0xF029 | ((register as u16) << 8), assembler: command },
            ASM::BCD(register) =>
                Opcode { opcode: 0xF033 | ((register as u16) << 8), assembler: command },
            ASM::STOR(register) =>
                Opcode { opcode: 0xF055 | ((register as u16) << 8), assembler: command },
            ASM::READ(register) =>
                Opcode { opcode: 0xF065 | ((register as u16) << 8), assembler: command },
            ASM::ERR =>
                Opcode { opcode: 0x0000, assembler: command },
        }
    }

    pub fn as_asm(&self) -> ASM {
        self.assembler
    }

    fn decode(&mut self) {
        let nibbles = self.as_nibbles();
        self.assembler = match nibbles.0 {
            0x0 => self.retrieve_0_opcodes(&nibbles),
            0x1 => ASM::JMP(self.as_masked(0x0FFF)),
            0x2 => ASM::CALL(self.as_masked(0x0FFF)),
            0x3 => ASM::SE(nibbles.1, self.as_masked(0x00FF) as u8),
            0x4 => ASM::SNE(nibbles.1, self.as_masked(0x00FF) as u8),
            0x5 => self.retrieve_5_opcodes(&nibbles),
            0x6 => ASM::LD(nibbles.1, self.as_masked(0x00FF) as u8),
            0x7 => ASM::ADDI(nibbles.1, self.as_masked(0x00FF) as u8),
            0x8 => self.retrieve_8_opcodes(&nibbles),
            0x9 => self.retrieve_9_opcodes(&nibbles),
            0xA => ASM::LDI(self.as_masked(0x0FFF)),
            0xB => ASM::RJMP(self.as_masked(0x0FFF)),
            0xC => ASM::RND(nibbles.1, self.as_masked(0x00FF) as u8),
            0xD => ASM::DRW(nibbles.1, nibbles.2, nibbles.3),
            0xE => self.retrieve_e_opcodes(&nibbles),
            0xF => self.retrieve_f_opcodes(&nibbles),
            _ => ASM::ERR
        };

        match self.assembler {
            ASM::SHL(_) => self.opcode = self.opcode & 0xFF0F,
            ASM::SHR(_) => self.opcode = self.opcode & 0xFF0F,
            _ => {}
        }
    }

    fn retrieve_0_opcodes(&self, nibbles: &(u8, u8, u8, u8)) -> ASM {
        match nibbles.2 {
            0xE => match nibbles.3 {
                0x0 => ASM::CLS,
                0xE => ASM::RET,
                _ => ASM::SYS(self.as_masked(self.opcode))
            },
            _ => ASM::SYS(self.as_masked(self.opcode))
        }

    }

    fn retrieve_5_opcodes(&self, nibbles: &(u8, u8, u8, u8)) -> ASM {
        match nibbles.3 {
            0x0 => ASM::CPSE(nibbles.1, nibbles.2),
            _ => ASM::ERR
        }
    }

    fn retrieve_8_opcodes(&self, nibbles: &(u8, u8, u8, u8)) -> ASM {
        match nibbles.3 {
            0x0 => ASM::CP(nibbles.1, nibbles.2),
            0x1 => ASM::OR(nibbles.1, nibbles.2),
            0x2 => ASM::AND(nibbles.1, nibbles.2),
            0x3 => ASM::XOR(nibbles.1, nibbles.2),
            0x4 => ASM::ADD(nibbles.1, nibbles.2),
            0x5 => ASM::SUB(nibbles.1, nibbles.2),
            0x6 => ASM::SHR(nibbles.1),
            0x7 => ASM::SUBN(nibbles.1, nibbles.2),
            0xE => ASM::SHL(nibbles.1),
            _ => ASM::ERR
        }
    }

    fn retrieve_9_opcodes(&self, nibbles: &(u8, u8, u8, u8)) -> ASM {
        match nibbles.3 {
            0x0 => ASM::SNER(nibbles.1, nibbles.2),
            _ => ASM::ERR
        }
    }

    fn retrieve_e_opcodes(&self, nibbles: &(u8, u8, u8, u8)) -> ASM {
        match nibbles.2 {
            0x9 => match nibbles.3 {
                0xE => ASM::SKPK(nibbles.1),
                _ => ASM::ERR
            },
            0xA => match nibbles.3 {
                0x1 => ASM::SKPNK(nibbles.1),
                _ => ASM::ERR
            },
            _ => ASM::ERR
        }
    }

    fn retrieve_f_opcodes(&self, nibbles: &(u8, u8, u8, u8)) -> ASM {
        match nibbles.2 {
            0x0 => match nibbles.3 {
                0x7 => ASM::LDDT(nibbles.1),
                0xA => ASM::WLDK(nibbles.1),
                _ => ASM::ERR
            },
            0x1 => match nibbles.3 {
                0x5 => ASM::SDTR(nibbles.1),
                0x8 => ASM::SSTR(nibbles.1),
                0xE => ASM::ADDIR(nibbles.1),
                _ => ASM::ERR
            },
            0x2 => match nibbles.3 {
                0x9 => ASM::LDSPR(nibbles.1),
                _ => ASM::ERR
            },
            0x3 => match nibbles.3 {
                0x3 => ASM::BCD(nibbles.1),
                _ => ASM::ERR
            },
            0x5 => match nibbles.3 {
                0x5 => ASM::STOR(nibbles.1),
                _ => ASM::ERR
            },
            0x6 => match nibbles.3 {
                0x5 => ASM::READ(nibbles.1),
                _ => ASM::ERR
            },
            _ => ASM::ERR
        }
    }

    fn as_nibbles(&self) -> (u8, u8, u8, u8) {
        ((self.opcode >> 12) as u8 & 0xF, (self.opcode >> 8) as u8 & 0xF, (self.opcode >> 4) as u8 & 0xF, (self.opcode & 0xF) as u8)
    }

    #[allow(dead_code)]
    fn as_u16(&self) -> u16 {
        self.opcode as u16
    }

    fn as_masked(&self, bitmask: u16) -> u16 {
        self.opcode & bitmask
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let opnibbles = self.as_nibbles();
        write!(f, "0x{:X}{:X}{:X}{:X}: {:?}", opnibbles.0, opnibbles.1, opnibbles.2, opnibbles.3, self.assembler)
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;
    use super::ASM;

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
        let under_test = Opcode::new(0x00E0);
        assert!(under_test.as_asm() == ASM::CLS);
    }

    #[test]
    fn identify_ret() {
        let under_test = Opcode::new(0x00EE);
        assert!(under_test.as_asm() == ASM::RET)
    }

    #[test]
    fn identify_sys() {
        let under_test = Opcode::new(0x0123);
        assert!(under_test.as_asm() == ASM::SYS(0x123))
    }

    #[test]
    fn identify_jmp() {
        let under_test = Opcode::new(0x1123);
        assert!(under_test.as_asm() == ASM::JMP(0x123))
    }

    #[test]
    fn identify_call() {
        let under_test = Opcode::new(0x2123);
        assert!(under_test.as_asm() == ASM::CALL(0x123))
    }

    #[test]
    fn identify_se() {
        let under_test = Opcode::new(0x3123);
        assert!(under_test.as_asm() == ASM::SE(0x1, 0x23))
    }

    #[test]
    fn identify_sne() {
        let under_test = Opcode::new(0x4123);
        assert!(under_test.as_asm() == ASM::SNE(0x1, 0x23))
    }

    #[test]
    fn identify_cpse() {
        let under_test = Opcode::new(0x5120);
        assert!(under_test.as_asm() == ASM::CPSE(0x1, 0x2))
    }

    #[test]
    fn identify_ld() {
        let under_test = Opcode::new(0x6123);
        assert!(under_test.as_asm() == ASM::LD(0x1, 0x23))
    }

    #[test]
    fn identify_addi() {
        let under_test = Opcode::new(0x7123);
        assert!(under_test.as_asm() == ASM::ADDI(0x1, 0x23))
    }

    #[test]
    fn identify_cp() {
        let under_test = Opcode::new(0x8120);
        assert!(under_test.as_asm() == ASM::CP(0x1, 0x2))
    }

    #[test]
    fn identify_or() {
        let under_test = Opcode::new(0x8121);
        assert!(under_test.as_asm() == ASM::OR(0x1, 0x2))
    }

    #[test]
    fn identify_and() {
        let under_test = Opcode::new(0x8122);
        assert!(under_test.as_asm() == ASM::AND(0x1, 0x2))
    }

    #[test]
    fn identify_xor() {
        let under_test = Opcode::new(0x8123);
        assert!(under_test.as_asm() == ASM::XOR(0x1, 0x2))
    }

    #[test]
    fn identify_add() {
        let under_test = Opcode::new(0x8124);
        assert!(under_test.as_asm() == ASM::ADD(0x1, 0x2))
    }

    #[test]
    fn identify_sub() {
        let under_test = Opcode::new(0x8125);
        assert!(under_test.as_asm() == ASM::SUB(0x1, 0x2))
    }

    #[test]
    fn identify_shr() {
        let under_test = Opcode::new(0x8126);
        assert!(under_test.as_u16() == 0x8106);
        assert!(under_test.as_asm() == ASM::SHR(0x1))
    }

    #[test]
    fn identify_subn() {
        let under_test = Opcode::new(0x8127);
        assert!(under_test.as_asm() == ASM::SUBN(0x1, 0x2))
    }

    #[test]
    fn identify_shl() {
        let under_test = Opcode::new(0x812E);
        assert!(under_test.as_u16() == 0x810E);
        assert!(under_test.as_asm() == ASM::SHL(0x1))
    }

    #[test]
    fn identify_sner() {
        let under_test = Opcode::new(0x9120);
        assert!(under_test.as_asm() == ASM::SNER(0x1, 0x2))
    }

    #[test]
    fn identify_ldi() {
        let under_test = Opcode::new(0xA123);
        assert!(under_test.as_asm() == ASM::LDI(0x123))
    }

    #[test]
    fn identify_rjmp() {
        let under_test = Opcode::new(0xB123);
        assert!(under_test.as_asm() == ASM::RJMP(0x123))
    }

    #[test]
    fn identify_rnd() {
        let under_test = Opcode::new(0xC123);
        assert!(under_test.as_asm() == ASM::RND(0x1, 0x23))
    }

    #[test]
    fn identify_drw() {
        let under_test = Opcode::new(0xD123);
        assert!(under_test.as_asm() == ASM::DRW(0x1, 0x2, 0x3))
    }

    #[test]
    fn identify_skpk() {
        let under_test = Opcode::new(0xE19E);
        assert!(under_test.as_asm() == ASM::SKPK(0x1))
    }

    #[test]
    fn identify_skpnk() {
        let under_test = Opcode::new(0xE1A1);
        assert!(under_test.as_asm() == ASM::SKPNK(0x1))
    }

    #[test]
    fn identify_lddt() {
        let under_test = Opcode::new(0xF107);
        assert!(under_test.as_asm() == ASM::LDDT(0x1))
    }

    #[test]
    fn identify_wldk() {
        let under_test = Opcode::new(0xF10A);
        assert!(under_test.as_asm() == ASM::WLDK(0x1))
    }

    #[test]
    fn identify_sdtr() {
        let under_test = Opcode::new(0xF115);
        assert!(under_test.as_asm() == ASM::SDTR(0x1))
    }

    #[test]
    fn identify_sstr() {
        let under_test = Opcode::new(0xF118);
        assert!(under_test.as_asm() == ASM::SSTR(0x1))
    }

    #[test]
    fn identify_addir() {
        let under_test = Opcode::new(0xF11E);
        assert!(under_test.as_asm() == ASM::ADDIR(0x1))
    }

    #[test]
    fn identify_ldspr() {
        let under_test = Opcode::new(0xF129);
        assert!(under_test.as_asm() == ASM::LDSPR(0x1))
    }

    #[test]
    fn identify_bcd() {
        let under_test = Opcode::new(0xF133);
        assert!(under_test.as_asm() == ASM::BCD(0x1))
    }

    #[test]
    fn identify_stor() {
        let under_test = Opcode::new(0xF155);
        assert!(under_test.as_asm() == ASM::STOR(0x1))
    }

    #[test]
    fn identify_read() {
        let under_test = Opcode::new(0xF165);
        assert!(under_test.as_asm() == ASM::READ(0x1))
    }

    #[test]
    fn identify_invalid_5xxx() {
        let under_test = Opcode::new(0x5123);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_8xxx() {
        let under_test = Opcode::new(0x8128);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_9xxx() {
        let under_test = Opcode::new(0x9123);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_exxx() {
        let under_test = Opcode::new(0xE1B3);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_ex9x() {
        let under_test = Opcode::new(0xE193);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_exax() {
        let under_test = Opcode::new(0xE1A3);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_fxxx() {
        let under_test = Opcode::new(0xF177);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_fx0x() {
        let under_test = Opcode::new(0xF108);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_fx1x() {
        let under_test = Opcode::new(0xF116);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_fx2x() {
        let under_test = Opcode::new(0xF123);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_fx3x() {
        let under_test = Opcode::new(0xF134);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_fx5x() {
        let under_test = Opcode::new(0xF156);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn identify_invalid_fx6x() {
        let under_test = Opcode::new(0xF166);
        assert!(under_test.as_asm() == ASM::ERR)
    }

    #[test]
    fn from_asm_cls() {
        let under_test = Opcode::from_asm(ASM::CLS);
        assert!(under_test.as_u16() == 0x00E0);
    }

    #[test]
    fn from_asm_ret() {
        let under_test = Opcode::from_asm(ASM::RET);
        assert!(under_test.as_u16() == 0x00EE)
    }

    #[test]
    fn from_asm_sys() {
        let under_test = Opcode::from_asm(ASM::SYS(0x123));
        assert!(under_test.as_u16() == 0x0123)
    }

    #[test]
    fn from_asm_jmp() {
        let under_test = Opcode::from_asm(ASM::JMP(0x123));
        assert!(under_test.as_u16() == 0x1123)
    }

    #[test]
    fn from_asm_call() {
        let under_test = Opcode::from_asm(ASM::CALL(0x123));
        assert!(under_test.as_u16() == 0x2123)
    }

    #[test]
    fn from_asm_se() {
        let under_test = Opcode::from_asm(ASM::SE(0x1, 0x23));
        assert!(under_test.as_u16() == 0x3123)
    }

    #[test]
    fn from_asm_sne() {
        let under_test = Opcode::from_asm(ASM::SNE(0x1, 0x23));
        assert!(under_test.as_u16() == 0x4123)
    }

    #[test]
    fn from_asm_cpse() {
        let under_test = Opcode::from_asm(ASM::CPSE(0x1, 0x2));
        assert!(under_test.as_u16() == 0x5120)
    }

    #[test]
    fn from_asm_ld() {
        let under_test = Opcode::from_asm(ASM::LD(0x1, 0x23));
        assert!(under_test.as_u16() == 0x6123)
    }

    #[test]
    fn from_asmaddi() {
        let under_test = Opcode::from_asm(ASM::ADDI(0x1, 0x23));
        assert!(under_test.as_u16() == 0x7123)
    }

    #[test]
    fn from_asm_cp() {
        let under_test = Opcode::from_asm(ASM::CP(0x1, 0x2));
        assert!(under_test.as_u16() == 0x8120)
    }

    #[test]
    fn from_asm_or() {
        let under_test = Opcode::from_asm(ASM::OR(0x1, 0x2));
        assert!(under_test.as_u16() == 0x8121)
    }

    #[test]
    fn from_asm_and() {
        let under_test = Opcode::from_asm(ASM::AND(0x1, 0x2));
        assert!(under_test.as_u16() == 0x8122)
    }

    #[test]
    fn from_asm_xor() {
        let under_test = Opcode::from_asm(ASM::XOR(0x1, 0x2));
        assert!(under_test.as_u16() == 0x8123)
    }

    #[test]
    fn from_asm_add() {
        let under_test = Opcode::from_asm(ASM::ADD(0x1, 0x2));
        assert!(under_test.as_u16() == 0x8124)
    }

    #[test]
    fn from_asm_sub() {
        let under_test = Opcode::from_asm(ASM::SUB(0x1, 0x2));
        assert!(under_test.as_u16() == 0x8125)
    }

    #[test]
    fn from_asm_shr() {
        let under_test = Opcode::from_asm(ASM::SHR(0x1));

        assert!(under_test.as_u16() == 0x8106)
    }

    #[test]
    fn from_asm_subn() {
        let under_test = Opcode::from_asm(ASM::SUBN(0x1, 0x2));
        assert!(under_test.as_u16() == 0x8127)
    }

    #[test]
    fn from_asm_shl() {
        let under_test = Opcode::from_asm(ASM::SHL(0x1));
        assert!(under_test.as_u16() == 0x810E)
    }

    #[test]
    fn from_asm_sner() {
        let under_test = Opcode::from_asm(ASM::SNER(0x1, 0x2));
        assert!(under_test.as_u16() == 0x9120)
    }

    #[test]
    fn from_asm_ldi() {
        let under_test = Opcode::from_asm(ASM::LDI(0x123));
        assert!(under_test.as_u16() == 0xA123)
    }

    #[test]
    fn from_asm_rjmp() {
        let under_test = Opcode::from_asm(ASM::RJMP(0x123));
        assert!(under_test.as_u16() == 0xB123)
    }

    #[test]
    fn from_asm_rnd() {
        let under_test = Opcode::from_asm(ASM::RND(0x1, 0x23));
        assert!(under_test.as_u16() == 0xC123)
    }

    #[test]
    fn from_asm_drw() {
        let under_test = Opcode::from_asm(ASM::DRW(0x1, 0x2, 0x3));
        println!("{:X} == 0xD123", under_test.as_u16());
        assert!(under_test.as_u16() == 0xD123)
    }

    #[test]
    fn from_asm_skpk() {
        let under_test = Opcode::from_asm(ASM::SKPK(0x1));
        assert!(under_test.as_u16() == 0xE19E)
    }

    #[test]
    fn from_asm_skpnk() {
        let under_test = Opcode::from_asm(ASM::SKPNK(0x1));
        assert!(under_test.as_u16() == 0xE1A1)
    }

    #[test]
    fn from_asm_lddt() {
        let under_test = Opcode::from_asm(ASM::LDDT(0x1));
        assert!(under_test.as_u16() == 0xF107)
    }

    #[test]
    fn from_asm_wldk() {
        let under_test = Opcode::from_asm(ASM::WLDK(0x1));
        assert!(under_test.as_u16() == 0xF10A)
    }

    #[test]
    fn from_asm_sdtr() {
        let under_test = Opcode::from_asm(ASM::SDTR(0x1));
        assert!(under_test.as_u16() == 0xF115)
    }

    #[test]
    fn from_asm_sstr() {
        let under_test = Opcode::from_asm(ASM::SSTR(0x1));
        assert!(under_test.as_u16() == 0xF118)
    }

    #[test]
    fn from_asm_addir() {
        let under_test = Opcode::from_asm(ASM::ADDIR(0x1));
        assert!(under_test.as_u16() == 0xF11E)
    }

    #[test]
    fn from_asm_ldspr() {
        let under_test = Opcode::from_asm(ASM::LDSPR(0x1));
        assert!(under_test.as_u16() == 0xF129)
    }

    #[test]
    fn from_asm_bcd() {
        let under_test = Opcode::from_asm(ASM::BCD(0x1));
        assert!(under_test.as_u16() == 0xF133)
    }

    #[test]
    fn from_asm_stor() {
        let under_test = Opcode::from_asm(ASM::STOR(0x1));
        assert!(under_test.as_u16() == 0xF155)
    }

    #[test]
    fn from_asm_read() {
        let under_test = Opcode::from_asm(ASM::READ(0x1));
        assert!(under_test.as_u16() == 0xF165)
    }

    #[test]
    fn conversion_cls() {
        let under_test = Opcode::from_asm(ASM::CLS);
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_ret() {
        let under_test = Opcode::from_asm(ASM::RET);
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_sys() {
        let under_test = Opcode::from_asm(ASM::SYS(0x123));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_jmp() {
        let under_test = Opcode::from_asm(ASM::JMP(0x123));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_call() {
        let under_test = Opcode::from_asm(ASM::CALL(0x123));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_se() {
        let under_test = Opcode::from_asm(ASM::SE(0x1, 0x23));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_sne() {
        let under_test = Opcode::from_asm(ASM::SNE(0x1, 0x23));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_cpse() {
        let under_test = Opcode::from_asm(ASM::CPSE(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_ld() {
        let under_test = Opcode::from_asm(ASM::LD(0x1, 0x23));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_addi() {
        let under_test = Opcode::from_asm(ASM::ADDI(0x1, 0x23));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_cp() {
        let under_test = Opcode::from_asm(ASM::CP(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_or() {
        let under_test = Opcode::from_asm(ASM::OR(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_and() {
        let under_test = Opcode::from_asm(ASM::AND(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_xor() {
        let under_test = Opcode::from_asm(ASM::XOR(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_add() {
        let under_test = Opcode::from_asm(ASM::ADD(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_sub() {
        let under_test = Opcode::from_asm(ASM::SUB(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_shr() {
        let under_test = Opcode::from_asm(ASM::SHR(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_subn() {
        let under_test = Opcode::from_asm(ASM::SUBN(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_shl() {
        let under_test = Opcode::from_asm(ASM::SHL(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_sner() {
        let under_test = Opcode::from_asm(ASM::SNER(0x1, 0x2));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_ldi() {
        let under_test = Opcode::from_asm(ASM::LDI(0x123));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_rjmp() {
        let under_test = Opcode::from_asm(ASM::RJMP(0x123));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_rnd() {
        let under_test = Opcode::from_asm(ASM::RND(0x1, 0x23));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_drw() {
        let under_test = Opcode::from_asm(ASM::DRW(0x1, 0x2, 0x3));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_skpk() {
        let under_test = Opcode::from_asm(ASM::SKPK(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_skpnk() {
        let under_test = Opcode::from_asm(ASM::SKPNK(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_lddt() {
        let under_test = Opcode::from_asm(ASM::LDDT(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_wldk() {
        let under_test = Opcode::from_asm(ASM::WLDK(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_sdtr() {
        let under_test = Opcode::from_asm(ASM::SDTR(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_sstr() {
        let under_test = Opcode::from_asm(ASM::SSTR(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_addir() {
        let under_test = Opcode::from_asm(ASM::ADDIR(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_ldspr() {
        let under_test = Opcode::from_asm(ASM::LDSPR(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_bcd() {
        let under_test = Opcode::from_asm(ASM::BCD(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_stor() {
        let under_test = Opcode::from_asm(ASM::STOR(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }

    #[test]
    fn conversion_read() {
        let under_test = Opcode::from_asm(ASM::READ(0x1));
        assert!(under_test == Opcode::new(under_test.as_u16()))
    }
}