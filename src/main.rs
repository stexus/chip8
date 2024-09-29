#![feature(thread_sleep_until)]
use chip8::hardware::*;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};

fn timer_loop(timer_ref: Arc<Timer>, tx: &Receiver<T>) {
    if timer_ref.active() {}
    let freq = 60;
    let time_per_cycle = Duration::from_secs_f32(1f32 / freq as f32);
    let mut next_cycle_time = Instant::now();
    loop {
        thread::sleep_until(next_cycle_time);
        next_cycle_time += time_per_cycle;
    }
}
//
fn start_timer(timer: Arc<Timer>, delay: u8, tx: Sender<bool>) {
    thread::spawn(move || timer_loop());
}
fn main() {
    let registers: [u8; 16] = [0; 16];
    let sound_timer = Arc::new(Timer::new());
    let delay_timer = Arc::new(Timer::new());
    // declare the frequency
    // set current instant
    // wait until next time + 1/frequency
    let instructions_per_second = 700;
    let duration_per_inst = Duration::from_secs_f32(1f32 / instructions_per_second as f32);
    let mut next_inst_time = Instant::now();
    let (main_tx, main_rx) = mpsc::channel::<bool>();
    loop {
        thread::sleep_until(next_inst_time);
        next_inst_time += duration_per_inst;
        //do some processing
    }
}
