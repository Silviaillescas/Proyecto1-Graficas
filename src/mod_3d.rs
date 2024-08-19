extern crate minifb;

use minifb::{Window, Key};
use std::f32::consts::PI;
use std::time::{Instant, Duration};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

pub const MAZE_WIDTH: usize = 20;
pub const MAZE_HEIGHT: usize = 20;

const PLAYER_SPEED: f32 = 0.05;
const ROTATION_SPEED: f32 = 0.03;

const MINIMAP_SCALE: usize = 5;
const SCALE: usize = 30;

const EXIT_X: usize = 18;  // Posición de la salida
const EXIT_Y: usize = 18;

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

pub fn run_3d_with_window(window: &mut Window) -> bool {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut player = Player::new(1.5, 1.5, 0.0);

    let mut last_update = Instant::now();
    let mut frame_count = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(&mut player, window);

        if player_at_exit(&player) {
            return true;  // Aquí podrías cambiar de modo o mostrar la pantalla de éxito
        }

        draw_3d_view(&mut buffer, &player);
        draw_minimap(&mut buffer, &player);

        // Calcular y mostrar FPS
        frame_count += 1;
        if last_update.elapsed() >= Duration::from_secs(1) {
            let fps = frame_count;
            frame_count = 0;
            last_update = Instant::now();
            draw_fps(&mut buffer, fps);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    false
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

fn player_at_exit(player: &Player) -> bool {
    let player_x = player.x as usize;
    let player_y = player.y as usize;
    player_x == EXIT_X && player_y == EXIT_Y
}

pub fn draw_success_screen(window: &mut Window) {
    let mut buffer: Vec<u32> = vec![0x00FF00; WIDTH * HEIGHT]; // Pantalla verde de éxito

    draw_text(&mut buffer, "FIN", WIDTH / 2 - 16, HEIGHT / 2 - 4); // Mostrar "FIN"

    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

    // Esperar a que se presione una tecla para regresar a la pantalla de bienvenida
    while window.is_open() {
        if window.is_key_down(Key::Enter) || window.is_key_down(Key::Space) || window.is_key_down(Key::Backspace) {
            break;
        }
    }
}

fn draw_fps(buffer: &mut [u32], fps: usize) {
    let text = format!("FPS: {}", fps);
    draw_text(buffer, &text, 10, 10); // Dibuja el texto FPS en la parte superior izquierda
}

fn draw_text(buffer: &mut [u32], text: &str, x: usize, y: usize) {
    let font = get_font();
    let mut offset_x = x;
    
    for ch in text.chars() {
        if let Some(char_pattern) = font.get(&ch) {
            draw_char(buffer, char_pattern, offset_x, y);
            offset_x += 8; // Asumiendo un ancho fijo de 8 píxeles por carácter
        }
    }
}

fn draw_char(buffer: &mut [u32], char_pattern: &[u8], x: usize, y: usize) {
    for (row, &row_pattern) in char_pattern.iter().enumerate() {
        for col in 0..8 {
            if row_pattern & (1 << (7 - col)) != 0 {
                let pixel_x = x + col;
                let pixel_y = y + row;
                if pixel_x < WIDTH && pixel_y < HEIGHT {
                    buffer[pixel_y * WIDTH + pixel_x] = 0xFFFFFF; // Blanco para las letras
                }
            }
        }
    }
}

fn get_font() -> std::collections::HashMap<char, [u8; 8]> {
    let mut font = std::collections::HashMap::new();

    font.insert('F', [
        0b11111111,
        0b10000000,
        0b10000000,
        0b11111100,
        0b10000000,
        0b10000000,
        0b10000000,
        0b10000000,
    ]);

    font.insert('I', [
        0b00111100,
        0b00011000,
        0b00011000,
        0b00011000,
        0b00011000,
        0b00011000,
        0b00011000,
        0b00111100,
    ]);

    font.insert('N', [
        0b10000010,
        0b11000010,
        0b10100010,
        0b10010010,
        0b10001010,
        0b10000110,
        0b10000010,
        0b10000010,
    ]);

    // Puedes agregar más caracteres aquí.

    font
}

fn draw_3d_view(buffer: &mut [u32], player: &Player) {
    let wall_height = HEIGHT as f32 / 2.0;

    for x in 0..WIDTH {
        let ray_angle = player.angle - (PI / 6.0) + (x as f32 / WIDTH as f32) * (PI / 3.0);

        let (mut distance_to_wall, wall_x, wall_y) = cast_ray(player, ray_angle);

        // Corrección de distorsión de ojo de pez
        distance_to_wall *= (player.angle - ray_angle).cos();

        let line_height = (wall_height / distance_to_wall) as isize;

        
        let draw_start = (HEIGHT as isize / 2) - (line_height / 2).max(0);
        let draw_end = (HEIGHT as isize / 2) + (line_height / 2).min(HEIGHT as isize - 1);

        let mut wall_color = create_mosaic_texture(wall_x, wall_y);

      
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

    // Dibujar la salida en el minimapa
    fill_rect(buffer, EXIT_X * MINIMAP_SCALE, EXIT_Y * MINIMAP_SCALE, MINIMAP_SCALE, MINIMAP_SCALE, 0x00FF00); // Verde para la salida
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
