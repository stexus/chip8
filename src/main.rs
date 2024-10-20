#![feature(thread_sleep_until)]
use chip8::hardware::*;
use chip8::wires::*;

use std::array;
use std::thread;
use std::time::{Duration, Instant};

const INSTRUCTIONS_PER_SECOND: i32 = 700;
// fn timer_loop(timer_ref: Arc<Timer>, tx: &Receiver<T>) {
//     if timer_ref.active() {}
//     let freq = 60;
//     let time_per_cycle = Duration::from_secs_f32(1f32 / freq as f32);
//     let mut next_cycle_time = Instant::now();
//     loop {
//         thread::sleep_until(next_cycle_time);
//         next_cycle_time += time_per_cycle;
//     }
// }
enum ReturnCode {
    SoundTimer,
    DelayTimer,
    CondBranch,
    UncondBranch,
    Arithmetic,
}

fn fetch(pc: &mut ProgramCounter, mem: &Memory) -> Result<u8, Exception> {
    let raw_inst = mem.fetch_instruction(pc);
    match pc.inc() {
        Err(e) => Err(e),
        Ok(()) => Ok(raw_inst),
    }
}
// remember this is BIG ENDIAN
// raw_inst[0] = most significant byte  (mask with 0x00ff)
// raw_inst[1] = least significant byte (mask with 0xff00)
fn decode(raw_inst: u8) -> Result<Instruction, Exception> {
    // nibbles[0] will be the least significant bit
    let nibbles_raw: [u8; 4] = array::from_fn(|i| raw_inst & (0x000f << (i * 4)) >> (i * 4));
    let mut nibbles = nibbles_raw;
    nibbles.reverse();
    match &nibbles[3] {
        0 => match (nibbles[1], nibbles[0]) {
            (0xE, 0xE) => Instruction::new(Operation::RET, &nibbles),
            (0xE, 0) => Instruction::new(Operation::CLS, &nibbles),
            _ => Instruction::new(Operation::SYS, &nibbles),
        },
        1 => todo!(),
        2 => todo!(),
        3 => todo!(),
        4 => todo!(),
        5 => todo!(),
        6 => todo!(),
        7 => todo!(),
        8 => todo!(),
        9 => todo!(),
        0xA => todo!(),
        0xB => todo!(),
        0xC => todo!(),
        0xD => todo!(),
        0xE => todo!(),
        0xF => todo!(),
        _ => Err(Exception::BadOp),
    }
    Ok(())
}
//might just need a bool return (is_timer)
fn execute(inst: Instruction) -> Result<ReturnCode, Exception> {
    todo!()
}

fn main() -> Result<(), Exception> {
    // ========== PREAMBLE ======================
    let mut memory = Memory::new();
    let mut pc = ProgramCounter::new();
    let mut registers: [u8; 16] = [0; 16];
    let mut sound_timer = Timer::new();
    let mut delay_timer = Timer::new();
    let mut next_soundtimer_time = Instant::now();
    let mut next_delaytimer_time = Instant::now();

    let timer_freq = 60;
    let timer_resolution_duration = Duration::from_secs_f32(1f32 / timer_freq as f32);

    // declare the frequency
    // set current instant
    // wait until next time + 1/frequency

    let instructions_per_second = INSTRUCTIONS_PER_SECOND;
    let duration_per_inst = Duration::from_secs_f32(1f32 / instructions_per_second as f32);
    let mut next_inst_time = Instant::now();
    // ===========================================

    loop {
        thread::sleep_until(next_inst_time);
        // maybe macrofy this
        if sound_timer.active() {
            if Instant::now() >= next_soundtimer_time {
                sound_timer.dec();
                if sound_timer.active() {
                    next_soundtimer_time = Instant::now() + timer_resolution_duration;
                } else {
                    //play the sound
                    todo!();
                }
            }
        }
        if delay_timer.active() {
            if Instant::now() >= next_delaytimer_time {
                delay_timer.dec();
                if delay_timer.active() {
                    next_delaytimer_time = Instant::now() + timer_resolution_duration;
                }
            }
        }
        next_inst_time += duration_per_inst;
        let raw_inst = fetch(&mut pc, &memory)?;
        let decoded_instruction: Instruction = decode(raw_inst)?;
        let return_code = execute(decoded_instruction)?;
        match return_code {
            ReturnCode::SoundTimer => {
                next_soundtimer_time = Instant::now() + timer_resolution_duration
            }
            ReturnCode::DelayTimer => {
                next_delaytimer_time = Instant::now() + timer_resolution_duration
            }
            _ => (),
        }
    }
}
