//! Heap allocator.

use allocator::Allocator;
use log::info;

use crate::config::KERNEL_HEAP_SIZE;

// For 64MiB of memory, it will take 26 bits to represent each byte.
// So 32 bits are enough.
#[global_allocator]
static ALLOCATOR: Allocator<32> = Allocator::new();
static mut KERNEL_HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

/// Initialize the heap allocator.
///
/// This function initializes the heap allocator by adding the kernel heap memory range to the allocator.
///
/// This function should be called only once.
pub fn init() {
    unsafe {
        ALLOCATOR
            .lock()
            .add_size(KERNEL_HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
    info!(
        "Initialized {} KiB of kernel heap.",
        KERNEL_HEAP_SIZE / 1024
    );
}
