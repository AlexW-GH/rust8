mod component;

use std::string::String;
use std::vec::Vec;
use rand::Rng;
use rand::StdRng;

use self::component::timer::Timer;
use self::component::input::Input;
use self::component::screen::Screen;
use self::component::registers::Registers;
use self::component::memory;
use self::component::memory::Memory;
use self::component::opcode::Opcode;
use self::component::opcode::Command::*;
use emulator::Emulator;

const FONTSET: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0xf0, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];

pub struct Chip8 {
    memory: Memory,
    stack: Vec<u16>,
    registers: Registers,
    delay_timer: Timer,
    sound_timer: Timer,
    input: Input,
    screen: Screen,
    pc: u16,
    title: String,
    rng: StdRng,
    need_redraw: bool,
}

fn retrieve_op(memory: &Memory, address: u16) -> Opcode {
    Opcode::new(((memory.retrieve_value_from_address(address) as u16) << 8) + memory.retrieve_value_from_address(address + 1) as u16)

}

impl Default for Chip8 {
    fn default() -> Chip8 {
        log_logo();
        Chip8 {
            pc: memory::PROGRAM_ADDRESS,
            screen: Default::default(),
            input: Default::default(),
            sound_timer: Default::default(),
            delay_timer: Default::default(),
            registers: Default::default(),
            memory: Default::default(),
            stack: Vec::new(),
            title: String::from("Chip 8"),
            rng: StdRng::new().unwrap(),
            need_redraw: false,
        }
    }
}

impl Emulator for Chip8 {
    fn retrieve_screen_pixels(&self) -> &[bool] {
        self.screen.retrieve_state()
    }

    fn retrieve_screen_size(&self) -> (usize, usize) {
        self.screen.get_dimensions()
    }

    fn update(&mut self) {
        self.delay_timer.tick_down();
        self.sound_timer.tick_down();
        let mut opcode = retrieve_op(&self.memory, self.pc);
        self.pc += 2;
        self.execute_op(&mut opcode);
    }

    fn get_name(&self) -> &str {
        self.title.as_str()
    }

    fn load(&mut self, game_data: Vec<u8>) {
        self.memory.store_from_address_on(game_data.as_slice(), memory::PROGRAM_ADDRESS);
        self.memory.store_from_address_on(&FONTSET, memory::FONT_ADDRESS);
    }

    fn press_key(&mut self, key: u16) {
        self.input.press_key(key)
    }

    fn release_key(&mut self, key: u16) {
        self.input.release_key(key)
    }
    fn needs_redraw(&mut self) -> bool {
        let redraw = self.need_redraw;
        self.need_redraw = false;
        redraw
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { ..Default::default() }
    }

    fn execute_op(&mut self, opcode: &mut Opcode) {
        let opnibbles = opcode.as_nibbles();
        match opcode.as_asm() {
            CLS => {
                self.screen.clear();
                self.need_redraw = true;
            },
            RET => self.return_from_subroutine(),
            SYS => error!("RCA 1802 subroutine calls are not implemented - opcode {}", opcode),
            JMP => self.jump_to_address(opcode.as_masked(0x0FFF)),
            CALL => self.call_subroutine(opcode.as_masked(0x0FFF)),
            SE => self.skip_if_register_equals_value(opnibbles.1, opcode.as_masked(0x00FF) as u8),
            SNE => self.skip_if_register_not_equals_value(opnibbles.1, opcode.as_masked(0x00FF) as u8),
            CPSE => {
                let is_equal = self.registers.is_equal_to_register(opnibbles.1, opnibbles.2);
                self.skip_next_op_if(is_equal);
            },
            LD => self.registers.set_data_register_by_value(opnibbles.1, opcode.as_masked(0x00FF) as u8),
            ADDI => self.registers.add_data_register_with_value(opnibbles.1, opcode.as_masked(0x00FF) as u8),
            CP => self.registers.set_data_register_by_register(opnibbles.1, opnibbles.2),
            OR => {
                self.registers.set_data_register_by_register(opnibbles.1, opnibbles.1 | opnibbles.2);
                self.registers.reset_vf_to_zero();
            },
            AND => {
                self.registers.set_data_register_by_register(opnibbles.1, opnibbles.1 & opnibbles.2);
                self.registers.reset_vf_to_zero();
            },
            XOR => {
                self.registers.set_data_register_by_register(opnibbles.1, opnibbles.1 ^ opnibbles.2);
                self.registers.reset_vf_to_zero();
            },
            ADD => {
                let overflow = self.registers.add_data_register_with_register(opnibbles.1, opnibbles.1, opnibbles.2);
                self.registers.set_data_register_by_value(0xF, if overflow { 1 } else { 0 });
            },
            SUB => {
                let overflow = self.registers.sub_data_register_with_register(opnibbles.1, opnibbles.1, opnibbles.2);
                self.registers.set_data_register_by_value(0xF, if overflow { 0 } else { 1 });
            },
            SHR => self.registers.shift_right_and_set_vf_to_lsb(opnibbles.1),
            SUBN => {
                let overflow = self.registers.sub_data_register_with_register(opnibbles.1, opnibbles.2, opnibbles.1);
                self.registers.set_data_register_by_value(0xF, if overflow { 0 } else { 1 });
            },
            SHL => self.registers.shift_left_and_set_vf_to_msb(opnibbles.1),
            SNER => {
                let is_equal = self.registers.is_equal_to_register(opnibbles.1, opnibbles.2);
                self.skip_next_op_if(!is_equal);
            },
            LDI => self.registers.set_address_register_value(opcode.as_masked(0x0FFF)),
            JMPI => self.jump_to_v0_plus_value(opcode.as_masked(0x0FFF)),
            RND => self.set_data_register_to_random(opnibbles.1, opcode.as_masked(0x00FF) as u8),
            DRW => self.draw_sprite_and_set_vf_if_pixel_flipped_to_zero(opnibbles.1, opnibbles.2, opnibbles.3),
            SKPK => {
                let button_pressed = self.input.is_pressed(self.registers.get_data_register_value(opnibbles.1));
                self.skip_next_op_if(button_pressed)
            },
            SKPNK => {
                let button_pressed = self.input.is_pressed(self.registers.get_data_register_value(opnibbles.1));
                self.skip_next_op_if(!button_pressed)
            },
            LDDT => self.registers.set_data_register_by_value(opnibbles.1, self.delay_timer.get_value()),
            WLDK => self.wait_for_key_and_set_register_to_key_value(opnibbles.1),
            SDTR => self.delay_timer.set_value(self.registers.get_data_register_value(opnibbles.1)),
            SSTR => self.sound_timer.set_value(self.registers.get_data_register_value(opnibbles.1)),
            ADDIR => self.registers.add_address_register_with_register(opnibbles.1),
            LDSPR => self.registers.set_address_register_to_sprite_from_register(opnibbles.1),
            BCD => self.memory.store_binary_representation_of_value(self.registers.get_data_register_value(opnibbles.1), self.registers.get_address_register_value()),
            STOR => self.memory.store_from_address_on(self.registers.get_data_registers(0x0, opnibbles.1), self.registers.get_address_register_value()),
            READ => {
                let address_value = self.registers.get_address_register_value();
                self.registers.store_until_register(opnibbles.1, address_value, &self.memory)
            }
            ERR => error!("Unknown opcode: {}", opcode)
        }
    }

    fn jump_to_v0_plus_value(&mut self, value: u16) {
        let address = self.registers.get_data_register_value(0x0) as u16 + value;
        self.jump_to_address(address);
    }

    fn skip_if_register_equals_value(&mut self, register: u8, value: u8) {
        let is_equal = self.registers.is_equal_to_value(register, value);
        self.skip_next_op_if(is_equal);
    }

    fn skip_if_register_not_equals_value(&mut self, register: u8, value: u8) {
        let is_equal = self.registers.is_equal_to_value(register, value);
        self.skip_next_op_if(!is_equal);
    }

    fn return_from_subroutine(&mut self) {
        self.pc = self.stack.pop().unwrap();
        debug!("Returning to {:X} from Subroutine", self.pc);
    }

    fn jump_to_address(&mut self, to_address: u16) {
        self.pc = to_address;
    }

    fn call_subroutine(&mut self, to_address: u16) {
        debug!("Initiate subroutine at {:X}, jumping from {:X}", to_address, self.pc);
        self.stack.push(self.pc);
        self.pc = to_address;
    }

    fn skip_next_op_if(&mut self, condition: bool) {
        if condition { self.pc += 2 }
    }

    fn set_data_register_to_random(&mut self, register: u8, value: u8) {
        let random = (self.rng.next_u32() as u8) & value;
        self.registers.set_data_register_by_value(register, random)
    }

    fn draw_sprite_and_set_vf_if_pixel_flipped_to_zero(&mut self, pos_x: u8, pos_y: u8, height: u8) {
        let sprite = self.memory.retrieve_range(self.registers.get_address_register_value(), height);
        let pixel_flipped = self.screen.draw(self.registers.get_data_register_value(pos_x), self.registers.get_data_register_value(pos_y), sprite);
        self.registers.set_data_register_by_value(0xF, if pixel_flipped { 1 } else { 0 });
        self.need_redraw = true;
    }

    fn wait_for_key_and_set_register_to_key_value(&mut self, register: u8) {
        {
            if let Some(key) = self.input.get_any_pressed_key() {
                self.registers.set_data_register_by_value(register, key)
            } else {
                self.pc -= 2
            }
        }
    }
}

fn log_logo() {
    info!(" _____ ___ _____ ________ __________ _____ ______ ______ ______ ______");
    info!("|_____|___|_____|________|__________|_____|______|______|______|______|");
    info!("___  ___  ___  .______       __    __       _______.___________.  ___   ___  ___  ___  ___");
    info!("\\  \\ \\  \\ \\  \\ |   _  \\     |  |  |  |     /       |           | / _ \\  \\  \\ \\  \\ \\  \\ \\  \\");
    info!(" \\  \\ \\  \\ \\  \\|  |_)  |    |  |  |  |    |   (----`---|  |----`| (_) |  \\  \\ \\  \\ \\  \\ \\  \\");
    info!("  >  > >  > >  >      /     |  |  |  |     \\   \\       |  |      > _ <    >  > >  > >  > >  >");
    info!(" /  / /  / /  /|  |\\  \\----.|  `--'  | .----)   |      |  |     | (_) |  /  / /  / /  / /  /");
    info!("/__/ /__/ /__/ | _| `._____| \\______/  |_______/       |__|      \\___/  /__/ /__/ /__/ /__/");
    info!(" _____ ___ ______ _______ __________ _____ ______ ______ ______ ______ ______ ______ ______");
    info!("|_____|_A_|_Rust_|_CHIP8_|_Emulator_|_____|______|______|______|______|______|______|______|");
    info!("|_____|___|______|_______|__________|_____|______|______|______|______|______|______|______|");
}

