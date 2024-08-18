extern crate minifb;

use minifb::{Window, Key};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

pub const MAZE_WIDTH: usize = 20;
pub const MAZE_HEIGHT: usize = 20;

const SCALE: usize = 30;

pub const MAZE: [[i32; MAZE_WIDTH]; MAZE_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1],
    [1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
    [1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub fn run_2d_with_window(window: &mut Window) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    draw_maze(&mut buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_maze(buffer: &mut [u32]) {
    for y in 0..MAZE_HEIGHT {
        for x in 0..MAZE_WIDTH {
            let color = if MAZE[y][x] == 1 {
                create_mosaic_texture(x, y)
            } else {
                0xADD8E6 // Azul claro para el fondo
            };
            fill_rect(buffer, x * SCALE, y * SCALE, SCALE, SCALE, color);
        }
    }
}

fn fill_rect(buffer: &mut [u32], x: usize, y: usize, width: usize, height: usize, color: u32) {
    for py in 0..height {
        for px in 0..width {
            let buffer_x = x + px;
            let buffer_y = y + py;
            if buffer_x < WIDTH && buffer_y < HEIGHT {
                buffer[buffer_y * WIDTH + buffer_x] = color;
            }
        }
    }
}

fn create_mosaic_texture(x: usize, y: usize) -> u32 {
    let pattern = (x % 2 == y % 2) as u32;
    if pattern == 1 {
        0x000000
    } else {
        0x5A5A5A
    }
}
