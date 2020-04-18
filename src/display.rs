const WIDTH: usize = 64;
const HEIGHT: usize = 32;
pub const NUM_PIXELS: usize = HEIGHT * WIDTH;

pub struct Display {
    memory: [bool; NUM_PIXELS],
    fontset: [u8; 80]
}

impl Display {
    pub fn new() -> Display {
        Display {
            memory: [false; NUM_PIXELS],
            fontset: [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80  // F
            ]
        }
    }

    pub fn get_fontset(&mut self) -> &[u8; 80] {
        &self.fontset
    }

    pub fn clear_screen(&mut self) {
        for x in 0..WIDTH*HEIGHT {
            self.memory[x] = false;
        }
    }

    pub fn draw_sprite_at_position(&mut self, x_left: usize, y_top: usize, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (sprite_row, byte) in sprite.iter().enumerate() {

            for bit in 0..8 {

                let x = x_left + bit;
                let y = y_top + sprite_row;

                let x_display = self.wrap_x(x);
                let y_display = self.wrap_y(y);

                let display_position = y_display * WIDTH + x_display;
                let sprite_bit = byte & (0x01 << (7 -bit));

                if self.memory[display_position] && sprite_bit > 1 {
                    collision = true;
                } 

                self.memory[display_position] = self.memory[display_position] ^ is_set(sprite_bit);
            }
        }

        collision
    }

    pub fn memory_reference(&mut self) -> *const [bool; NUM_PIXELS] {
        // todo return ref somehow
        &self.memory
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.memory[x + WIDTH * y]
    }

    fn wrap_x(&self, x: usize) -> usize {
        if x >= WIDTH {
            x - WIDTH
        } else {
            x
        }
    }

    fn wrap_y(&self, y: usize) -> usize {
        if y >= HEIGHT {
            y - HEIGHT
        } else {
            y
        }
    }
}

fn is_set(num: u8) -> bool {
    num > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_if_bit_is_set() {
        assert_eq!(true, is_set(3));
    }

    #[test]
    fn false_if_bit_is_unsset() {
        assert_eq!(false, is_set(0));
    }

    #[test]
    fn wrap_x() {
        let display = Display::new();
        let new_x = display.wrap_x(WIDTH);
        assert_eq!(0, new_x);
    }

    #[test]
    fn no_wrap_x() {
        let display = Display::new();
        let new_x = display.wrap_x(WIDTH-1);
        assert_eq!(WIDTH-1, new_x);
    }

    #[test]
    fn wrap_y() {
        let display = Display::new();
        let new_y = display.wrap_y(HEIGHT);
        assert_eq!(0, new_y);
    }

    #[test]
    fn no_wrap_y() {
        let display = Display::new();
        let new_y = display.wrap_y(HEIGHT-1);
        assert_eq!(HEIGHT-1, new_y);
    }

    #[test]
    fn draw_display() {
        let mut display = Display::new();

        let sprite = vec![0b00011000, 0b01000001];
        let collision = display.draw_sprite_at_position(0, 0, &sprite);

        assert_eq!(false, display.get_pixel(0, 0));
        assert_eq!(false, display.get_pixel(1, 0));
        assert_eq!(false, display.get_pixel(2, 0));
        assert_eq!(true, display.get_pixel(3, 0));
        assert_eq!(true, display.get_pixel(4, 0));
        assert_eq!(false, display.get_pixel(5, 0));
        assert_eq!(false, display.get_pixel(6, 0));
        assert_eq!(false, display.get_pixel(7, 0));
    
        assert_eq!(false, display.get_pixel(0, 1));
        assert_eq!(true, display.get_pixel(1, 1));
        assert_eq!(false, display.get_pixel(2, 1));
        assert_eq!(false, display.get_pixel(3, 1));
        assert_eq!(false, display.get_pixel(4, 1));
        assert_eq!(false, display.get_pixel(5, 1));
        assert_eq!(false, display.get_pixel(6, 1));
        assert_eq!(true, display.get_pixel(7, 1));
    }

    #[test]
    fn detect_collision() {
        let mut display = Display::new();

        let sprite1 = vec![0b00011000];
        display.draw_sprite_at_position(0, 0, &sprite1);

        let sprite2 = vec![0b00010000];
        let collision = display.draw_sprite_at_position(0, 0, &sprite2);

        assert_eq!(true, collision);
    }

    #[test]
    fn detect_no_collision() {
        let mut display = Display::new();

        let sprite1 = vec![0b00001000];
        display.draw_sprite_at_position(0, 0, &sprite1);

        let sprite2 = vec![0b00010000];
        let collision = display.draw_sprite_at_position(0, 0, &sprite2);

        assert_eq!(false, collision);
    }

}


/*
For each line in the sprite

    Starting with MSB, For each bit in the sprite (that is still on the screen)

        If the bit is set

            Determine the address of the effected byte on the screen

            Determine the effected bit in the byte

            Check to see if the screen's bit is set and set VF appropriately

            XOR the source bit and screen bit

            Write the effected bit to the screen
*/