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

struct PCOverflow;
impl ProgramCounter {
    pub fn new() -> Self {
        ProgramCounter { counter: 0 }
    }
    pub fn inc(&mut self) -> Result<(), PCOverflow> {
        let max_pc = 2u16.pow(12) - 1;
        if self.counter == max_pc {
            Err(PCOverflow)
        } else {
            self.counter += 2;
            Ok(())
        }
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
