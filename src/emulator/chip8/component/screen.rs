const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;
const SCREEN_PIXELS: usize = SCREEN_HEIGHT * SCREEN_WIDTH;

const SPRITE_WIDTH: usize = 8;

pub struct Screen {
    screen: Vec<bool>,
    screen_height: usize,
    screen_width: usize,
}

impl Default for Screen {
    fn default() -> Screen {
        Screen { screen: vec![false; SCREEN_PIXELS], screen_height: SCREEN_HEIGHT, screen_width: SCREEN_WIDTH }
    }
}

impl Screen {
    pub fn retrieve_state(&self) -> &[bool] {
        self.screen.as_slice()
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.screen_width, self.screen_height)
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.screen {
            *pixel = false;
        }
    }

    pub fn draw(&mut self, pos_x: u8, pos_y: u8, sprite: &[u8]) {
        for (row_index, sprite_row) in sprite.into_iter().enumerate() {
            let mask: u8 = 0b00000001;
            let pos = self.translate_coordinate(pos_x, pos_y + (row_index as u8));
            for i in 0..SPRITE_WIDTH {
                self.screen[pos + i] = (sprite_row & mask << i) > 0;
            }
        }
    }

    fn translate_coordinate(&mut self, pos_x: u8, pos_y: u8) -> usize {
        (pos_x as usize) + ((pos_y as usize) * self.screen_width)
    }
}

