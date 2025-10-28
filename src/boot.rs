use crate::keyboard;
use crate::vga_buffer;
use crate::{print, println};

pub extern "C" fn init() {
    // Draw text in brown
    vga_buffer::TERMINAL
        .lock()
        .set_color(vga_buffer::Color::Brown, vga_buffer::Color::Black);

    println!(" _____                     _     _____ _____");
    println!("/  ___|                   | |   |  _  /  ___|");
    println!("\\ `--.  ___ __ _ _ __ __ _| |__ | | | \\ `--.");
    println!(" `--. \\/ __/ _` | '__/ _` | '_ \\| | | |`--. \\");
    println!("/\\__/ / (_| (_| | | | (_| | |_) \\ \\_/ /\\__/ /");
    println!("\\____/ \\___\\__,_|_|  \\__,_|_.__/ \\___/\\____/");

    // Reset Color
    vga_buffer::TERMINAL
        .lock()
        .set_color(vga_buffer::Color::White, vga_buffer::Color::Black);

    print!("try 'help'\n\n> ");
    keyboard::keyboard_loop();
}
