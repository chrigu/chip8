
pub const NUM_KEYS: usize = 16;

pub struct Keyboard {
    keys: [bool; NUM_KEYS]
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; NUM_KEYS],
        }
    }

    pub fn press_key(&mut self, key_index: u8) {
        self.keys[key_index as usize] = true;
        // log!("keydown: {:x?}", self.keys[key_index as usize]);
    }

    pub fn release_key(&mut self, key_index: u8) {
        self.keys[key_index as usize] = false;
        // log!("keyup: {:x?}", self.keys[key_index as usize]);

    }

    pub fn is_key_set(&mut self, key_index: u8) -> bool {
        self.keys[key_index as usize]
    }
}