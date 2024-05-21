use core::ptr::addr_of;

use alloc::vec::Vec;
use log::debug;

use crate::config::MEMORY_END;

pub mod addr;
mod frame;
mod heap;
pub mod layout;

extern "C" {
    static __kernel_end: u8;
}

pub fn init() {
    heap::init();
    test_heap();
    frame::init(unsafe { addr_of!(__kernel_end) as usize }, MEMORY_END);

}

fn test_heap() {
    let mut v = Vec::new();
    for i in 0..100 {
        v.push(i);
    }
    v.iter().enumerate().take(100).for_each(|(i, &x)| {
        assert_eq!(i, x);
    });
    debug!("Heap test passed.")
}