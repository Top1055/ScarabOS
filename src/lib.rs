#![feature(lang_items)]
#![no_std] // don't link the Rust standard library

mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern fn rust_main() {
    let mut terminal = vga_buffer::Terminal{
        row: 0,
        column: 0,
        color: 0,
        buffer: unsafe {&mut *(0xb8000 as *mut [u16; (vga_buffer::VGA_WIDTH as usize) * (vga_buffer::VGA_HEIGHT as usize) ])},
    };
    terminal.clear();
    terminal.print("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n");
    terminal.print("Hello World!");
    loop {}
}

use core::panic::PanicInfo;
// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {
}
