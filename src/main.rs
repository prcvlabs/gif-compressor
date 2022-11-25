use std::fs::File;
use gif;

fn main() {
    println!("Reading GIF..");

    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    let file = File::open("lol-yolo.gif").unwrap();

    let mut decoder = decoder.read_info(file).unwrap();

    let mut i = 0;
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        i += 1;
    }

    println!("Found {i} frames in gif");
}
