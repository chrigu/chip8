use console_error_panic_hook;
extern crate web_sys;

use crate::display::{Display, NUM_PIXELS};

use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// todo find better solution, don't repeat display...
// #[wasm_bindgen]
// #[derive(Clone, Copy)]
// pub struct DisplayMemory {
//     memory: Vec<bool>
// }

#[wasm_bindgen]
pub struct Cpu {
    memory: [u8; 4096],
    v: [u8; 16],
    pc: u16,
    sp: u8,
    i: u16,
    display: Display,
    stack: [u16; 16]
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            display: Display::new()
        }
    }

    pub fn init(&mut self, rom: &[u8]) {
        self.pc = 0x200;
        self.sp = 0;
        self.i = 0;
        self.display.clear_screen();
        self.memory[0x200..].copy_from_slice(rom);
    }

    pub fn emulate_cycle(&mut self) {

        // fetch opcode
        let opcode = read_word(self.memory, self.pc);

        self.pc += 2;

        match opcode {
            0x00E0 => { // Clear the screen.
                self.display.clear_screen();
            }
            0x00EE => { // Return from subroutine
                self.sp = self.sp -1;
                self.pc = self.stack[self.sp as usize];
            }
            0x1000..=0x1FFF => { // Jump to  
                self.pc = opcode & 0x0FFF;
            }
            0x3000..=0x3FFF => { // Skip  
                let value = opcode & 0x0FFF;
                let register = opcode & 0x0F00;

                if self.v[register as usize] as usize == value as usize {
                    self.pc += 2;
                }
            }
            0x6000..=0x6FFF => { // Set vX 
                let register = ((opcode & 0x0F00) >> 12);
                self.v[register as usize] = opcode_register_value(opcode);
            }
            0x7000..=0x7FFF => { // Add vX 
                let register = opcode & 0x0F00;
                self.v[register as usize] = self.v[register as usize]  + opcode_register_value(opcode);
            }
            0xA000..=0xAFFF => { // Set I
                self.i = opcode & 0x0FFF;
            }
            0xD000..=0xDFFF => { // Draw sprite
                let vx = opcode_3rd_octet(opcode) as usize;
                let vy = opcode_2nd_octet(opcode) as usize;
                let sprite_size = opcode_1st_octet(opcode) as usize;
                let sprite_address = self.i as usize;
                
                let sprite_address_end = (sprite_address + sprite_size) as usize;
                let sprite = &self.memory[sprite_address..sprite_address_end];
                log!("x: {}, y: {}, size: {}, address: {}, opcode: {}", self.v[vx], self.v[vy], sprite_size, sprite_address, opcode);

                for (i, c) in sprite.iter().enumerate() {
                    log!("bit {}", c);
                }

                self.display.draw_sprite_at_position(self.v[vx] as usize, self.v[vy] as usize, &sprite);

                // get bytes
                // draw to display
                // detect collision
  
            }
            _ => {
                println!("unkown opcode {:?}", opcode);
            }
        }
    }

    pub fn read_display(&mut self) -> *const [bool; NUM_PIXELS] {
        self.display.read_display()
    }

    pub fn read_pc(&mut self) -> u16 {
        self.pc
    }

}

fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8 | memory[index as usize + 1] as u16
}

fn opcode_register_value(opcode: u16) -> u8 {
    return (opcode & 0x00FF) as u8
}

// fn opcode_4th_octet(opcode: u16) -> usize {

// }

fn opcode_3rd_octet(opcode: u16) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}

fn opcode_2nd_octet(opcode: u16) -> u8 {
    ((opcode & 0x00F0) >> 4) as u8
}

fn opcode_1st_octet(opcode: u16) -> u8 {
    (opcode & 0x000F) as u8
}


/*
00e0 a22a 600c 6108 d01f 7009 a239 d01f
a248 7008 d01f 7004 a257 d01f 7008 a266
d01f 7008 a275 d01f 1228 ff00 ff00 3c00
3c00 3c00 3c00 ff00 ffff 00ff 0038 003f
003f 0038 00ff 00ff 8000 e000 e000 8000
8000 e000 e000 80f8 00fc 003e 003f 003b
0039 00f8 00f8 0300 0700 0f00 bf00 fb00
f300 e300 43e0 00e0 0080 0080 0080 0080
00e0 00e0 
*/