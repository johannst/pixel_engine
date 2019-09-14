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

    let mut render = |x, letter| {
        for (l, line) in pixel_engine::font::get_sprite(letter).lines().enumerate() {
            for (c, &pixel) in line.iter().enumerate() {
                if pixel == pixel_engine::Pixel::C {
                    buffer[l * WIDTH + c + x] = 0x00ff_ffff;
                }
            }
        }
    };
    render(0, 'D');
    render(8, 'E');
    render(16, 'A');
    render(24, 'D');
    render(32, 'B');
    render(40, 'E');
    render(48, 'E');
    render(56, 'F');

    window.update_with_buffer(&buffer).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
    }
}
