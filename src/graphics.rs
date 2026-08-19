use crate::port::inb;
use crate::vga_buffer::{make_color, Color, TERMINAL, VGA_HEIGHT, VGA_WIDTH};
use crate::{print, println, vec};
use alloc::vec::Vec;

static BACKGROUND: Color = Color::DarkGrey;
static FOREGROUND: Color = Color::Green;

// Golden formula
// (x, y, z)
// x' = x/z
// y' = y/z

#[derive(Clone, Copy)]
struct Pos2 {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy)]
struct Pos3 {
    x: f32,
    y: f32,
    z: f32,
}

fn screen(p: Pos2) -> Pos2 {
    return Pos2 {
        x: (p.x + 1.0) / 2.0 * (VGA_WIDTH as f32),
        y: (1.0 - (p.y + 1.0) / 2.0) * ((VGA_HEIGHT) as f32 * 2.0),
    };
}

fn project(p: Pos3) -> Pos2 {
    if p.z == 0.0 {
        return Pos2 { x: 0.0, y: 0.0 };
    }
    return Pos2 {
        x: p.x / p.z,
        y: p.y / p.z,
    };
}

fn timeout() {
    loop {
        unsafe {
            let status = inb(0x3DA);
            let bit_is_set = (status & (1 << 3)) != 0;
            println!("status: {}", bit_is_set);
        }
    }
}

pub fn cube() {
    timeout();
}
