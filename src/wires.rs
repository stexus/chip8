enum Operation {
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
