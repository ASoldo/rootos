use crate::serial_putchar;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Heap size (in bytes) - ensure this is adequate
const HEAP_SIZE: usize = 4096; // Adjust this size if needed

#[link_section = ".bss.heap"]
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        serial_putchar('I'); // Debugging

        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP_SIZE;

        serial_putchar('A'); // Before locking allocator
        let mut allocator = ALLOCATOR.lock();
        serial_putchar('B'); // After locking allocator

        allocator.init(heap_start, heap_size);
        serial_putchar('C'); // After initializing heap

        serial_putchar('O'); // End of initialization
    }
}
