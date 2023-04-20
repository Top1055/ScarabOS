use core::arch::asm;
use crate::cli;
use crate::vga_buffer;
use crate::{print};

//command buffer
pub const KEYBOARD_BUFFER_SIZE: usize = 256;

#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack));
    result
}

fn buffcpy(src: [char; KEYBOARD_BUFFER_SIZE], mut dst: [char; KEYBOARD_BUFFER_SIZE], len: usize) {
    dst = ['\0'; KEYBOARD_BUFFER_SIZE];
    for letter in 0..len {
        dst[letter] = src[letter];
    }
}

pub fn keyboard_loop() {
    /*
    No need to initialize the keyboard controller
    as we did this in boot.asm
    */

    //setup command buffer
    let mut cmd_buffer: [char; KEYBOARD_BUFFER_SIZE] = ['\0'; KEYBOARD_BUFFER_SIZE];
    let mut cmd_buffer_hist: [char; KEYBOARD_BUFFER_SIZE] = ['\0'; KEYBOARD_BUFFER_SIZE];
    // Index for tracking buffer
    let mut cmd_length = 0;
    let mut cmd_length_hist = 0;
    let mut shift: bool = false;

    // Wait for input
    loop {
        // Poll the keyboard controller
        let status: u8 = unsafe { inb(0x64) };

        if status & 0x01 == 0 {
            continue;
        }
        
        // Read the input
        let data: u8 = unsafe { inb(0x60) };

        // check if shift is being held
        match data {
            0x2A | 0x36 => {
                shift = true;
            },
            0xAA | 0xB6 => {
                shift = false;
            },
            _ => {

            }
        }

        // Handle the input
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
            0x10 => {
                if shift {
                    'Q'
                } else {
                    'q'
                }
            },
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
            0x37 => '*',  // Numeric keypad multiplication symbol
            0x39 => ' ',  // Space bar
            0x4A => '-',  // Numeric keypad subtraction symbol
            0x4E => '+',  // Numeric keypad addition symbol
            0x53 => '.',  // Numeric keypad decimal point symbol
            0x4B => '4', // Left arrow
            0x4D => '6', // Right arrow
            0x48 => {

                // Clear current command
                for _ in 0..cmd_length {
                    vga_buffer::TERMINAL.lock().back();
                }

                // Copy last command to buffer
                buffcpy(cmd_buffer_hist, cmd_buffer, cmd_length_hist);
                cmd_length = cmd_length_hist;

                // Display command
                for i in 0..cmd_length {
                    print!("{}", cmd_buffer[i]);
                }

                continue;
            },
            0x0E => { // Backspace

                if cmd_length > 0 {

                    vga_buffer::TERMINAL.lock().back();
                    cmd_length -= 1;

                }

                continue;
            },

            // Enter key
            0x1C => {
                // Process command
                cli::process_cmd(cmd_length, cmd_buffer);

                // Logging previous command
                buffcpy(cmd_buffer, cmd_buffer_hist, cmd_length);
                cmd_length_hist = cmd_length;

                cmd_length = 0;
                continue;

            },

            // Ignore other keys
            _ => {
                //print!("data: {}", data);
                continue;
            }
        };

        // print char
        print!("{}", ascii_char);
        cmd_buffer[cmd_length] = ascii_char;
        cmd_length += 1;
        
    }
}
