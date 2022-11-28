//use volatile:Volatile;

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

pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

fn make_color(fg: Color, bg: Color) -> u8 {
    return (fg as u8) | (bg as u8) << 4;
}

fn make_vga_entry(c: char, color: u8) -> u16 {
    let c16 = c as u16;
    let color16 = color as u16;
    return c16 | color16 << 8;
}

pub struct Terminal {
    pub row: usize,
    pub column: usize,
    pub color: u8,
    pub buffer: &'static mut [u16; ((VGA_WIDTH) * (VGA_HEIGHT))],
}

impl Terminal {

    fn put_entry_at(&mut self, c: char, color: u8, x: usize, y: usize) {
        let index = y * VGA_WIDTH + x;
        self.buffer[index] = make_vga_entry(c, color);
    }

    pub fn clear(&mut self) {
        self.row = 0;
        self.column = 0;
        // Need to move across from pub scope
        self.color = make_color(Color::LightGrey, Color::Black);

        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                self.put_entry_at(' ', make_color(Color::LightGrey, Color::Black), x, y);
            }
        }
    }

    pub fn scroll(&mut self) {
        for y in 0..VGA_HEIGHT-1 {
            for x in 0..VGA_WIDTH {
                let prev = (y * VGA_WIDTH) + x;
                let next = ((y + 1) * VGA_WIDTH) + x;
                self.buffer[prev] = self.buffer[next];
            }
        }

        self.row -= 1;
        self.column = 0;

        for x in 0..VGA_WIDTH {
            let index = (self.row * VGA_WIDTH) + x;
            self.put_entry_at(' ', make_color(Color::LightGrey, Color::Black), x, self.row);
        }

    }
    pub fn put_char(&mut self, c: char) {
        if c == '\n' {
            
            self.row += 1;
            if self.row >= VGA_HEIGHT {
                self.scroll();
            }

            self.column = 0;

        } else {

            self.put_entry_at(c, self.color, self.column, self.row);
            self.column += 1;
            if(self.column == VGA_WIDTH) {
                self.column = 0;
                
                self.row += 1;
                if (self.row > VGA_HEIGHT) {
                    self.scroll();
                }
            }
        }

    }
    pub fn print(&mut self, data: &str) {
        for c in data.chars() {
            self.put_char(c);
        }
    }
}
