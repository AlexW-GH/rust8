const REGISTER_COUNT: usize = 16;

use ::emulator::chip8::component::memory;
use ::emulator::chip8::component::memory::Memory;

#[derive(Default)]
pub struct Registers {
    data_registers: [u8; REGISTER_COUNT],
    address_register: u16,
}

impl Registers {
    pub fn set_data_register_by_register(&mut self, dest: u8, src: u8) {
        assert!(self.is_register_valid(src) && self.is_register_valid(dest));
        self.data_registers[dest as usize] = self.data_registers[src as usize];
    }

    pub fn set_data_register_by_value(&mut self, dest: u8, value: u8) {
        assert!(self.is_register_valid(dest));
        self.data_registers[dest as usize] = value;
    }

    pub fn get_data_register_value(&self, register: u8) -> u8 {
        assert!(self.is_register_valid(register));
        self.data_registers[register as usize]
    }

    pub fn add_data_register_with_register(&mut self, dest: u8, addend1: u8, addend2: u8) -> bool {
        assert!(self.is_register_valid(dest) && self.is_register_valid(addend1) && self.is_register_valid(addend2));
        let result = self.data_registers[addend1 as usize].overflowing_add(self.data_registers[addend2 as usize]);
        self.data_registers[dest as usize] = result.0;
        result.1
    }

    pub fn add_data_register_with_value(&mut self, register: u8, value: u8) {
        assert!(self.is_register_valid(register));
        let result = self.data_registers[register as usize].overflowing_add(value);
        self.data_registers[register as usize] = result.0;
    }

    pub fn sub_data_register_with_register(&mut self, dest: u8, minuend: u8, subtrahend: u8) -> bool {
        assert!(self.is_register_valid(dest) && self.is_register_valid(minuend) && self.is_register_valid(subtrahend));
        let result = self.data_registers[minuend as usize].overflowing_sub(self.data_registers[subtrahend as usize]);
        self.data_registers[dest as usize] = result.0;
        result.1
    }

    pub fn shift_right_and_set_vf_to_lsb(&mut self, register: u8) {
        assert!(self.is_register_valid(register));
        let lsb = self.data_registers[register as usize] & 0b00000001;
        self.data_registers[0xF] = lsb;
        self.data_registers[register as usize] >>= 1;
    }

    pub fn shift_left_and_set_vf_to_msb(&mut self, register: u8) {
        assert!(self.is_register_valid(register));
        let msb = self.data_registers[register as usize] & 0b10000000;
        self.data_registers[0xF] = msb;
        self.data_registers[register as usize] <<= 1;
    }

    pub fn reset_vf_to_zero(&mut self) {
        self.data_registers[0xF] = 0;
    }

    pub fn get_address_register_value(&self) -> u16 {
        self.address_register
    }

    pub fn set_address_register_value(&mut self, value: u16) {
        self.address_register = value;
    }

    pub fn add_address_register_with_register(&mut self, register: u8) {
        assert!(self.is_register_valid(register));
        self.address_register += self.data_registers[register as usize] as u16;
    }

    pub fn set_address_register_to_sprite_from_register(&mut self, register: u8) {
        let char = self.get_data_register_value(register) as u16;
        self.address_register = memory::FONT_ADDRESS + (5 * char);
    }

    pub fn get_data_registers(&self, start: u8, end: u8) -> &[u8] {
        self.data_registers[start as usize..(end + 1) as usize].into_iter().as_slice()
    }

    pub fn store_until_register(&mut self, register: u8, address: u16, memory: &Memory) {
        for i in 0..register + 1 {
            self.data_registers[i as usize] = memory.retrieve_value_from_address(address as u16 + i as u16);
        }
    }

    pub fn is_equal_to_value(&self, register: u8, value: u8) -> bool {
        assert!(self.is_register_valid(register));
        self.data_registers[register as usize] == value
    }

    pub fn is_equal_to_register(&self, register1: u8, register2: u8) -> bool {
        assert!(self.is_register_valid(register1) && self.is_register_valid(register2));
        self.data_registers[register1 as usize] == self.data_registers[register2 as usize]
    }

    fn is_register_valid(&self, register: u8) -> bool {
        register < REGISTER_COUNT as u8
    }

}

/*
0x6E05 VE = 05
0x6500 V5 = 00
0x6B06 Sprite Y Pos @ 06
0x6A00 Sprite X Pos @ 00
0xA30C Sprite @ 30C
0xDAB1 draw
*/
