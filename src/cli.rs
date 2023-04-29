use crate::{print, println, vec};
use crate::vga_buffer::{TERMINAL, Color};
use crate::vec::Vec;

fn find_seperator(cmd: &Vec::<char>) -> usize {

    // Finds first space in cmd
    for i in 0..cmd.len() {
        if cmd[i] == ' ' {
            return i;
        }
    }

    // if unable to find space
    return 0;
}

fn vec_char_starts_with(vec: &Vec<char>, s: &str, n: usize) -> bool {
    if vec.len() < n {
        return false;
    }

    for i in 0..n {
        if vec[i] != s.as_bytes()[i] as char {
            return false;
        }
    }

    true
}

fn vec_char_range_match(vec: &Vec<char>, s: &str, a: usize, b: usize) -> bool {
    if vec.len() < a || vec.len() < b {
        return false;
    }

    for i in a..b {
        if vec[i] != s.as_bytes()[i-a] as char {
            return false;
        }
    }

    true
}

pub fn process_cmd (
    cmd: Vec::<char>) {

    if cmd.len() == 0 {
        print!("\n> ");
        return;
    }

    print!("\n");

    // Location of space in commands with arguments
    let seperator = find_seperator(&cmd);

    if seperator == 0 {

        check_single_commands(&cmd);

    } else {

        check_arg_commands(&cmd, seperator);

    }

    print!("\n> ");
}

fn check_single_commands(cmd: &Vec<char>) {

    if vec_char_starts_with(cmd, "clear", cmd.len()) {
        TERMINAL.lock().clear();

    } else if vec_char_starts_with(cmd, "error", cmd.len()) {

        // This will fail, it is just here
        // To produce a panic for debugging
        let mut fail = vec![1];
        fail[2] = 2;

    } else {
        println!("Command not found!");
    }
}

fn check_arg_commands(cmd: &Vec<char>, seperator: usize) {

    if vec_char_starts_with(cmd, "echo ", seperator) {

        print!("Scarab: ");
        for letter in seperator+1..cmd.len() {
            print!("{}", cmd[letter]);
        }

    } else if vec_char_starts_with(cmd, "color ", seperator){

        let mut color: Color = Color::White;

        if vec_char_range_match(cmd, "red", seperator+1, cmd.len()) {
            color = Color::Red;

        } else if vec_char_range_match(cmd, "cyan", seperator+1, cmd.len()) {
            color = Color::Cyan;

        } else if vec_char_range_match(cmd, "magenta", seperator+1, cmd.len()) {
            color = Color::Magenta;

        } else if vec_char_range_match(cmd, "green", seperator+1, cmd.len()) {
            color = Color::Green;

        } else if vec_char_range_match(cmd, "brown", seperator+1, cmd.len()) {
            color = Color::Brown;

        } else if vec_char_range_match(cmd, "light magenta", seperator+1, cmd.len()) {
            color = Color::LightMagenta;

        } else if vec_char_range_match(cmd, "white", seperator+1, cmd.len()) {
            color = Color::White;

        } else {
            println!("Unable to set color");
        }

        TERMINAL.lock().set_color(
            color,
            Color::Black
        );

    } else {

        println!("Command not found!");

    }

}
