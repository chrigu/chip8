use std::fs::File;
use std::io::Read;
use::chip8;

fn main() {
    let mut file = File::open("./src/ibm_logo.ch8").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    chip8::decompile(&buffer)

}
