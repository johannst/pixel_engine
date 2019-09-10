extern crate minifb;

use minifb::{Key, Window, WindowOptions};

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

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for (l, line) in pixel_engine::font::get_sprite('A').lines().enumerate() {
        for (c, &pixel) in line.iter().enumerate() {
            if pixel == pixel_engine::Pixel::C {
                buffer[l * WIDTH + c] = 0x00ff_ffff;
            }
        }
    }

    window.update_with_buffer(&buffer).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}
