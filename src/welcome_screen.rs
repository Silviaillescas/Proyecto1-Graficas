// extern crate rodio;
// use rodio::{OutputStream, Sink, Decoder};
// use std::fs::File;
// use std::io::BufReader;
use minifb::Window;


const WIDTH: usize = 600;
const HEIGHT: usize = 600;

pub fn draw_welcome_screen(window: &mut Window) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Reproduce la música de fondo
    // play_music("ruta/a/taylor_swift_song.mp3");

    // Fondo con patrón sutil
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if (x + y) % 2 == 0 { 0x2E8B57 } else { 0x3CB371 };
            buffer[y * WIDTH + x] = color;
        }
    }

    // Títulos
    let title = "RAY CASTER";
    let subtitle1 = "1: 2D";
    let subtitle2 = "2: 3D";

    draw_centered_text(&mut buffer, WIDTH / 2, HEIGHT / 4, title, 0xFFFFFF);
    draw_centered_text(&mut buffer, WIDTH / 2, HEIGHT / 2 - 20, subtitle1, 0xFFFFFF);
    draw_centered_text(&mut buffer, WIDTH / 2, HEIGHT / 2, subtitle2, 0xFFFFFF);
   
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
}

fn draw_centered_text(buffer: &mut [u32], center_x: usize, y: usize, text: &str, color: u32) {
    let text_width = text.len() * 18;
    let start_x = center_x.saturating_sub(text_width / 2);

    for (i, c) in text.chars().enumerate() {
        draw_char(buffer, start_x + i * 18, y, c, color);
    }
}

fn draw_char(buffer: &mut [u32], x: usize, y: usize, c: char, color: u32) {
    let font = get_font(c);
    for (py, row) in font.iter().enumerate() {
        for (px, &pixel) in row.iter().enumerate() {
            if pixel == 1 {
                let index = (y + py * 3) * WIDTH + (x + px * 3);
                if index < WIDTH * HEIGHT {
                    buffer[index] = color;
                    buffer[index + 1] = color;
                    buffer[index + WIDTH] = color;
                    buffer[index + WIDTH + 1] = color;
                }
            }
        }
    }
}

fn get_font(c: char) -> [[u8; 5]; 5] {
    match c {
        'R' => [
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 1, 0, 0],
            [1, 0, 0, 1, 0],
        ],
        'A' => [
            [0, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
        ],
        'Y' => [
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
        ],
        'C' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ],
        'S' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
        ],
        'T' => [
            [1, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
        ],
        'E' => [
            [1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 0],
        ],
        'P' => [
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ],
        '2' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [1, 1, 1, 1, 0],
        ],
        'D' => [
            [1, 1, 1, 0, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
        ],
        '1' => [
            [0, 1, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 1, 1, 0, 0],
        ],
        '3' => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 0, 1, 1, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ],
        _ => [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    }
}

// fn play_music(file_path: &str) {
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     let sink = Sink::try_new(&stream_handle).unwrap();

//     let file = File::open(file_path).unwrap();
//     let source = Decoder::new(BufReader::new(file)).unwrap();
//     sink.append(source);
//     sink.detach();
// }
