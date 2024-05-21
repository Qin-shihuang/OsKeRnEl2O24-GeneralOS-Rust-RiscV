#![no_std]
extern crate alloc;
extern crate spin;

use alloc::alloc::{GlobalAlloc, Layout};
use core::ops::Deref;
use spin::Mutex;

mod buddy;
mod list;

use buddy::Heap;

pub struct Allocator<const ORDER: usize>(Mutex<Heap<ORDER>>);

/// A generic allocator implementation.
impl<const ORDER: usize> Allocator<ORDER> {
    pub const fn new() -> Self {
        Allocator(Mutex::new(Heap::new()))
    }
}

impl<const ORDER: usize> Default for Allocator<ORDER> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const ORDER: usize> Deref for Allocator<ORDER> {
    type Target = Mutex<Heap<ORDER>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Implements the `GlobalAlloc` trait for the `Allocator` struct.
unsafe impl<const ORDER: usize> GlobalAlloc for Allocator<ORDER> {
    /// Allocates a block of memory with the given layout.
    /// If successful, returns a pointer to the block of memory, else returns a null pointer.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |ptr| ptr.as_ptr())
    }

    /// Deallocates the block of memory pointed to by `ptr` with the given layout.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .dealloc(core::ptr::NonNull::new_unchecked(ptr), layout)
    }
}
