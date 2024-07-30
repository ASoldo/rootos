#![no_std]
#![no_main]
#![feature(asm_const)]

use core::arch::global_asm;
use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; 1024] = [0; 1024];

global_asm!(include_str!("boot.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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
    serial_putchar('R');
    loop {}
}
