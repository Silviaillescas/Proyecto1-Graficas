extern crate rodio;

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let file = File::open("Taylor_Swift.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.play();
    sink.sleep_until_end();
}
