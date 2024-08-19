extern crate minifb;

mod welcome_screen;
mod mod_2d;
mod mod_3d;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
    let mut window = Window::new(
        "Laberinto 2D/3D",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut current_mode = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        match current_mode {
            0 => {
                welcome_screen::draw_welcome_screen(&mut window);

                if window.is_key_down(Key::Key1) {
                    current_mode = 1;
                } else if window.is_key_down(Key::Key2) {
                    current_mode = 2;
                }
            },
            1 => {
                mod_2d::run_2d_with_window(&mut window);
                if window.is_key_down(Key::Key2) {
                    current_mode = 2; // Cambia a modo 3D
                } else if window.is_key_down(Key::Backspace) {
                    current_mode = 0; // Regresa a la pantalla de inicio
                }
            },
            2 => {
                mod_3d::run_3d_with_window(&mut window);
                if window.is_key_down(Key::Key1) {
                    current_mode = 1; // Cambia a modo 2D
                } else if window.is_key_down(Key::Backspace) {
                    current_mode = 0; // Regresa a la pantalla de inicio
                }
            },
            _ => current_mode = 0,
        }

        window.update();
    }
}
