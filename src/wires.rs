pub enum Operation {
    SYS,
    CLS,
    RET,
    JP,
}
pub struct Instruction {
    op: Operation,
    addr: u16,
    nibble: u8,
    x: u8,
    y: u8,
    kk: u8,
}

impl Instruction {
    pub fn new(op: Operation, nibbles: &[u8; 4]) -> Self {
        let addr: u16 =
            ((nibbles[2] as u16) << 8) | ((nibbles[1] as u16) << 4) | (nibbles[0] as u16);
        let nibble = nibbles[0];
        let x = nibbles[2];
        let y = nibbles[1];
        let kk = (nibbles[1] << 4) | nibbles[2];
        Instruction {
            op,
            addr,
            nibble,
            x,
            y,
            kk,
        }
    }
}
