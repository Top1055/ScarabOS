#![no_std]      // Making sure we aren't using any OS reliant code
#![no_main]     // Disabling runtime checks that are os based

use core::panic::PanicInfo;

// called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Keep function name unchanged on compile
pub extern "C" fn _start() -> ! {
    // This function is called first by the linker

    //VGA text buffer
    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
