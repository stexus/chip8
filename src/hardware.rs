use std::sync::Mutex;
pub struct Display {
    w_pixels: [bool; 64],
    v_pixels: [bool; 32],
}
pub struct Memory {
    // byte-addressable, 4KB
    sp: u8,
    mem: [u8; 4096],
}
pub struct Timer {
    delay: u8,
}
pub struct ProgramCounter {
    counter: u16,
}

#[derive(Debug)]
pub enum Exception {
    PCOverflow,
    BadOp,
}
impl ProgramCounter {
    pub fn new() -> Self {
        ProgramCounter { counter: 0x200 }
    }
    pub fn get(&self) -> u16 {
        self.counter
    }
    pub fn inc(&mut self) -> Result<(), Exception> {
        let max_pc = 2u16.pow(12) - 1;
        if self.counter == max_pc {
            Err(Exception::PCOverflow)
        } else {
            self.counter += 2;
            Ok(())
        }
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            sp: 0,
            mem: [0; 4096],
        }
    }
    pub fn fetch_instruction(&self, pc: &mut ProgramCounter) -> u8 {
        let lo = self.mem[pc.get() as usize];
        let hi = self.mem[pc.get() as usize + 1];
        hi << 4 | lo
    }
}

impl Timer {
    pub fn new() -> Self {
        Timer { delay: 0 }
    }
    pub fn active(&self) -> bool {
        self.delay > 0
    }
    pub fn dec(&mut self) -> u8 {
        if self.delay > 0 {
            self.delay -= 1;
            return self.delay;
        } else {
            0
        }
    }
}
