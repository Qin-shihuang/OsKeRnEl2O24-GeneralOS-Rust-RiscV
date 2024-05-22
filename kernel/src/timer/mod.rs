use log::info;
use riscv::register::{sie, sstatus, time};
use sbi::legacy::sbi_set_timer;

use crate::board::CLOCK_FREQ;

const INTERVAL: usize = CLOCK_FREQ / 100;

#[inline]
pub fn set_next_timeout() {
    sbi_set_timer((time::read() + INTERVAL) as u64);
}

pub fn tick() {
    static mut TICKS: usize = 0;
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            log::info!("{} ticks passed.", TICKS);
        }
    }
    set_next_timeout();
}

pub fn init() {
    unsafe {
        sie::set_stimer();
        sstatus::set_sie();
    }
    set_next_timeout();
    info!("timer initialized.");
}