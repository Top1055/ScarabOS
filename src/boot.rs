use crate::{print};
use crate::keyboard;

pub extern fn init() {
    print!("\n\nWelcome to ScarabOS\n\n> ");
    keyboard::keyboard_loop();
}