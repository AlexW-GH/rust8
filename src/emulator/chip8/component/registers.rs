const REGISTER_COUNT: usize = 16;

use ::emulator::chip8::component::memory;
use ::emulator::chip8::component::memory::Memory;

#[derive(Default)]
pub struct Registers {
    data_registers: [u8; REGISTER_COUNT],
    address_register: u16,
}

impl Registers {

    pub fn set_data_register_by_value(&mut self, dest: u8, value: u8) {
        assert!(self.is_register_valid(dest));
        self.data_registers[dest as usize] = value;
    }

    pub fn get_data_register_value(&self, register: u8) -> u8 {
        assert!(self.is_register_valid(register));
        self.data_registers[register as usize]
    }

    pub fn get_address_register_value(&self) -> u16 {
        self.address_register
    }

    pub fn set_address_register_value(&mut self, value: u16) {
        self.address_register = value;
    }

    pub fn set_data_register_by_register(&mut self, dest: u8, src: u8) {
        assert!(self.is_register_valid(src) && self.is_register_valid(dest));
        self.data_registers[dest as usize] = self.data_registers[src as usize];
    }

    pub fn reset_vf_to_zero(&mut self) {
        self.data_registers[0xF] = 0;
    }

    pub fn is_equal_to_value(&self, register: u8, value: u8) -> bool {
        assert!(self.is_register_valid(register));
        self.data_registers[register as usize] == value
    }

    pub fn is_equal_to_register(&self, register1: u8, register2: u8) -> bool {
        assert!(self.is_register_valid(register1) && self.is_register_valid(register2));
        self.data_registers[register1 as usize] == self.data_registers[register2 as usize]
    }

    pub fn get_data_registers(&self, start: u8, end: u8) -> &[u8] {
        self.data_registers[start as usize..(end + 1) as usize].into_iter().as_slice()
    }

    pub fn store_until_register(&mut self, register: u8, address: u16, memory: &Memory) {
        for i in 0..register + 1 {
            self.data_registers[i as usize] = memory.retrieve_value_from_address(address as u16 + i as u16);
        }
    }

    pub fn add_data_register_with_value(&mut self, register: u8, value: u8) {
        assert!(self.is_register_valid(register));
        let result = self.data_registers[register as usize].overflowing_add(value);
        self.data_registers[register as usize] = result.0;
    }

    pub fn add_data_register_with_register(&mut self, dest: u8, addend1: u8, addend2: u8) -> bool {
        assert!(self.is_register_valid(dest) && self.is_register_valid(addend1) && self.is_register_valid(addend2));
        let result = self.data_registers[addend1 as usize].overflowing_add(self.data_registers[addend2 as usize]);
        self.data_registers[dest as usize] = result.0;
        result.1
    }

    pub fn sub_data_register_with_register(&mut self, dest: u8, minuend: u8, subtrahend: u8) -> bool {
        assert!(self.is_register_valid(dest) && self.is_register_valid(minuend) && self.is_register_valid(subtrahend));
        let result = self.data_registers[minuend as usize].overflowing_sub(self.data_registers[subtrahend as usize]);
        self.data_registers[dest as usize] = result.0;
        result.1
    }

    pub fn shift_right_and_set_vf_to_lsb(&mut self, register: u8) {
        assert!(self.is_register_valid(register));
        let lsb = if (self.data_registers[register as usize] & 0b00000001) > 0 { 1 } else { 0 };
        self.data_registers[0xF] = lsb;
        self.data_registers[register as usize] >>= 1;
    }

    pub fn shift_left_and_set_vf_to_msb(&mut self, register: u8) {
        assert!(self.is_register_valid(register));
        let msb = if (self.data_registers[register as usize] & 0b10000000) > 0 { 1 } else { 0 };
        self.data_registers[0xF] = msb;
        self.data_registers[register as usize] <<= 1;
    }

    pub fn add_address_register_with_register(&mut self, register: u8) {
        assert!(self.is_register_valid(register));
        self.address_register += self.data_registers[register as usize] as u16;
    }

    pub fn set_address_register_to_sprite_from_register(&mut self, register: u8) {
        let char = self.get_data_register_value(register) as u16;
        self.address_register = memory::FONT_ADDRESS + (5 * char);
    }

    fn is_register_valid(&self, register: u8) -> bool {
        register < REGISTER_COUNT as u8
    }

}


#[cfg(test)]
mod tests {
    use super::Registers;
    use super::super::memory;
    use super::super::memory::Memory;

    #[test]
    fn set_get_value_success() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);

        let result = under_test.get_data_register_value(1);

        assert!(result == 100);
    }

    #[test]
    fn set_by_register_success() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);
        under_test.set_data_register_by_register(2, 1);

        let result = under_test.get_data_register_value(2);

        assert!(result == 100);
    }

    #[test]
    fn reset_vf_to_zero_success() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(15, 100);
        under_test.reset_vf_to_zero();

        let result = under_test.get_data_register_value(15);

        assert!(result == 0);
    }

    #[test]
    fn is_equal_to_value_success() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);

        let result = under_test.is_equal_to_value(1, 100);

        assert!(result);
    }

    #[test]
    fn is_equal_to_register_success() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);
        under_test.set_data_register_by_value(2, 100);

        let result = under_test.is_equal_to_register(1, 2);

        assert!(result);
    }

    #[test]
    fn get_data_registers_success() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);
        under_test.set_data_register_by_value(2, 150);
        under_test.set_data_register_by_value(3, 200);

        let result = under_test.get_data_registers(1, 3);

        assert!(result.len() == 3);
        assert!(result[0] == 100);
        assert!(result[1] == 150);
        assert!(result[2] == 200);
    }

    #[test]
    pub fn store_until_register_success() {
        let mut under_test: Registers = Default::default();
        let test_vec = vec![
            0x0, 0x1, 0x2, 0x3,
            0x4, 0x5, 0x6, 0x7,
            0x8, 0x9, 0xA, 0xB,
            0xC, 0xD, 0xE, 0xF,
        ];
        let test_mem = Memory::new(0x200, test_vec);

        under_test.store_until_register(4, 0x204, &test_mem);

        let result = under_test.get_data_registers(0, 3);
        assert!(result[0] == 0x4);
        assert!(result[1] == 0x5);
        assert!(result[2] == 0x6);
        assert!(result[3] == 0x7);
    }

    #[test]
    fn add_data_register_with_value_success() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);

        under_test.add_data_register_with_value(1, 50);

        let result = under_test.get_data_register_value(1);
        assert!(result == 150);
    }

    #[test]
    fn add_data_register_with_value_overflow() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 130);

        under_test.add_data_register_with_value(1, 128);

        let result = under_test.get_data_register_value(1);
        assert!(result == 2);
    }

    #[test]
    fn add_data_register_with_register_same_register() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);

        let overflow = under_test.add_data_register_with_register(1, 1, 1);

        let result = under_test.get_data_register_value(1);
        assert!(!overflow);
        assert!(result == 200);
    }

    #[test]
    fn add_data_register_with_register_different_register() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);
        under_test.set_data_register_by_value(2, 50);

        let overflow = under_test.add_data_register_with_register(1, 1, 2);

        let result = under_test.get_data_register_value(1);
        assert!(!overflow);
        assert!(result == 150);
    }

    #[test]
    fn add_data_register_with_register_overflow() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 130);
        under_test.set_data_register_by_value(2, 128);

        let overflow = under_test.add_data_register_with_register(1, 1, 2);

        let result = under_test.get_data_register_value(1);
        assert!(overflow);
        assert!(result == 2);
    }

    #[test]
    fn sub_data_register_with_register_same_register() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);

        let overflow = under_test.sub_data_register_with_register(1, 1, 1);

        let result = under_test.get_data_register_value(1);
        assert!(!overflow);
        assert!(result == 0);
    }

    #[test]
    fn sub_data_register_with_register_different_register() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);
        under_test.set_data_register_by_value(2, 50);

        let overflow = under_test.sub_data_register_with_register(1, 1, 2);

        let result = under_test.get_data_register_value(1);
        assert!(!overflow);
        assert!(result == 50);
    }

    #[test]
    fn sub_data_register_with_register_overflow() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 1);
        under_test.set_data_register_by_value(2, 2);

        let overflow = under_test.sub_data_register_with_register(1, 1, 2);

        let result = under_test.get_data_register_value(1);
        assert!(overflow);
        assert!(result == 255);
    }

    #[test]
    fn shift_right_and_set_vf_to_lsb_zero() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 0b00000010);

        under_test.shift_right_and_set_vf_to_lsb(1);

        let result = under_test.get_data_register_value(1);
        let vf = under_test.get_data_register_value(15);

        assert!(result == 1);
        assert!(vf == 0);
    }

    #[test]
    fn shift_right_and_set_vf_to_lsb_one() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 0b00000011);

        under_test.shift_right_and_set_vf_to_lsb(1);

        let result = under_test.get_data_register_value(1);
        let vf = under_test.get_data_register_value(15);

        assert!(result == 1);
        assert!(vf == 1);
    }

    #[test]
    fn shift_left_and_set_vf_to_msb_zero() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 0b01000000);

        under_test.shift_left_and_set_vf_to_msb(1);

        let result = under_test.get_data_register_value(1);
        let vf = under_test.get_data_register_value(15);
        assert!(result == 128);
        assert!(vf == 0);
    }

    #[test]
    fn shift_left_and_set_vf_to_msb_one() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 0b11000000);

        under_test.shift_left_and_set_vf_to_msb(1);

        let result = under_test.get_data_register_value(1);
        let vf = under_test.get_data_register_value(15);
        assert!(result == 128);
        assert!(vf == 1);
    }

    #[test]
    fn add_address_register_with_register_success() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 100);
        under_test.set_address_register_value(100);

        under_test.add_address_register_with_register(1);

        let result = under_test.get_address_register_value();
        assert!(result == 200)
    }

    #[test]
    fn add_address_register_with_register_result_over_1byte() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 200);
        under_test.set_address_register_value(200);

        under_test.add_address_register_with_register(1);

        let result = under_test.get_address_register_value();
        assert!(result == 400)
    }

    #[test]
    fn set_address_register_to_sprite_from_register() {
        let mut under_test: Registers = Default::default();
        under_test.set_data_register_by_value(1, 4);

        under_test.set_address_register_to_sprite_from_register(1);

        let result = under_test.get_address_register_value();
        assert!(result == (memory::FONT_ADDRESS + 20))
    }
}

