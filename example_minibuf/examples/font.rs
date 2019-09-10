extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    pixel_engine::say_hello();

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
    for l in 0..HEIGHT {
        for c in 0..WIDTH {
            if l < HEIGHT / 2 {
                buffer[l * WIDTH + c] = 0x00ff_0000;
            } else {
                buffer[l * WIDTH + c] = 0x0000_ff00;
            }
        }
    }

    window.update_with_buffer(&buffer).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}
