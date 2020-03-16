
pub const NUM_KEYS: usize = 16;

pub struct Keyboard {
    keys: [bool; NUM_KEYS]
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; NUM_KEYS],
        }
    }

    pub fn press_key(&mut self, key_index: u8) {
        self.keys[key_index as usize] = true;
    }

    pub fn release_key(&mut self, key_index: u8) {
        self.keys[key_index as usize] = false;
    }

    pub fn is_key_set(&mut self, key_index: u8) -> bool {
        self.keys[key_index as usize]
    }
}