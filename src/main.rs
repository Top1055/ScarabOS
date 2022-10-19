#![no_std]      // Making sure we aren't using any OS reliant code
#![no_main]     // Disabling runtime checks that are os based

use core::panic::PanicInfo;

// called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // Keep function name unchanged on compile
pub extern "C" fn _start() -> ! {
    // This function is called first by the linker
    loop {}
}
