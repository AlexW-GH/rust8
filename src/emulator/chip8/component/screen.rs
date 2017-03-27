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

    pub fn draw(&mut self, pos_x: u8, pos_y: u8, sprite: &[u8]) -> bool {
        let mut pixel_flipped: bool = false;
        for (row_index, sprite_row) in sprite.into_iter().enumerate() {
            if (row_index + pos_y as usize) >= self.screen_height {
                break;
            }
            let mask: u8 = 0b10000000;
            let pos = self.translate_coordinate(pos_x, pos_y + (row_index as u8));
            for col_index in 0..SPRITE_WIDTH {
                if (col_index + pos_x as usize) >= self.screen_width {
                    break;
                }
                if (sprite_row & mask >> col_index) > 0 {
                    let cur_pos = pos + col_index;
                    if self.screen[cur_pos] { pixel_flipped = true }
                    self.screen[cur_pos] = !self.screen[cur_pos]
                }
            }
        }
        pixel_flipped
    }

    fn translate_coordinate(&mut self, pos_x: u8, pos_y: u8) -> usize {
        (pos_x as usize) + ((pos_y as usize) * self.screen_width)
    }
}

#[cfg(test)]
mod tests {
    use super::Screen;

    const TEST_HEIGHT: usize = 5;
    const TEST_WIDTH: usize = 10;
    const TEST_DATA: [u8; 4] = [
        0b01111110,
        0b00011000,
        0b00011000,
        0b00011000,
    ];

    fn create_test_screen() -> Vec<bool> {
        vec![false; TEST_WIDTH * TEST_HEIGHT]
    }

    #[test]
    fn translate_coordinate_upperleft() {
        let mut under_test: Screen = Screen { screen: create_test_screen(), screen_height: TEST_HEIGHT, screen_width: TEST_WIDTH };
        let result = under_test.translate_coordinate(0, 0);
        assert!(result == 0)
    }

    #[test]
    fn translate_coordinate_upperright() {
        let mut under_test: Screen = Screen { screen: create_test_screen(), screen_height: TEST_HEIGHT, screen_width: TEST_WIDTH };
        let result = under_test.translate_coordinate(TEST_WIDTH as u8 - 1, 0);
        assert!(result == 9)
    }

    #[test]
    fn translate_coordinate_lowerleft() {
        let mut under_test: Screen = Screen { screen: create_test_screen(), screen_height: TEST_HEIGHT, screen_width: TEST_WIDTH };
        let result = under_test.translate_coordinate(0, TEST_HEIGHT as u8 - 1);
        assert!(result == 40)
    }

    #[test]
    fn translate_coordinate_lowerright() {
        let mut under_test: Screen = Screen { screen: create_test_screen(), screen_height: TEST_HEIGHT, screen_width: TEST_WIDTH };
        let result = under_test.translate_coordinate(TEST_WIDTH as u8 - 1, TEST_HEIGHT as u8 - 1);
        assert!(result == 49)
    }

    #[test]
    fn draw_success() {
        let mut under_test: Screen = Screen { screen: create_test_screen(), screen_height: TEST_HEIGHT, screen_width: TEST_WIDTH };
        let switch = under_test.draw(0, 0, &TEST_DATA);

        let result = under_test.retrieve_state();

        let expected = [
            false, true, true, true, true, true, true, false, false, false,
            false, false, false, true, true, false, false, false, false, false,
            false, false, false, true, true, false, false, false, false, false,
            false, false, false, true, true, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false
        ];

        for (i, res) in result.iter().enumerate() {
            assert!(*res == expected[i]);
        }
        assert!(switch == false);
    }

    #[test]
    fn draw_and_remove() {
        let mut under_test: Screen = Screen { screen: create_test_screen(), screen_height: TEST_HEIGHT, screen_width: TEST_WIDTH };
        let _ = under_test.draw(0, 0, &TEST_DATA);
        let switch = under_test.draw(0, 0, &TEST_DATA);

        let result = under_test.retrieve_state();

        let expected = [false; TEST_WIDTH * TEST_HEIGHT];

        for (i, res) in result.iter().enumerate() {
            assert!(*res == expected[i]);
        }
        assert!(switch == true);
    }

    #[test]
    fn draw_x_cutoff() {
        let mut under_test: Screen = Screen { screen: create_test_screen(), screen_height: TEST_HEIGHT, screen_width: TEST_WIDTH };
        let switch = under_test.draw(5, 0, &TEST_DATA);

        let result = under_test.retrieve_state();

        let expected = [
            false, false, false, false, false, false, true, true, true, true,
            false, false, false, false, false, false, false, false, true, true,
            false, false, false, false, false, false, false, false, true, true,
            false, false, false, false, false, false, false, false, true, true,
            false, false, false, false, false, false, false, false, false, false
        ];

        for (i, res) in result.iter().enumerate() {
            assert!(*res == expected[i]);
        }
        assert!(switch == false);
    }

    #[test]
    fn draw_y_cutoff() {
        let mut under_test: Screen = Screen { screen: create_test_screen(), screen_height: TEST_HEIGHT, screen_width: TEST_WIDTH };
        let switch = under_test.draw(0, 2, &TEST_DATA);

        let result = under_test.retrieve_state();

        let expected = [
            false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
            false, true, true, true, true, true, true, false, false, false,
            false, false, false, true, true, false, false, false, false, false,
            false, false, false, true, true, false, false, false, false, false,
        ];


        for (i, res) in result.iter().enumerate() {
            assert!(*res == expected[i]);
        }
        assert!(switch == false);
    }
}
