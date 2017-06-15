pub const MEM_SIZE: usize = 4096;
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
    #[allow(dead_code)]
    pub fn new(address: usize, init_vec: Vec<u8>) -> Memory {
        let mut init_memory: [u8; MEM_SIZE] = [0; MEM_SIZE];
        for (i, value) in init_vec.into_iter().enumerate() {
            init_memory[i + address] = value;
        }
        Memory { memory: init_memory }
    }

    pub fn retrieve_value_from_address(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn retrieve_range(&self, mem_start: u16, size: u8) -> &[u8] {
        if ((mem_start + size as u16) as usize) < MEM_SIZE {
            self.memory[mem_start as usize..(mem_start + (size as u16)) as usize].into_iter().as_slice()
        } else { panic!("Memory Overflow when retrieving memory range") }

    }

    pub fn store_binary_representation_of_value(&mut self, value: u8, address: u16) {
        self.memory[address as usize] = value / 100;
        self.memory[(address + 1) as usize] = (value / 10) % 10;
        self.memory[(address + 2) as usize] = value % 10;
    }

    pub fn store_from_address_on(&mut self, value: &[u8], address: u16) {
        for (index, byte) in value.into_iter().enumerate() {
            self.memory[(address as usize) + index] = *byte;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn retrieve_value_from_address_success() {
        let under_test = create_test_memory(0x200);

        let result = under_test.retrieve_value_from_address(0x201);

        assert!(result == 0x1);
    }

    #[test]
    fn retrieve_range_begin() {
        let under_test = create_test_memory(0x0);

        let result = under_test.retrieve_range(0x0, 4);

        assert!(result.len() == 4);
        assert!(result[0] == 0x0);
        assert!(result[1] == 0x1);
        assert!(result[2] == 0x2);
        assert!(result[3] == 0x3);
    }

    #[test]
    fn retrieve_range_end() {
        let under_test = create_test_memory(super::MEM_SIZE - 17);

        let result = under_test.retrieve_range((super::MEM_SIZE - 5) as u16, 4);

        println!("{}", result[0]);
        assert!(result.len() == 4);
        assert!(result[0] == 0xC);
        assert!(result[1] == 0xD);
        assert!(result[2] == 0xE);
        assert!(result[3] == 0xF);
    }

    #[test]
    #[should_panic(expected = "Memory Overflow when retrieving memory range")]
    fn retrieve_range_overflow() {
        let under_test = create_test_memory(super::MEM_SIZE - 17);

        let _ = under_test.retrieve_range((super::MEM_SIZE - 4) as u16, 4);
    }

    #[test]
    fn store_binary_representation_of_value_zero() {
        let mut under_test = create_test_memory(0x200);

        under_test.store_binary_representation_of_value(0, 0x300);

        let result1 = under_test.retrieve_value_from_address(0x302);
        let result10 = under_test.retrieve_value_from_address(0x301);
        let result100 = under_test.retrieve_value_from_address(0x300);

        assert!(result100 == 0);
        assert!(result10 == 0);
        assert!(result1 == 0);
    }

    #[test]
    fn store_binary_representation_of_value_single_digit() {
        let mut under_test = create_test_memory(0x200);

        under_test.store_binary_representation_of_value(1, 0x300);

        let result1 = under_test.retrieve_value_from_address(0x302);
        let result10 = under_test.retrieve_value_from_address(0x301);
        let result100 = under_test.retrieve_value_from_address(0x300);

        assert!(result100 == 0);
        assert!(result10 == 0);
        assert!(result1 == 1);
    }

    #[test]
    fn store_binary_representation_of_value_double_digit() {
        let mut under_test = create_test_memory(0x200);

        under_test.store_binary_representation_of_value(12, 0x300);

        let result1 = under_test.retrieve_value_from_address(0x302);
        let result10 = under_test.retrieve_value_from_address(0x301);
        let result100 = under_test.retrieve_value_from_address(0x300);

        assert!(result100 == 0);
        assert!(result10 == 1);
        assert!(result1 == 2);
    }

    #[test]
    fn store_binary_representation_of_value_triple_digit() {
        let mut under_test = create_test_memory(0x200);

        under_test.store_binary_representation_of_value(123, 0x300);

        let result1 = under_test.retrieve_value_from_address(0x302);
        let result10 = under_test.retrieve_value_from_address(0x301);
        let result100 = under_test.retrieve_value_from_address(0x300);

        assert!(result100 == 1);
        assert!(result10 == 2);
        assert!(result1 == 3);
    }

    #[test]
    fn store_from_address_on_success() {
        let mut under_test = create_test_memory(0x200);

        under_test.store_from_address_on(&vec![1, 2, 3, 4], 0x300);

        let result0 = under_test.retrieve_value_from_address(0x300);
        let result1 = under_test.retrieve_value_from_address(0x301);
        let result2 = under_test.retrieve_value_from_address(0x302);
        let result3 = under_test.retrieve_value_from_address(0x303);

        assert!(result0 == 1);
        assert!(result1 == 2);
        assert!(result2 == 3);
        assert!(result3 == 4);
    }


    fn create_test_memory(start: usize) -> Memory {
        Memory::new(start, vec![
            0x0, 0x1, 0x2, 0x3,
            0x4, 0x5, 0x6, 0x7,
            0x8, 0x9, 0xA, 0xB,
            0xC, 0xD, 0xE, 0xF,
        ])
    }
}

