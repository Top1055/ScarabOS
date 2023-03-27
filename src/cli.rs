use crate::{print};
use crate::keyboard;

pub fn process_cmd (
    cmd_length: usize,
    cmd_buffer: [char; keyboard::KEYBOARD_BUFFER_SIZE]) {

    //TODO implement match
    //TODO implement struct

    //echo
    if cmd_buffer[0..5] == ['e', 'c', 'h', 'o', ' '] {

        print!("Scarab: ");
        // Print argument
        for letter in 5..cmd_length {
            print!("{}", cmd_buffer[letter]);
        }

    } else { // no command

        print!("Scarab: Command not found!\n'");
        for letter in 0..cmd_length {
            print!("{}", cmd_buffer[letter]);
        }
        print!("'");
    }
    print!("\n\n> ");
}
