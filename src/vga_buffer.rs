extern crate volatile;
extern crate lazy_static;
extern crate spin;

use self::volatile::Volatile;
use self::lazy_static::lazy_static;
use self::spin::Mutex;
use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGrey = 7,
	DarkGrey = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	LightMagenta = 13,
	LightBrown = 14,
	White = 15,
}

lazy_static! {
    pub static ref TERMINAL: Mutex<Terminal> = Mutex::new(Terminal {
        row: 0,
        column: 0,
        color: make_color(Color::White, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut [Volatile<u16>; VGA_WIDTH * VGA_HEIGHT ])},
    });
}

// Dimensions for the screen
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

// converting enum to u8
pub fn make_color(fg: Color, bg: Color) -> u8 {
    return (fg as u8) | (bg as u8) << 4; // big brain
}

// just makes converting easier
fn make_vga_entry(c: char, color: u8) -> u16 {
    let c16 = c as u16;
    let color16 = color as u16;
    return c16 | color16 << 8;
}

pub struct Terminal {
    row: usize,
    column: usize,
    color: u8,
    buffer: &'static mut [Volatile<u16>; (VGA_WIDTH) * (VGA_HEIGHT)],
}

impl Terminal {

    fn put_entry_at(&mut self, c: char, color: u8, x: usize, y: usize) {
        let index = y * VGA_WIDTH + x;
        self.buffer[index].write(make_vga_entry(c, color));
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = make_color(fg, bg);
    }

    //For removing characters, aka backspace functionality
    pub fn back(&mut self, len: usize) {

        // remove current cursor
        self.update_cursor(false);

        for _ in 0..len {

            // if go back 
            if self.column <= 0 && self.row > 0 {

                // Undo a new line
                self.column = 0;
                self.row -= 1;

            } else {

                self.column -= 1;

            }

            // Clear space and draw cursor
            self.put_entry_at(' ', self.color, self.column, self.row);
            self.update_cursor(true);

        }
    }

    // To empty all contents of vga buffer
    pub fn clear(&mut self) {
        self.row = 0;
        self.column = 0;

        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                // Always sets bg black, could be later issue
                self.put_entry_at(' ', make_color(Color::White, Color::Black), x, y);
            }
        }
    }

    fn scroll(&mut self) {
        for y in 0..VGA_HEIGHT-1 {
            for x in 0..VGA_WIDTH {
                let prev = (y * VGA_WIDTH) + x;
                let next = ((y + 1) * VGA_WIDTH) + x;
                // Transfer all contents up one row
                self.buffer[prev].write(self.buffer[next].read());
            }
        }

        self.row -= 1;
        self.column = 0;

        // Create empty last line
        for x in 0..VGA_WIDTH {
            self.put_entry_at(' ', make_color(Color::LightGrey, Color::Black), x, self.row);
        }

    }

    fn put_char(&mut self, c: char) {
        if c == '\n' {
            
            self.row += 1;
            if self.row >= VGA_HEIGHT {
                self.scroll();
            }

            self.column = 0;

        } else {

            self.put_entry_at(c, self.color, self.column, self.row);
            self.column += 1;
            if self.column == VGA_WIDTH {
                self.column = 0;
                
                self.row += 1;
                if self.row >= VGA_HEIGHT {
                    self.scroll();
                }
            }
        }
    }

    pub fn print(&mut self, data: &str) {

        self.update_cursor(false);

        for c in data.chars() {
            self.put_char(c);
        }

        self.update_cursor(true);
    }

    pub fn update_cursor(&mut self, visable: bool) {

        // Can change these depends how I feel
        let color = if visable { make_color(Color::Black, Color::White) } else { self.color };
        let c_char = ' ';

        self.put_entry_at(c_char, color, self.column, self.row);

    }

    // Empties contents and replaces with red screen (scary!)
    pub fn panic(&mut self) {
        self.row = 0;
        self.column = 0;
        self.set_color(Color::White, Color::Red);

        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                self.put_entry_at(' ', make_color(Color::White, Color::Red), x, y);
            }
        }
    }

    pub fn color_test(&mut self) {
        let backup_color = self.color;
        
        self.color = make_color(
            Color::White, Color::Black
        );
        self.print(" White + Black ");

        self.color = make_color(
            Color::White, Color::Blue
        );
        self.print(" White + Blue ");

        self.color = make_color(
            Color::White, Color::Green
        );
        self.print(" White + Green ");

        self.color = make_color(
            Color::Black, Color::Cyan
        );
        self.print(" Black + Cyan ");

        self.color = make_color(
            Color::White, Color::Red
        );
        self.print(" White + Red ");

        self.color = make_color(
            Color::White, Color::Magenta
        );
        self.print(" White + Magenta ");

        self.color = make_color(
            Color::White, Color::Brown
        );
        self.print(" White + Brown ");

        self.color = make_color(
            Color::Black, Color::LightGrey
        );
        self.print(" Black + LightGrey ");

        self.color = make_color(
            Color::White, Color::DarkGrey
        );
        self.print(" White + DarkGrey ");

        self.color = make_color(
            Color::Black, Color::LightBlue
        );
        self.print(" Black + LightBlue ");

        self.color = make_color(
            Color::Black, Color::LightGreen
        );
        self.print(" Black + LightGreen ");

        self.color = make_color(
            Color::Black, Color::LightCyan
        );
        self.print(" Black + LightCyan ");

        self.color = make_color(
            Color::Black, Color::LightRed
        );
        self.print(" Black + LightRed ");

        self.color = make_color(
            Color::Black, Color::LightMagenta
        );
        self.print(" Black + LightMagenta ");

        self.color = make_color(
            Color::Black, Color::LightBrown
        );
        self.print(" Black + LightBrown ");

        self.color = make_color(
            Color::Black, Color::White
        );
        self.print(" Black + White ");

        self.color = backup_color;

    }

}

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

// Macros
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    TERMINAL.lock().write_fmt(args).unwrap();
}
