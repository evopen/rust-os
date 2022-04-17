#![no_std]
#![no_main]
#![feature(const_mut_refs)]

mod vga_buffer;

static HELLO: &[u8] = b"hello world";

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}\nfuck", "!");
    panic!("panicking");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
