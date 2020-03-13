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
        let opcode = read_word(self.memory, self.pc);
        self.pc += 2;

        log!("opcode: {:x?}", opcode);
        self.handle_opcode(opcode);
    }

    pub fn handle_opcode(&mut self, opcode:u16) {
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
            0x2000..=0x2FFF => { // Call subroutine 
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp +1;
                self.pc = opcode & 0x0FFF;
            }
            0x3000..=0x3FFF => { // Skip if equal
                let value = opcode & 0x00FF;
                let register = opcode_3rd_octet(opcode) as usize;

                if self.v[register as usize] as usize == value as usize {
                    self.pc += 2;
                }
            }
            0x4000..=0x4FFF => { // Skip if not equal
                let value = opcode & 0x00FF;
                let register = opcode_3rd_octet(opcode) as usize;

                if self.v[register as usize] as usize != value as usize {
                    self.pc += 2;
                }
            }
            0x5000..=0x5FFF => { // Skip if register values equal 
                let vx = opcode_3rd_octet(opcode) as usize;
                let vy = opcode_2nd_octet(opcode) as usize;

                if self.v[vx] as usize == self.v[vy] as usize {
                    self.pc += 2;
                }
            }
            0x6000..=0x6FFF => { // Set vX 
                let register = ((opcode & 0x0F00) >> 8);
                self.v[register as usize] = opcode_register_value(opcode);
            }
            0x7000..=0x7FFF => { // Add vX 
                let register = opcode_3rd_octet(opcode) as usize;
                self.v[register as usize] = self.v[register as usize]  + opcode_register_value(opcode);
            }
            0x8000..=0x8FFF => { // Assign value from to register

                let first_octet = opcode_1st_octet(opcode);
                let vx = opcode_3rd_octet(opcode) as usize;
                let vy = opcode_2nd_octet(opcode) as usize;

                match first_octet {
                    0 => {
                        self.v[vx] = self.v[vy] as u8;
                    }
                    1 => {
                        self.v[vx] = (self.v[vx] | self.v[vy]) as u8;
                    }
                    2 => {
                        self.v[vx] = (self.v[vx] & self.v[vy]) as u8;
                    }
                    3 => {
                        self.v[vx] = (self.v[vx] ^ self.v[vy]) as u8;
                    }
                    4 => {
                        let sum:usize = self.v[vx] as usize + self.v[vy] as usize;
                        
                        self.v[0xF] = if sum > 255 {
                            1
                        } else {
                            0
                        };
                        self.v[vx] = sum as u8;
                    }
                    5 => {
                        self.v[0xF] = if self.v[vx] > self.v[vy] {
                            1
                        } else {
                            0
                        };

                        self.v[vx] = self.v[vx].wrapping_sub(self.v[vy]);
                    }
                    6 => {
                        self.v[0xF] = 0x1 & self.v[vx];
                        self.v[vx] = self.v[vx] >> 1;
                    }
                    7 => {
                        self.v[0xF] = if self.v[vy] > self.v[vx] {
                            1
                        } else {
                            0
                        };

                        self.v[vx] = self.v[vy].wrapping_sub(self.v[vx]);
                    }
                    0xE => {
                        self.v[0xF] = if (0x80 & self.v[vx]) > 0 {
                            1
                        } else {
                            0
                        };
                        self.v[vx] = (self.v[vx] & 0x0F) << 1
                    }
                    _ => {
                        println!("unkown opcode {:?}", opcode);
                    }
                }

            }
            0x9000..=0x9FFF => { // Skip if not equal
                let vx = opcode_3rd_octet(opcode) as usize;
                let vy = opcode_2nd_octet(opcode) as usize;

                if self.v[vx] as usize != self.v[vy] as usize {
                    self.pc += 2;
                }
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
                // log!("x: {}, y: {}, size: {}, address: {}, opcode: {}", self.v[vx], self.v[vy], sprite_size, sprite_address, opcode);
                log!("vx: {}, vy: {}", vx, vy);

                // for (i, c) in sprite.iter().enumerate() {
                //     log!("bit {}", c);
                // }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_3_octet() {
        assert_eq!(0xF, opcode_3rd_octet(0xF0A));
    }

    #[test]
    fn get_2_octet() {
        assert_eq!(0x1, opcode_2nd_octet(0xB1A));
    }

    #[test]
    fn get_1_octet() {
        assert_eq!(0x1, opcode_1st_octet(0x1));
    }

    #[test]
    fn jump_to_address_0x1xxx() {
        let mut cpu = Cpu::new();
        let address = 0xA1A;
        let opcode = 0x1000 | address;
        cpu.handle_opcode(opcode);
        assert_eq!(address, cpu.pc);
    }

    #[test]
    fn call_subroutine_0x2xxx() {
        let mut cpu = Cpu::new();

        let inital_pc = cpu.pc;
        let address = 0xA1A;
        let opcode = 0x2000 | address;

        cpu.handle_opcode(opcode);
        assert_eq!(address, cpu.pc);
        assert_eq!(1, cpu.sp);
        assert_eq!(cpu.stack[0], inital_pc);
    }

    #[test]
    fn skip_if_equal_0x3xxx() {
        let mut cpu = Cpu::new();

        let inital_pc = cpu.pc;
        cpu.v[8] = 2;

        let opcode = 0x3802;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.pc, inital_pc + 2);
    }

    #[test]
    fn no_skip_if_not_equal_0x3xxx() {
        let mut cpu = Cpu::new();

        let inital_pc = cpu.pc;
        cpu.v[8] = 5;

        let opcode = 0x3802;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.pc, inital_pc);
    }

    #[test]
    fn skip_if_not_equal_0x4xxx() {
        let mut cpu = Cpu::new();

        let inital_pc = cpu.pc;
        cpu.v[8] = 2;

        let opcode = 0x480A;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.pc, inital_pc + 2);
    }

    #[test]
    fn no_skip_if_equal_0x4xxx() {
        let mut cpu = Cpu::new();

        let inital_pc = cpu.pc;
        cpu.v[8] = 2;

        let opcode = 0x4802;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.pc, inital_pc);
    }

    #[test]
    fn skip_if_equal_0x5xxx() {
        let mut cpu = Cpu::new();

        let inital_pc = cpu.pc;
        cpu.v[8] = 2;
        cpu.v[3] = 2;

        let opcode = 0x5830;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.pc, inital_pc + 2);
    }

    #[test]
    fn no_skip_if_not_equal_0x5xxx() {
        let mut cpu = Cpu::new();

        let inital_pc = cpu.pc;
        cpu.v[8] = 2;
        cpu.v[3] = 5;

        let opcode = 0x5830;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.pc, inital_pc);
    }

    #[test]
    fn set_vx_0x6xxx() {
        let mut cpu = Cpu::new();

        let inital_pc = cpu.pc;
        cpu.v[0xA] = 0;

        let value = 0xBC;
        let opcode = 0x6A00 | value;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0xA] as u16, value);
    }

    #[test]
    fn set_vx_0x7xxx() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 1;

        let value = 0xBC;
        let opcode = 0x7A00 | value;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0xA] as u16, value + 1);
    }

    #[test]
    fn assign_vy_to_vx_0x8xx0() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 5;
        cpu.v[0x0] = 0;

        let opcode = 0x80A0;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0xA], cpu.v[0x0]);
    }

    #[test]
    fn vy_or_vx_0x8xx1() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 5;
        cpu.v[0x0] = 9;

        let opcode = 0x80A1;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 0xD);
    }

    #[test]
    fn vy_and_vx_0x8xx2() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 6;
        cpu.v[0x0] = 0xA;

        let opcode = 0x80A2;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 0x2);
    }

    #[test]
    fn vy_xor_vx_0x8xx3() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 0x5;
        cpu.v[0x0] = 0x9;

        let opcode = 0x80A3;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 0xC);
    }

    #[test]
    fn vy_add_vx_0x8xx4() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 5;
        cpu.v[0x0] = 9;

        let opcode = 0x80A4;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 14);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn vy_add_vx_with_vf_0x8xx4() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 254;
        cpu.v[0x0] = 9;

        let opcode = 0x80A4;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 7);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn vx_sub_vy_0x8xx5() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 10;
        cpu.v[0x0] = 30;

        let opcode = 0x80A5;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 20);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn vx_sub_vy_with_vf_0x8xx5() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 20;
        cpu.v[0x0] = 10;

        let opcode = 0x80A5;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 246);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn shr_0x8xx6() {
        let mut cpu = Cpu::new();

        cpu.v[0x0] = 10;

        let opcode = 0x80A6;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 5);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn shr_set_vf_0x8xx6() {
        let mut cpu = Cpu::new();

        cpu.v[0x0] = 11;

        let opcode = 0x80A6;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 5);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn vy_sub_vx_0x8xx7() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 30;
        cpu.v[0x0] = 10;

        let opcode = 0x80A7;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 20);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn vy_sub_vx_with_vf_0x8xx7() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 10;
        cpu.v[0x0] = 20;

        let opcode = 0x80A7;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 246);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn shl_with_vf_0x8xxe() {
        let mut cpu = Cpu::new();

        cpu.v[0x0] = 134;

        let opcode = 0x80AE;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 0xC);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn shl_0x8xxe() {
        let mut cpu = Cpu::new();

        cpu.v[0x0] = 6;

        let opcode = 0x80AE;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.v[0x0], 0xC);
        assert_eq!(cpu.v[0xF], 0);
    }

    #[test]
    fn skip_next_0x9xx0() {
        let mut cpu = Cpu::new();

        cpu.v[0xA] = 10;
        cpu.v[0x0] = 20;
        let initial_pc = cpu.pc;

        let opcode = 0x9A00;

        cpu.handle_opcode(opcode);
        assert_eq!(cpu.pc, initial_pc + 2);
    }
}

