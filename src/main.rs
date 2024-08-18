extern crate minifb;
// extern crate rodio;

mod welcome_screen;
mod mod_2d;
mod mod_3d;

use minifb::{Key, Window, WindowOptions};
// use rodio::{Decoder, OutputStream, Sink};
// use std::fs::File;
// use std::io::BufReader;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
    // let (_stream, handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&handle).unwrap();

    // Carga de la canción de Taylor Swift
    // let file = File::open("Taylor_Swift.mp3").unwrap();
    // let source = Decoder::new(BufReader::new(file)).unwrap();
    // sink.append(source);
    // sink.play(); // Inicia la reproducción
    // sink.detach(); // Deja que la canción siga sonando mientras el programa corre

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
                // Espera a que se presione una tecla antes de cambiar el modo
                window.update();
                if window.is_key_down(Key::Key2) {
                    current_mode = 2; // Cambia a modo 3D
                } else if window.is_key_down(Key::Backspace) {
                    current_mode = 0; // Regresa a la pantalla de inicio
                }
            },
            2 => {
                mod_3d::run_3d_with_window(&mut window);
                // Espera a que se presione una tecla antes de cambiar el modo
                window.update();
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
