use crate::serial_putchar;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Heap size (in bytes)
const HEAP_SIZE: usize = 4096;

#[link_section = ".bss.heap"]
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

/// Initialize the heap allocator
pub fn init_heap() {
    unsafe {
        serial_putchar('I'); // Start heap init debug character
        let heap_start = HEAP.as_mut_ptr();
        let heap_size = HEAP_SIZE;
        if heap_start.is_null() {
            serial_putchar('X'); // Indicates heap_start is null
        } else {
            serial_putchar('Y'); // Indicates heap_start is not null
        }
        serial_putchar('A'); // Before locking allocator
        let mut allocator = ALLOCATOR.lock();
        serial_putchar('B'); // After locking allocator
        allocator.init(heap_start, heap_size);
        serial_putchar('C'); // After initializing heap
        serial_putchar('O'); // End heap init debug character
    }
}
