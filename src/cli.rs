use crate::{print};
use crate::vga_buffer;
use crate::keyboard;

pub fn process_cmd (
    cmd_length: usize,
    cmd_buffer: [char; keyboard::KEYBOARD_BUFFER_SIZE]) {

    //TODO implement match
    //TODO implement struct

    print!("\n");

    // Location of space in commands with arguments
    let mut seperator = 0;

    for i in 1..cmd_length {
        if cmd_buffer[i] == ' ' {
            seperator = i;
        }
    }

    if seperator == 0 {
        /*
            Single word commands

            - clear
        */

        match cmd_buffer[0..cmd_length] {

            ['c', 'l', 'e', 'a', 'r'] => {

                vga_buffer::TERMINAL.lock().clear();

            },
            ['H', 'e', 'l', 'l', 'o', '!'] => {

                print!("Scarab: Hi there!\n");

            }
            _ => { // Unkown command

                print!("Command not found! '");

            }
        };

    } else {
        /*
            Argument commands

            - echo
        */
        
        match cmd_buffer[0..seperator] {
            ['e', 'c', 'h', 'o'] => {
                
                print!("Scarab: ");
                // Print argument
                for letter in seperator+1..cmd_length {
                    print!("{}", cmd_buffer[letter]);
                }

            },
            ['c', 'o', 'l', 'o', 'r'] => {

                match cmd_buffer[seperator+1..cmd_length] {
                    ['r', 'e', 'd'] => {
                        vga_buffer::TERMINAL.lock().set_color(
                            vga_buffer::Color::Red,
                            vga_buffer::Color::Black
                        );
                    }
                    _ => {

                        print!("unable to set color\n");

                    }
                }

            }
            _ => { // Unkown command

                print!("Command not found!");

            }
        };
    }

    print!("\n\n> ");
}
