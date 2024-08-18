extern crate minifb;

use minifb::{Window, Key};
use std::f32::consts::PI;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

pub const MAZE_WIDTH: usize = 20;
pub const MAZE_HEIGHT: usize = 20;

const PLAYER_SPEED: f32 = 0.05;
const ROTATION_SPEED: f32 = 0.03;

const MINIMAP_SCALE: usize = 5;

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
    angle: f32,
}

impl Player {
    fn new(x: f32, y: f32, angle: f32) -> Self {
        Player { x, y, angle }
    }

    fn move_forward(&mut self, dist: f32) {
        let new_x = self.x + dist * self.angle.cos();
        let new_y = self.y + dist * self.angle.sin();
        if !self.is_colliding(new_x, self.y) {
            self.x = new_x;
        }
        if !self.is_colliding(self.x, new_y) {
            self.y = new_y;
        }
    }

    fn move_backward(&mut self, dist: f32) {
        let new_x = self.x - dist * self.angle.cos();
        let new_y = self.y - dist * self.angle.sin();
        if !self.is_colliding(new_x, self.y) {
            self.x = new_x;
        }
        if !self.is_colliding(self.x, new_y) {
            self.y = new_y;
        }
    }

    fn rotate(&mut self, angle: f32) {
        self.angle += angle;
    }

    fn is_colliding(&self, x: f32, y: f32) -> bool {
        let maze_x = x as usize;
        let maze_y = y as usize;

        if maze_x >= MAZE_WIDTH || maze_y >= MAZE_HEIGHT {
            return true;
        }

        MAZE[maze_y][maze_x] == 1
    }
}

pub fn run_3d_with_window(window: &mut Window) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut player = Player::new(1.5, 1.5, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(&mut player, window);

        draw_3d_view(&mut buffer, &player);
        draw_minimap(&mut buffer, &player); // Añadimos el minimapa
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn handle_input(player: &mut Player, window: &Window) {
    if window.is_key_down(Key::W) {
        player.move_forward(PLAYER_SPEED);
    }

    if window.is_key_down(Key::S) {
        player.move_backward(PLAYER_SPEED);
    }

    if window.is_key_down(Key::A) {
        player.rotate(-ROTATION_SPEED);
    }

    if window.is_key_down(Key::D) {
        player.rotate(ROTATION_SPEED);
    }
}

fn draw_3d_view(buffer: &mut [u32], player: &Player) {
    let wall_height = HEIGHT as f32 / 2.0;

    for x in 0..WIDTH {
        let ray_angle = player.angle - (PI / 6.0) + (x as f32 / WIDTH as f32) * (PI / 3.0);

        let (mut distance_to_wall, wall_x, wall_y) = cast_ray(player, ray_angle);

        // Corrección de distorsión de ojo de pez
        distance_to_wall *= (player.angle - ray_angle).cos();

        let line_height = (wall_height / distance_to_wall) as isize;

        // Asegúrate de que draw_start y draw_end estén dentro de los límites
        let draw_start = (HEIGHT as isize / 2) - (line_height / 2).max(0);
        let draw_end = (HEIGHT as isize / 2) + (line_height / 2).min(HEIGHT as isize - 1);

        let mut wall_color = create_mosaic_texture(wall_x, wall_y);

        // Aplicar efecto de sombreado
        let shade_factor = 1.0 / distance_to_wall;
        wall_color = apply_shading(wall_color, shade_factor);

        for y in 0..HEIGHT {
            let y_isize = y as isize;
            if y_isize < draw_start {
                buffer[y * WIDTH + x] = 0xB0E0E6; // Color del cielo más suave
            } else if y_isize >= draw_start && y_isize <= draw_end {
                buffer[y * WIDTH + x] = wall_color; // Color de la pared
            } else {
                buffer[y * WIDTH + x] = 0x8B4513; // Color del suelo (Marrón)
            }
        }
    }
}

fn draw_minimap(buffer: &mut [u32], player: &Player) {
    for y in 0..MAZE_HEIGHT {
        for x in 0..MAZE_WIDTH {
            let color = if MAZE[y][x] == 1 { 0x000000 } else { 0xFFFFFF };
            fill_rect(buffer, x * MINIMAP_SCALE, y * MINIMAP_SCALE, MINIMAP_SCALE, MINIMAP_SCALE, color);
        }
    }

    // Dibujar al jugador en el minimapa
    let player_x = (player.x * MINIMAP_SCALE as f32) as usize;
    let player_y = (player.y * MINIMAP_SCALE as f32) as usize;
    fill_rect(buffer, player_x, player_y, MINIMAP_SCALE, MINIMAP_SCALE, 0xFF0000); // Rojo para el jugador
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

fn cast_ray(player: &Player, angle: f32) -> (f32, usize, usize) {
    let mut distance_to_wall = 0.0;
    let mut hit_wall = false;

    let eye_x = angle.cos();
    let eye_y = angle.sin();

    let mut wall_x = 0;
    let mut wall_y = 0;

    while !hit_wall && distance_to_wall < 30.0 {
        distance_to_wall += 0.1;

        wall_x = (player.x + eye_x * distance_to_wall) as usize;
        wall_y = (player.y + eye_y * distance_to_wall) as usize;

        // Verificar que no se exceda el índice
        if wall_x >= MAZE_WIDTH || wall_y >= MAZE_HEIGHT {
            break;
        }

        if MAZE[wall_y][wall_x] == 1 {
            hit_wall = true;
        }
    }

    (distance_to_wall, wall_x, wall_y)
}

fn create_mosaic_texture(x: usize, y: usize) -> u32 {
    let pattern = (x % 2 == y % 2) as u32;
    if pattern == 1 {
        0x000000 // Negro
    } else {
        0x5A5A5A // Gris oscuro
    }
}

fn apply_shading(color: u32, shade_factor: f32) -> u32 {
    let r = ((color >> 16) & 0xFF) as f32;
    let g = ((color >> 8) & 0xFF) as f32;
    let b = (color & 0xFF) as f32;

    let r = (r * shade_factor).min(255.0) as u32;
    let g = (g * shade_factor).min(255.0) as u32;
    let b = (b * shade_factor).min(255.0) as u32;

    (r << 16) | (g << 8) | b
}
