use crate::port::inb;
use crate::vga_buffer::{make_color, Color, TERMINAL, VGA_HEIGHT, VGA_WIDTH};
use crate::{print, println, vec};
use alloc::vec::Vec;
use core::hint::spin_loop;
use core::ops::Index;

// Delay for spin, will differ on machines
const D: usize = 100000;
const FOREGROUND: Color = Color::Green;
const BACKGROUND: Color = Color::LightGrey;

// Golden formula
// (x, y, z)
// x' = x/z
// y' = y/z

#[derive(Clone, Copy)]
struct Pos2 {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct Pos3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Mesh {
    vertices: Vec<Pos3>,
    edges: Vec<(usize, usize)>,
}

static mut CANVAS: [u8; VGA_WIDTH * VGA_HEIGHT * 2] = [0; VGA_WIDTH * VGA_HEIGHT * 2];

fn screen(p: Pos2) -> Pos2 {
    return Pos2 {
        x: (p.x + 1.0) / 2.0 * (VGA_WIDTH as f64),
        y: (1.0 - (p.y + 1.0) / 2.0) * ((VGA_HEIGHT) as f64 * 2.0),
    };
}

fn project(p: Pos3) -> Pos2 {
    if p.z == 0.0 {
        return Pos2 { x: -99.0, y: -99.0 };
    }
    return Pos2 {
        x: p.x / p.z,
        y: p.y / p.z,
    };
}

fn lerp(a: Pos2, b: Pos2) {
    // a + (b - a) * t
    let dx = (a.x as isize - b.x as isize).abs();
    let dy = (a.y as isize - b.y as isize).abs();
    let t = dx.max(dy);
    if t == 0 {
        return;
    };
    for i in 0..=t {
        let f = i as f64 / t as f64;
        let x = a.x + (b.x - a.x) * f;
        let y = a.y + (b.y - a.y) * f;
        if x < 0.0 || x as usize >= VGA_WIDTH {
            continue;
        } else if y < 0.0 || y as usize >= VGA_HEIGHT * 2 {
            continue;
        }
        let index = y as usize * VGA_WIDTH + x as usize;
        if index < VGA_WIDTH * VGA_HEIGHT * 2 {
            unsafe {
                CANVAS[index] = 1;
            }
        }
    }
}

// SAFETY:
// this is a single thread process and animate is the only function to call this, creating a safe access
fn render_mesh(m: &Mesh, offset: f64) {
    unsafe {
        CANVAS.fill(0);
    }
    for e in m.edges.iter().copied() {
        let a = screen(project(Pos3 {
            z: m.vertices[e.0].z + offset,
            ..m.vertices[e.0]
        }));
        let b = screen(project(Pos3 {
            z: m.vertices[e.1].z + offset,
            ..m.vertices[e.1]
        }));
        lerp(a, b);
    }
}

fn blits() {
    fn coord_to_index(x: usize, y: usize) -> usize {
        return y * VGA_WIDTH + x;
    }
    let mut term = TERMINAL.lock();
    unsafe {
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                let bg = if CANVAS[coord_to_index(x, y * 2 + 1)] == 1 {
                    FOREGROUND
                } else {
                    BACKGROUND
                };
                let fg = if CANVAS[coord_to_index(x, y * 2)] == 1 {
                    FOREGROUND
                } else {
                    BACKGROUND
                };

                term.put_entry_at('▄', make_color(fg, bg), x, y);
            }
        }
    }
}

fn animate(m: &Mesh) {
    let mut dx = 0.0;
    let difference = 0.01;
    loop {
        // SAFETY:
        // Clearing before render
        // single threadded process lowers this risk
        unsafe {
            CANVAS.fill(0);
        }
        render_mesh(m, dx);
        blits();
        for _ in 0..D {
            spin_loop();
        }
        dx += difference;
    }
}

pub fn cube() {
    // Setup cube
    let cube: Mesh = Mesh {
        vertices: vec![
            Pos3 {
                x: -0.5,
                y: 0.5,
                z: 0.0,
            },
            Pos3 {
                x: 0.5,
                y: 0.5,
                z: 0.0,
            },
            Pos3 {
                x: 0.5,
                y: -0.5,
                z: 0.0,
            },
            Pos3 {
                x: -0.5,
                y: -0.5,
                z: 0.0,
            },
            Pos3 {
                x: -0.5,
                y: 0.5,
                z: 1.0,
            },
            Pos3 {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
            Pos3 {
                x: 0.5,
                y: -0.5,
                z: 1.0,
            },
            Pos3 {
                x: -0.5,
                y: -0.5,
                z: 1.0,
            },
        ],
        edges: vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ],
    };
    animate(&cube);
}
