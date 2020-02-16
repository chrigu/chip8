pub struct Cpu {

    memory: [u8, 4096];
    v: [u8, 16];
    pc: u16;
    sp: u8;

    stack: [u16, 16];
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
        }
    }

    pub emulate_cycle(&self) {
        
    }
}

