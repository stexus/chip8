#![feature(thread_sleep_until)]
use chip8::hardware::*;
use chip8::wires::*;

use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc,
};
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

fn fetch(pc: &mut ProgramCounter) -> u16 {}
fn decode(raw_inst: u16) -> Instruction {}
//might just need a bool return (is_timer)
fn execute(inst: Instruction) -> ReturnCode {
    false
}

fn main() {
    let mut pc = ProgramCounter::new();
    let mut registers: [u8; 16] = [0; 16];
    let mut sound_timer = Timer::new();
    let mut delay_timer = Timer::new();
    let mut active_timers: u8 = 0;

    let timer_freq = 60;
    let timer_resolution_duration = Duration::from_secs_f32(1f32 / timer_freq as f32);

    // declare the frequency
    // set current instant
    // wait until next time + 1/frequency

    let instructions_per_second = INSTRUCTIONS_PER_SECOND;
    let duration_per_inst = Duration::from_secs_f32(1f32 / instructions_per_second as f32);

    let mut next_inst_time = Instant::now();
    loop {
        thread::sleep_until(next_inst_time);
        next_inst_time += duration_per_inst;
        let raw_instruction = fetch(&mut pc);
        let decoded_instruction: Instruction = decode(raw_instruction);
        let return_code = execute(decoded_instruction);
    }
}
