#![feature(lang_items)]
#![no_std] // don't link the Rust standard library

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern fn rust_main() {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
}
