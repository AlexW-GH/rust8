const MEM_SIZE: usize = 4096;

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

    pub fn set_value_to_address(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value;
    }
}

