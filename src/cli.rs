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
            break;
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

            }
            _ => { // Unkown command

                print!("Command not found!");

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

            }
            ['c', 'o', 'l', 'o', 'r'] => {

                let color = match cmd_buffer[seperator+1..cmd_length] {
                    ['b', 'l', 'u', 'e'] => vga_buffer::Color::Blue,
                    ['g', 'r', 'e', 'e', 'n'] => vga_buffer::Color::Green,
                    ['c', 'y', 'a', 'n'] => vga_buffer::Color::Cyan,
                    ['r', 'e', 'd'] => vga_buffer::Color::Red,
                    ['m', 'a', 'g', 'e', 'n', 't', 'a'] => vga_buffer::Color::Magenta,
                    ['b', 'r', 'o', 'w', 'n'] => vga_buffer::Color::Brown,

                    _ => {

                        print!("unable to set color\n");
                        vga_buffer::Color::White

                    }
                };

                vga_buffer::TERMINAL.lock().set_color(
                    color,
                    vga_buffer::Color::Black
                );

            }
            _ => { // Unkown command

                print!("Command not found!");

            }
        };
    }

    print!("\n\n> ");
}
