#![no_std]
#![no_main]
#![feature(asm_const, alloc_error_handler)]

extern crate alloc;

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::write_volatile;

mod allocator;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; 4096] = [0; 4096];

global_asm!(include_str!("boot.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_putchar('P'); // Panic indicator
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    serial_putchar('E'); // Alloc error indicator
    loop {}
}

fn serial_putchar(c: char) {
    let uart_addr = 0x0900_0000 as *mut u8;
    unsafe {
        *uart_addr = c as u8;
    }
}

#[no_mangle]
fn main() -> ! {
    serial_putchar('1');
    // allocator::init_heap();
    serial_putchar('2');
    loop {}
}
