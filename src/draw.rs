use crate::font;
use crate::{Pixel, PixelBuffer, PixelScale, Sprite};

/// Same as [draw_pixel](crate::draw_pixel) except accept
/// [PixelScale](crate::PixelScale) additionally.
pub fn draw_pixel_scaled<T: PixelBuffer>(
    buf: &mut T,
    x: usize,
    y: usize,
    rgb: u32,
    scale: PixelScale,
) {
    assert!(x < buf.width());
    assert!(y < buf.height());

    let scale = usize::from(scale);
    let width = buf.width();
    for l in 0..scale {
        for c in 0..scale {
            buf.buffer()[(y + l) * width + x + c] = rgb;
        }
    }
}

/// Same as [draw_sprite](crate::draw_sprite) except accept
/// [PixelScale](crate::PixelScale) additionally.
pub fn draw_sprite_scaled<T: PixelBuffer>(
    buf: &mut T,
    x: usize,
    y: usize,
    rgb: u32,
    sprite: &Sprite,
    scale: PixelScale,
) {
    let scale_val = usize::from(scale);

    for (l, pixel_line) in sprite.lines().enumerate() {
        for (c, &pixel) in pixel_line.iter().enumerate() {
            if pixel == Pixel::C {
                draw_pixel_scaled(buf, x + (c * scale_val), y + (l * scale_val), rgb, scale);
            }
        }
    }
}

/// Same as [draw_str](crate::draw_str) except accept
/// [PixelScale](crate::PixelScale) additionally.
pub fn draw_str_scaled<T: PixelBuffer>(
    buf: &mut T,
    x: usize,
    y: usize,
    rgb: u32,
    string: &str,
    scale: PixelScale,
) {
    let scale_val = usize::from(scale);

    let mut x_off = 0;
    for c in string.chars() {
        draw_sprite_scaled(buf, x + x_off, y, rgb, font::get_sprite(c), scale);
        x_off += 8 * scale_val;
    }
}

// Convenience methods with X1 scaling.

/// Draw single pixel at `(x, y)` with color `rgb` into the `PixelBuffer T`.
///
/// # Panics
///
/// Drawing panics in case `x >= T.width()` or `y >= T.height()`.
pub fn draw_pixel<T: PixelBuffer>(buf: &mut T, x: usize, y: usize, rgb: u32) {
    draw_pixel_scaled(buf, x, y, rgb, PixelScale::X1);
}

/// Draw sprite at `(x, y)` with color `rgb` into the `PixelBuffer T`.
///
/// Sprite is drawn with top-left corner starting at `(x, y)`.
///
/// # Panics
///
/// Drawing panics in case `x >= T.width()` or `y >= T.height()`.
/// Additionally panics in case drawing the Sprite exceeds the border of `T`.
pub fn draw_sprite<T: PixelBuffer>(buf: &mut T, x: usize, y: usize, rgb: u32, sprite: &Sprite) {
    draw_sprite_scaled(buf, x, y, rgb, sprite, PixelScale::X1);
}

/// Draw string at `(x, y)` with color `rgb` into the `PixelBuffer T`.
///
/// # Panics
///
/// Drawing panics in case `x >= T.width()` or `y >= T.height()`.
/// Additionally panics in case drawing the string exceeds the border of `T`.
pub fn draw_str<T: PixelBuffer>(buf: &mut T, x: usize, y: usize, rgb: u32, string: &str) {
    draw_str_scaled(buf, x, y, rgb, string, PixelScale::X1);
}

/// Draw rectangle at `(x, y)` with dimensions `(w, h)` and color `rgb` into the `PixelBuffer T`.
///
/// # Panics
///
/// Drawing panics in case `x >= T.width()` or `y >= T.height()`.
/// Additionally panics in case drawing the rectangle exceeds the border of `T`.
pub fn draw_rect<T: PixelBuffer>(buf: &mut T, x: usize, y: usize, rgb: u32, w: usize, h: usize) {
    for l in 0..h {
        for c in 0..w {
            draw_pixel(buf, x + c, y + l, rgb);
        }
    }
}
