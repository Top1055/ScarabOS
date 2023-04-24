use core::arch::asm;
use crate::cli;
use crate::vga_buffer;
use crate::{print};
use crate::vec::Vec;

// Reading ports using asm
#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nomem, nostack));
    result
}

fn get_key() -> u8 {

    unsafe {
        loop {

            // Poll the keyboard controller
            let status: u8 = inb(0x64);

            if status & 0x01 == 0 {
                continue;
            }
            
            // Read the input
            return inb(0x60);

        }
    }
}

pub fn keyboard_loop() {

    /*

        No need to initialize the keyboard controller
        as we did this in boot.asm

    */

    // Track keyboard inputs
    let mut cmd = Vec::<char>::new();

    //tracks if shift is held
    let mut shift: bool = false;

    // Wait for input
    loop {

        let scancode = get_key();

        // check if shift is being held
        match scancode {
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
        let key = if shift {

            translate_upper_alphanum(scancode)

        } else {

            translate_alphanum(scancode)

        };

        if key != '\0' {

            // print char
            print!("{}", key);
            cmd.push(key);

        } else {

            // Other key press functions
            match scancode {
                0x4B => {   // Left arrow

                        //TODO

                    },
                0x4D => {   // Right arrow

                        //TODO

                    },
                0x0E => {   // Backspace

                    if cmd.len() > 0 {

                        vga_buffer::TERMINAL.lock().back(1);
                        cmd.pop();

                    }

                },
                0x1C => {   // Enter

                    // Process command
                    cli::process_cmd(cmd);

                    cmd = Vec::<char>::new(); // Pops all elements and frees memory

                },
                _ => {      // Ignore other keys
                    //print!("scancode: {}", scancode);
                }
            };

        }
    }
}

fn translate_alphanum(code: u8) -> char {
    return match code {
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
        0x28 => '\'',   // Single quote
        0x29 => '`',    // Grave accent
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
        0x35 => '/',    // Forward slash
        0x37 => '*',    // Numeric keypad multiplication symbol
        0x39 => ' ',    // Space bar
        0x4A => '-',    // Numeric keypad subtraction symbol
        0x4E => '+',    // Numeric keypad addition symbol
        0x53 => '.',    // Numeric keypad decimal point symbol
        _ => '\0',      // unregistered
    }
}

fn translate_upper_alphanum(code: u8) -> char {
    return match code {
        0x02 => '!',
        0x03 => '@',
        0x04 => '#',
        0x05 => '$',
        0x06 => '%',
        0x07 => '^',
        0x08 => '&',
        0x09 => '*',
        0x0A => '(',
        0x0B => ')',
        0x0C => '_',
        0x0D => '+',
        0x10 => 'Q',
        0x11 => 'W',
        0x12 => 'E',
        0x13 => 'R',
        0x14 => 'T',
        0x15 => 'Y',
        0x16 => 'U',
        0x17 => 'I',
        0x18 => 'O',
        0x19 => 'P',
        0x1A => '{',
        0x1B => '}',
        0x1E => 'A',
        0x1F => 'S',
        0x20 => 'D',
        0x21 => 'F',
        0x22 => 'G',
        0x23 => 'H',
        0x24 => 'J',
        0x25 => 'K',
        0x26 => 'L',
        0x27 => ':',
        0x28 => '"',
        0x29 => '~',    // Tilde
        0x2B => '|',
        0x2C => 'Z',
        0x2D => 'X',
        0x2E => 'C',
        0x2F => 'V',
        0x30 => 'B',
        0x31 => 'N',
        0x32 => 'M',
        0x33 => '<',
        0x34 => '>',
        0x35 => '?',    // Forward slash
        0x37 => '*',    // Numeric keypad multiplication symbol
        0x39 => ' ',    // Space bar
        0x4A => '-',    // Numeric keypad subtraction symbol
        0x4E => '+',    // Numeric keypad addition symbol
        0x53 => '.',    // Numeric keypad decimal point symbol
        _ => '\0',      // unregistered
    }
}
