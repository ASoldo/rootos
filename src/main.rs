#![no_std]
#![no_main]
#![feature(asm_const, alloc_error_handler)]

extern crate alloc;

use core::arch::global_asm;
use core::panic::PanicInfo;

mod allocator;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; 8192] = [0; 8192];

global_asm!(include_str!("boot.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_putchar('P');
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    serial_putchar('E');
    loop {}
}

fn serial_putchar(c: char) {
    let uart_addr = 0x0900_0000 as *mut u8;
    unsafe {
        *uart_addr = c as u8;
    }
}

fn serial_puts(s: &str) {
    for c in s.chars() {
        serial_putchar(c);
    }
}

#[no_mangle]
fn main() -> ! {
    // serial_putchar('1');
    // allocator::init_heap();
    // serial_putchar('2');
    // serial_puts("OJOH");

    serial_putchar('A');
    serial_putchar('B');
    serial_putchar('C');
    serial_putchar('D');
    serial_putchar('E');
    serial_putchar('F');
    serial_putchar('G');
    serial_putchar('H');
    serial_putchar('I');
    serial_putchar('J');
    serial_putchar('K');
    serial_putchar('L');
    serial_putchar('M');
    serial_putchar('N');
    serial_putchar('O');
    serial_putchar('P');
    serial_putchar('R');
    serial_putchar('S');
    serial_putchar('T');
    serial_putchar('Q');
    serial_putchar('U');
    serial_putchar('V');
    serial_putchar('W');
    serial_putchar('Z');
    serial_putchar('X');
    serial_putchar('Y');
    loop {}
}
