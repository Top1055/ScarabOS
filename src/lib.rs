#![no_std] // don't link the Rust standard library
#![no_main]

pub mod boot;
pub mod vga_buffer;
pub mod keyboard;
pub mod cli;
pub mod scalloc;
pub mod vec;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main() -> ! {
    boot::init();
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_buffer::TERMINAL.lock().panic();
    print!("
 _   __                     _  ______           _           __
| | / /                    | | | ___ \\         (_)       _ / /
| |/ /  ___ _ __ _ __   ___| | | |_/ /_ _ _ __  _  ___  (_) |
|    \\ / _ \\ '__| '_ \\ / _ \\ | |  __/ _` | '_ \\| |/ __|   | |
| |\\  \\  __/ |  | | | |  __/ | | | | (_| | | | | | (__   _| |
\\_| \\_/\\___|_|  |_| |_|\\___|_| \\_|  \\__,_|_| |_|_|\\___| (_) |
                                                           \\_\\

");
    println!("{}", info);
    loop {}
}
