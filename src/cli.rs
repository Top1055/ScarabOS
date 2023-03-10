use crate::{print};
use crate::keyboard;


pub fn process_char(&ascii_char: char, &cmd_length: u8, &cmd_buffer: [char; keyboard::KEYBOARD_BUFFER_SIZE]) {
    print!("{}", ascii_char);

    //Check for enter
    if ascii_char == '\n' {
        //process command

        //echo
        if cmd_buffer.len() >= 6 && cmd_buffer[0..4] == ['e', 'c', 'h', 'o'] {
            print!("Scarab: ");
            for letter in 5..cmd_length {
                print!("{}", cmd_buffer[letter]);
            }
        } else {
            print!("Scarab: Command not found!");
        }
        print!("\n\n> ");
        cmd_buffer = 0; // Reset buffer
    } else {
        //store text
        cmd_buffer[cmd_buffer] = ascii_char as char;
        cmd_buffer = cmd_buffer  + 1;
    }
}