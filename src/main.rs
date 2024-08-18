extern crate minifb;

mod welcome_screen;
mod mod_2d;
mod mod_3d;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

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
    let mut last_update = Instant::now();
    let mut frame_count = 0;
    let mut fps = 0.0;

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
                    current_mode = 2;
                } else if window.is_key_down(Key::Backspace) {
                    current_mode = 0;
                }
            },
            2 => {
                mod_3d::run_3d_with_window(&mut window);
                if window.is_key_down(Key::Key1) {
                    current_mode = 1;
                } else if window.is_key_down(Key::Backspace) {
                    current_mode = 0;
                }
            },
            _ => current_mode = 0,
        }

        frame_count += 1;

        if last_update.elapsed() >= Duration::from_millis(500) {  // Reduce a 500ms
            fps = frame_count as f64 / last_update.elapsed().as_secs_f64();
            frame_count = 0;
            last_update = Instant::now();
        }        

        window.set_title(&format!("Laberinto 2D/3D - FPS: {:.2}", fps));

        window.update();
    }
}
