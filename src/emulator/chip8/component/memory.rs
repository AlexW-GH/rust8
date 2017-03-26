const MEM_SIZE: usize = 4096;
pub const FONT_ADDRESS: u16 = 0x100;
pub const PROGRAM_ADDRESS: u16 = 0x200;

pub struct Memory {
    memory: [u8; MEM_SIZE]
}

impl Default for Memory {
    fn default() -> Memory {
        Memory { memory: [0; MEM_SIZE] }
    }
}

impl Memory {
    pub fn retrieve_value_from_address(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn retrieve_range(&self, mem_start: u16, size: u8) -> &[u8] {
        self.memory[mem_start as usize..(mem_start + (size as u16)) as usize].into_iter().as_slice()
    }

    pub fn store_binary_representation_from_register(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value / 100;
        self.memory[(address + 1) as usize] = (value / 10) % 10;
        self.memory[(address + 2) as usize] = value % 10;
    }

    pub fn store_until_register(&mut self, registers: &[u8], address: u16) {
        for (i, register) in registers.into_iter().enumerate() {
            self.memory[(address as u8 + i as u8) as usize] = *register;
        }
    }

    pub fn store_all_to_address(&mut self, data: &[u8], address: u16) {
        for (index, byte) in data.into_iter().enumerate() {
            self.memory[(address as usize) + index] = *byte;
        }
    }
}

