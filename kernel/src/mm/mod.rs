use core::ptr::addr_of;

use alloc::vec::Vec;
use log::{debug, info};

use crate::config::{PHYSICAL_MEMORY_END, PHYSICAL_MEMORY_START};

use self::addr::{kva2pa, PhysAddr, VirtAddr};

pub mod addr;
pub mod consts;
mod frame;
mod heap;
pub mod layout;
mod paging;

extern "C" {
    static __kernel_end: u8;
}

pub fn init() {
    heap::init();
    test_heap();
    frame::init(
        kva2pa(VirtAddr(unsafe { addr_of!(__kernel_end) as usize })),
        PhysAddr(PHYSICAL_MEMORY_END),
    );
    layout::print_memory_layout();
    paging::pagetable::map_kernel_phys_seg();
    info!("Physical memory mapped at 0x{:x}", PHYSICAL_MEMORY_START);

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
