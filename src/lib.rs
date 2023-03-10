#![no_std] // don't link the Rust standard library
#![no_main]

pub mod boot;
pub mod vga_buffer;
pub mod keyboard;
pub mod cli;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main() -> ! {
    boot::init();
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}