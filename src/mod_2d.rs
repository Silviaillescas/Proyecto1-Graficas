extern crate minifb;

use minifb::{Key, Window};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

pub const MAZE_WIDTH: usize = 20;
pub const MAZE_HEIGHT: usize = 20;

const SCALE: usize = 30;
const PLAYER_SPEED: f32 = 0.1;

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

struct Player {
    x: f32,
    y: f32,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player { x, y }
    }

    fn move_if_no_collision(&mut self, new_x: f32, new_y: f32) {
        if !self.is_colliding(new_x, self.y) {
            self.x = new_x;
        }
        if !self.is_colliding(self.x, new_y) {
            self.y = new_y;
        }
    }

    fn is_colliding(&self, x: f32, y: f32) -> bool {
        let maze_x = x.floor() as usize;
        let maze_y = y.floor() as usize;

        if maze_x >= MAZE_WIDTH || maze_y >= MAZE_HEIGHT {
            return true;
        }

        MAZE[maze_y][maze_x] == 1
    }
}

pub fn run_2d_with_window(window: &mut Window) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut player = Player::new(1.5, 1.5);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(&mut player, window);

        draw_maze(&mut buffer, &player);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn handle_input(player: &mut Player, window: &Window) {
    let mut new_x = player.x;
    let mut new_y = player.y;

    if window.is_key_down(Key::W) {
        new_y -= PLAYER_SPEED;
    }

    if window.is_key_down(Key::S) {
        new_y += PLAYER_SPEED;
    }

    if window.is_key_down(Key::A) {
        new_x -= PLAYER_SPEED;
    }

    if window.is_key_down(Key::D) {
        new_x += PLAYER_SPEED;
    }

    player.move_if_no_collision(new_x, new_y);
}

fn draw_maze(buffer: &mut [u32], player: &Player) {
    for y in 0..MAZE_HEIGHT {
        for x in 0..MAZE_WIDTH {
            let color = if MAZE[y][x] == 1 {
                create_mosaic_texture(x, y)
            } else {
                0xFFFFFF // Blanco para el fondo
            };
            fill_rect(buffer, x * SCALE, y * SCALE, SCALE, SCALE, color);
        }
    }

    // Dibujar al jugador en el laberinto
    let player_x = (player.x * SCALE as f32) as usize;
    let player_y = (player.y * SCALE as f32) as usize;
    fill_rect(buffer, player_x, player_y, SCALE, SCALE, 0xFF0000); // Rojo para el jugador
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
        0x000000 // Negro
    } else {
        0x5A5A5A // Gris oscuro
    }
}
