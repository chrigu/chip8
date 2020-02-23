const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const NUM_PIXELS: usize = HEIGHT * WIDTH;

pub struct Display {
    memory: [bool; NUM_PIXELS]
}

impl Display {
    pub fn new() -> Display {
        Display {
            memory: [false; NUM_PIXELS],
        }
    }

    pub fn clear_screen(&mut self) {
        for x in 0..WIDTH*HEIGHT {
            self.memory[x] = false;
        }
    }

    pub fn draw_sprite_at_position(&mut self, x_left: usize, y_top: usize, sprite: &[u8]) -> bool {
        // let position = y * WIDTH + x;
        // self.memory[position] = bool;
        let mut collision = false;

        for (sprite_row, byte) in sprite.iter().enumerate() {
            for bit in 0..8 {

                let x = x_left + bit;
                let y = y_top + sprite_row;

                let x_display = self.wrap_x(x);

                let y_display = self.wrap_y(y);

                let display_position = y_display * WIDTH + x_display + bit;
                let sprite_bit = byte & (1 << bit);

                if self.memory[display_position] && sprite_bit > 1 {
                    collision = true;
                } 

                self.memory[display_position] = self.memory[display_position] ^ is_set(sprite_bit);
            }
        }

        collision
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
}

// (0..=100)
// .flat_map(|noun| (0..=100).map(move |verb| (noun, verb)))
// .filter_map(|(noun, verb)| {
//     let mut v = code.clone();
//     v[1] = noun;
//     v[2] = verb;
//     run(v).map(|r| (noun, verb, r))
// })
// .find(|(_, _, r)| *r == target)
// .map(|(noun, verb, _)| 100 * noun + verb)
// .ok_or(0)

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