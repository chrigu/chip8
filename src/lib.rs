pub mod cpu;
pub mod display;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, chip8!");
}

pub fn decompile(binary_buffer: &Vec<u8>) {
    println!("{:?}", binary_buffer);

    let opcodes = extract_opcodes(binary_buffer);

    for opcode in opcodes.into_iter() {
        println!("{:x}", opcode)
    }

}

fn extract_opcodes(binary_buffer: &Vec<u8>) -> Vec<u16> {
    let mut opcodes:Vec<u16> = Vec::new();

    for (index, _) in binary_buffer.into_iter().enumerate().step_by(2) {
        let opcode = bytes_to_opcode(binary_buffer[index + 1], binary_buffer[index]);
        opcodes.push(opcode)
    }

    opcodes
}

fn bytes_to_opcode(low_byte: u8, high_byte: u8) -> u16 {
    let opcode = ((high_byte as u16) << 8) | low_byte as u16;
    opcode
}
