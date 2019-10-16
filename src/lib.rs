#[macro_use]
extern crate lazy_static;

pub mod font;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Pixel {
    C,
    U,
}

pub struct Sprite {
    lines: [[Pixel; 8]; 8],
}

impl Sprite {
    pub fn lines(&self) -> SpriteLines<'_> {
        SpriteLines {
            sprite: &self,
            line: 0,
        }
    }
}

pub struct SpriteLines<'a> {
    sprite: &'a Sprite,
    line: usize,
}

impl<'a> Iterator for &'a mut SpriteLines<'_> {
    type Item = &'a [Pixel; 8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.line < self.sprite.lines.len() {
            self.line += 1;
            Some(&self.sprite.lines[self.line - 1])
        } else {
            None
        }
    }
}

pub trait PixelBuffer {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn buffer(&mut self) -> &mut [u32];
}

pub struct PixelVec {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl PixelVec {
    pub fn new(width: usize, height: usize) -> PixelVec {
        PixelVec {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }
}

impl PixelBuffer for PixelVec {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn buffer(&mut self) -> &mut [u32] {
        &mut self.buffer
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PixelScale {
    X1,
    X2,
    X4,
    X8,
}

fn get_scale_value(scale: PixelScale) -> usize {
    match scale {
        PixelScale::X1 => 1,
        PixelScale::X2 => 2,
        PixelScale::X4 => 4,
        PixelScale::X8 => 8,
        _ => {
            unimplemented!();
        }
    }
}

pub fn draw_pixel_with_scale<T: PixelBuffer>(
    buf: &mut T,
    x: usize,
    y: usize,
    rgb: u32,
    scale: PixelScale,
) {
    assert!(x < buf.width());
    assert!(y < buf.height());

    let scale = get_scale_value(scale);

    let width = buf.width();
    for l in 0..scale {
        for c in 0..scale {
            buf.buffer()[(y + l) * width + x + c] = rgb;
        }
    }
}

pub fn draw_sprite_with_scale<T: PixelBuffer>(
    buf: &mut T,
    x: usize,
    y: usize,
    sprite: &Sprite,
    scale: PixelScale,
) {
    let scale_val = get_scale_value(scale);

    for (l, pixel_line) in sprite.lines().enumerate() {
        for (c, &pixel) in pixel_line.iter().enumerate() {
            if pixel == Pixel::C {
                draw_pixel_with_scale(
                    buf,
                    x + (c * scale_val),
                    y + (l * scale_val),
                    0x00ffffff,
                    scale,
                );
            }
        }
    }
}

pub fn draw_str_with_scale<T: PixelBuffer>(
    buf: &mut T,
    x: usize,
    y: usize,
    string: &str,
    scale: PixelScale,
) {
    let scale_val = get_scale_value(scale);

    let mut x_off = 0;
    for c in string.chars() {
        draw_sprite_with_scale(buf, x + x_off, y, font::get_sprite(c), scale);
        x_off += 8 * scale_val;
    }
}

// PixelScale::X1 convenience methods
pub fn draw_pixel<T: PixelBuffer>(buf: &mut T, x: usize, y: usize, rgb: u32) {
    draw_pixel_with_scale(buf, x, y, rgb, PixelScale::X1);
}

pub fn draw_sprite<T: PixelBuffer>(buf: &mut T, x: usize, y: usize, sprite: &Sprite) {
    draw_sprite_with_scale(buf, x, y, sprite, PixelScale::X1);
}

pub fn draw_str<T: PixelBuffer>(buf: &mut T, x: usize, y: usize, string: &str) {
    draw_str_with_scale(buf, x, y, string, PixelScale::X1);
}
