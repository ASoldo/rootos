use crate::serial_putchar;
use core::sync::atomic::{AtomicBool, Ordering};
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Heap size (in bytes) - ensure this is adequate
const HEAP_SIZE: usize = 64 * 1024; // Adjust this size if needed

#[link_section = ".heap"]
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

// To debug if the lock is being acquired or not
static LOCK_ACQUIRED: AtomicBool = AtomicBool::new(false);

pub fn init_heap() {
    unsafe {
        serial_putchar('I'); // Debugging

        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP_SIZE;

        serial_putchar('A'); // Before locking allocator

        // Attempt to lock the allocator and add debugging information
        if let Some(mut allocator) = ALLOCATOR.try_lock() {
            serial_putchar('B'); // After locking allocator
            allocator.init(heap_start, heap_size);
            serial_putchar('C'); // After initializing heap
        } else {
            serial_putchar('L'); // Locking failed
        }

        serial_putchar('O'); // End of initialization
    }
}
