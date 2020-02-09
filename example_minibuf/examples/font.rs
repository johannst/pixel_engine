extern crate minifb;
use minifb::{Key, Window, WindowOptions};

use pixel_engine::PixelBuffer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Example - ESC to quit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buf = pixel_engine::PixelVec::new(WIDTH, HEIGHT);
    pixel_engine::draw_str_scaled(
        &mut buf,
        10,
        10,
        0x00ffffff,
        "PRESS [ESC] TO QUIT",
        pixel_engine::PixelScale::X2,
    );
    pixel_engine::draw_str(&mut buf, 10, 50, 0x00ffffff, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    pixel_engine::draw_str(&mut buf, 10, 60, 0x00ff0000, "RED");
    pixel_engine::draw_str(&mut buf, 10, 70, 0x0000ff00, "GREEN");
    pixel_engine::draw_str(&mut buf, 10, 80, 0x000000ff, "BLUE");

    window.update_with_buffer(buf.buffer()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}
