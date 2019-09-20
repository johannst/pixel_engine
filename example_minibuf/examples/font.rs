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
    for y in 0..100 {
        for x in 0..100 {
            pixel_engine::draw_pixel(&mut buf, x, y, 0x00ffffff);
        }
    }
    pixel_engine::draw_sprite(&mut buf, 200, 200, pixel_engine::font::get_sprite('D'));
    pixel_engine::draw_str(&mut buf, 100, 300, "DEADBEFF");

    window.update_with_buffer(buf.buffer()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}

