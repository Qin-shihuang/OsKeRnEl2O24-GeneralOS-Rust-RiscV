use core::ptr::addr_of;

use crate::{config::MEMORY_END, mm::addr::{PhysAddr, PAGE_SIZE}, println};

pub mod addr;
mod frame;
mod heap;
mod layout;

pub fn init() {
    heap::init();
    extern "C" {
        static __kernel_end: u8;
    }
    frame::FRAME_ALLOCATOR.lock().init(
        PhysAddr(unsafe { addr_of!(__kernel_end) } as usize).into(),
        PhysAddr(MEMORY_END).into(),
    );
    alloc_test();
}

fn alloc_test() {
    let ppn = frame::FRAME_ALLOCATOR.lock().alloc(4, 256).unwrap();
    // println!("Allocated frame: {}", ppn);
    // frame::FRAME_ALLOCATOR.lock().debug_print();
    // test write
    let base_ptr = (PhysAddr::from(ppn).0) as *mut u64;
    let pattern = 0xdeadbeef0066ccffu64;
    (0..4 * PAGE_SIZE)
        .step_by(8)
        .for_each(|i| unsafe { 
            base_ptr.add(i).write_volatile(pattern);
            assert!(base_ptr.add(i).read_volatile() == pattern);
         });
    // test free
    frame::FRAME_ALLOCATOR.lock().dealloc(ppn, 4);
    // frame::FRAME_ALLOCATOR.lock().debug_print();
}
