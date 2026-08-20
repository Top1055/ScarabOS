use crate::vec;
use crate::vga_buffer::{make_color, Color, TERMINAL, VGA_HEIGHT, VGA_WIDTH};
use alloc::vec::Vec;
use core::hint::spin_loop;

// Delay for spin, will differ on machines
const D: usize = 200000;
const FOREGROUND: Color = Color::Green;
const BACKGROUND: Color = Color::LightGrey;

pub const SIN_STEPS: usize = 256;
pub const SIN_TABLE: [f64; 256] = [
    0.00000000,
    0.02454123,
    0.04906767,
    0.07356456,
    0.09801714,
    0.12241068,
    0.14673047,
    0.17096189,
    0.19509032,
    0.21910124,
    0.24298018,
    0.26671276,
    0.29028468,
    0.31368174,
    0.33688985,
    0.35989504,
    0.38268343,
    0.40524131,
    0.42755509,
    0.44961133,
    0.47139674,
    0.49289819,
    0.51410274,
    0.53499762,
    0.55557023,
    0.57580819,
    0.59569930,
    0.61523159,
    0.63439328,
    0.65317284,
    0.67155895,
    0.68954054,
    0.70710678,
    0.72424708,
    0.74095113,
    0.75720885,
    0.77301045,
    0.78834643,
    0.80320753,
    0.81758481,
    0.83146961,
    0.84485357,
    0.85772861,
    0.87008699,
    0.88192126,
    0.89322430,
    0.90398929,
    0.91420976,
    0.92387953,
    0.93299280,
    0.94154407,
    0.94952818,
    0.95694034,
    0.96377607,
    0.97003125,
    0.97570213,
    0.98078528,
    0.98527764,
    0.98917651,
    0.99247953,
    0.99518473,
    0.99729046,
    0.99879546,
    0.99969882,
    1.00000000,
    0.99969882,
    0.99879546,
    0.99729046,
    0.99518473,
    0.99247953,
    0.98917651,
    0.98527764,
    0.98078528,
    0.97570213,
    0.97003125,
    0.96377607,
    0.95694034,
    0.94952818,
    0.94154407,
    0.93299280,
    0.92387953,
    0.91420976,
    0.90398929,
    0.89322430,
    0.88192126,
    0.87008699,
    0.85772861,
    0.84485357,
    0.83146961,
    0.81758481,
    0.80320753,
    0.78834643,
    0.77301045,
    0.75720885,
    0.74095113,
    0.72424708,
    0.70710678,
    0.68954054,
    0.67155895,
    0.65317284,
    0.63439328,
    0.61523159,
    0.59569930,
    0.57580819,
    0.55557023,
    0.53499762,
    0.51410274,
    0.49289819,
    0.47139674,
    0.44961133,
    0.42755509,
    0.40524131,
    0.38268343,
    0.35989504,
    0.33688985,
    0.31368174,
    0.29028468,
    0.26671276,
    0.24298018,
    0.21910124,
    0.19509032,
    0.17096189,
    0.14673047,
    0.12241068,
    0.09801714,
    0.07356456,
    0.04906767,
    0.02454123,
    0.00000000,
    -0.02454123,
    -0.04906767,
    -0.07356456,
    -0.09801714,
    -0.12241068,
    -0.14673047,
    -0.17096189,
    -0.19509032,
    -0.21910124,
    -0.24298018,
    -0.26671276,
    -0.29028468,
    -0.31368174,
    -0.33688985,
    -0.35989504,
    -0.38268343,
    -0.40524131,
    -0.42755509,
    -0.44961133,
    -0.47139674,
    -0.49289819,
    -0.51410274,
    -0.53499762,
    -0.55557023,
    -0.57580819,
    -0.59569930,
    -0.61523159,
    -0.63439328,
    -0.65317284,
    -0.67155895,
    -0.68954054,
    -0.70710678,
    -0.72424708,
    -0.74095113,
    -0.75720885,
    -0.77301045,
    -0.78834643,
    -0.80320753,
    -0.81758481,
    -0.83146961,
    -0.84485357,
    -0.85772861,
    -0.87008699,
    -0.88192126,
    -0.89322430,
    -0.90398929,
    -0.91420976,
    -0.92387953,
    -0.93299280,
    -0.94154407,
    -0.94952818,
    -0.95694034,
    -0.96377607,
    -0.97003125,
    -0.97570213,
    -0.98078528,
    -0.98527764,
    -0.98917651,
    -0.99247953,
    -0.99518473,
    -0.99729046,
    -0.99879546,
    -0.99969882,
    -1.00000000,
    -0.99969882,
    -0.99879546,
    -0.99729046,
    -0.99518473,
    -0.99247953,
    -0.98917651,
    -0.98527764,
    -0.98078528,
    -0.97570213,
    -0.97003125,
    -0.96377607,
    -0.95694034,
    -0.94952818,
    -0.94154407,
    -0.93299280,
    -0.92387953,
    -0.91420976,
    -0.90398929,
    -0.89322430,
    -0.88192126,
    -0.87008699,
    -0.85772861,
    -0.84485357,
    -0.83146961,
    -0.81758481,
    -0.80320753,
    -0.78834643,
    -0.77301045,
    -0.75720885,
    -0.74095113,
    -0.72424708,
    -0.70710678,
    -0.68954054,
    -0.67155895,
    -0.65317284,
    -0.63439328,
    -0.61523159,
    -0.59569930,
    -0.57580819,
    -0.55557023,
    -0.53499762,
    -0.51410274,
    -0.49289819,
    -0.47139674,
    -0.44961133,
    -0.42755509,
    -0.40524131,
    -0.38268343,
    -0.35989504,
    -0.33688985,
    -0.31368174,
    -0.29028468,
    -0.26671276,
    -0.24298018,
    -0.21910124,
    -0.19509032,
    -0.17096189,
    -0.14673047,
    -0.12241068,
    -0.09801714,
    -0.07356456,
    -0.04906767,
    -0.02454123,
];

fn rotate_xz(p: Pos3, r: usize) -> Pos3 {
    let c = SIN_TABLE[(r + (SIN_STEPS / 4)) % SIN_STEPS];
    let s = SIN_TABLE[r];
    return Pos3 {
        x: p.x * c - p.z * s,
        y: p.y,
        z: p.x * s + p.z * c,
    };
}

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
fn render_mesh(m: &Mesh, offset: f64, angle: usize) {
    unsafe {
        CANVAS.fill(0);
    }
    for e in m.edges.iter().copied() {
        // Rotate first around origin
        let a_rot = rotate_xz(m.vertices[e.0], angle);
        let a = screen(project(Pos3 {
            z: a_rot.z + offset,
            ..a_rot
        }));
        let b_rot = rotate_xz(m.vertices[e.1], angle);
        let b = screen(project(Pos3 {
            z: b_rot.z + offset,
            ..b_rot
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
    let mut angle = 0;
    let mut dx = 1.5;
    let difference = 0.0;
    loop {
        // SAFETY:
        // Clearing before render
        // single threadded process lowers this risk
        unsafe {
            CANVAS.fill(0);
        }
        render_mesh(m, dx, angle);
        blits();
        for _ in 0..D {
            spin_loop();
        }
        dx += difference;
        angle = (angle + 1) % SIN_STEPS;
    }
}

pub fn cube() {
    // Setup cube
    let cube: Mesh = Mesh {
        vertices: vec![
            Pos3 {
                x: -0.5,
                y: 0.5,
                z: -0.5,
            },
            Pos3 {
                x: 0.5,
                y: 0.5,
                z: -0.5,
            },
            Pos3 {
                x: 0.5,
                y: -0.5,
                z: -0.5,
            },
            Pos3 {
                x: -0.5,
                y: -0.5,
                z: -0.5,
            },
            Pos3 {
                x: -0.5,
                y: 0.5,
                z: 0.5,
            },
            Pos3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            Pos3 {
                x: 0.5,
                y: -0.5,
                z: 0.5,
            },
            Pos3 {
                x: -0.5,
                y: -0.5,
                z: 0.5,
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
