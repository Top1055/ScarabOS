use core::arch::asm;
use crate::{print, println};

#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack));
    result
}

//command buffer
const BUFFER_SIZE: usize = 256;

pub extern fn init() {
    print!("\n\nWelcome to ScarabOS\n\n> ");

    /*
    No need to initialize the keyboard controller
    as we did this in boot.asm
    */

    //setup command buffer
    let mut cmd_buffer: [char; BUFFER_SIZE] = ['\0'; BUFFER_SIZE];
    // Index for tracking buffer
    let mut i = 0;

    // Wait for input
    loop {
        // Poll the keyboard controller
        let mut status: u8 = 0;
        unsafe {
            status = inb(0x64);
        };
        if status & 0x01 == 0 {
            continue;
        }
        
        // Read the input
        let mut data: u8 = 0;
        unsafe {
            data = inb(0x60);
        };

        // Handle the input
        if data < 0x80 {
            // Key was pressed
            let ascii_char = match data {
                0x02 => '1',
                0x03 => '2',
                0x04 => '3',
                0x05 => '4',
                0x06 => '5',
                0x07 => '6',
                0x08 => '7',
                0x09 => '8',
                0x0A => '9',
                0x0B => '0',
                0x0C => '-',
                0x0D => '=',
                0x10 => 'q',
                0x11 => 'w',
                0x12 => 'e',
                0x13 => 'r',
                0x14 => 't',
                0x15 => 'y',
                0x16 => 'u',
                0x17 => 'i',
                0x18 => 'o',
                0x19 => 'p',
                0x1A => '[',
                0x1B => ']',
                0x1C => '\n', // Enter key
                0x1D => '\0', // Left Control key
                0x1E => 'a',
                0x1F => 's',
                0x20 => 'd',
                0x21 => 'f',
                0x22 => 'g',
                0x23 => 'h',
                0x24 => 'j',
                0x25 => 'k',
                0x26 => 'l',
                0x27 => ';',
                0x28 => '\'', // Single quote
                0x29 => '`',  // Grave accent
                0x2A => '\0', // Left Shift key
                0x2B => '\\',
                0x2C => 'z',
                0x2D => 'x',
                0x2E => 'c',
                0x2F => 'v',
                0x30 => 'b',
                0x31 => 'n',
                0x32 => 'm',
                0x33 => ',',
                0x34 => '.',
                0x35 => '/',  // Forward slash
                0x36 => '\0', // Right Shift key
                0x37 => '*',  // Numeric keypad multiplication symbol
                0x38 => '\0', // Left Alt key
                0x39 => ' ',  // Space bar
                0x3A => '\0', // Caps Lock key
                0x3B => '\0', // F1 key
                0x3C => '\0', // F2 key
                0x3D => '\0', // F3 key
                0x3E => '\0', // F4 key
                0x3F => '\0', // F5 key
                0x40 => '\0', // F6 key
                0x41 => '\0', // F7 key
                0x42 => '\0', // F8 key
                0x43 => '\0', // F9 key
                0x44 => '\0', // F10 key
                0x45 => '\0', // Num Lock key
                0x46 => '\0', // Scroll Lock key
                0x47 => '7',
                0x48 => '8',
                0x49 => '9',
                0x4A => '-',  // Numeric keypad subtraction symbol
                0x4B => '4',
                0x4C => '5',
                0x4D => '6',
                0x4E => '+',  // Numeric keypad addition symbol
                0x4F => '1',
                0x50 => '2',
                0x51 => '3',
                0x52 => '0',
                0x53 => '.',  // Numeric keypad decimal point symbol
                0x56 => '\0', // Keyboard non-US backslash and vertical bar key
                0x57 => '\0', // F11 key
                0x58 => '\0', // F12 key
                _ => '\0',    // Ignore all other keys
            };
            print!("{}", ascii_char);

            //Check for enter
            if ascii_char == '\n' {
                //process command

                //echo
                if cmd_buffer.len() >= 6 && cmd_buffer[0..4] == ['e', 'c', 'h', 'o'] {
                    for letter in 5..i {
                        print!("{}", cmd_buffer[letter]);
                    }
                    print!("\n> ");
                }
                i = 0;
            } else {
                //store text
                cmd_buffer[i] = ascii_char as char;
                i = i + 1;
            }
        }
    }
}