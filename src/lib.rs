#![feature(lang_items)]
#![no_std] // don't link the Rust standard library

mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern fn rust_main() {
    println!("\n\n");
    let x = 3 + 5;
    println!("Hello World! x = {}", x);
    loop {}
}

use core::panic::PanicInfo;
// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
}
