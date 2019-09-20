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
    width : usize,
    height : usize,
    buffer : Vec<u32>,
}

impl PixelVec {
    pub fn new(width : usize, height: usize) -> PixelVec {
        PixelVec {
            width,
            height,
            buffer : vec![0; width * height],
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

pub fn draw_pixel<T: PixelBuffer>(buf : &mut T, x : usize, y: usize, rgb : u32) {
    assert!(x < buf.width());
    assert!(y < buf.height());

    let width = buf.width();
    buf.buffer()[ y * width + x] = rgb;
}

pub fn draw_sprite<T: PixelBuffer>(buf : &mut T, x : usize, y: usize, sprite : &Sprite) {
    for (l, pixel_line) in sprite.lines().enumerate() {
        for (c, &pixel) in pixel_line.iter().enumerate() {
            if pixel == Pixel::C {
                draw_pixel(buf, x + c, y + l, 0x00ffffff);
            }
        }
    }
}

pub fn draw_str<T: PixelBuffer>(buf : &mut T, x : usize, y: usize, string: &str) {
    let mut x_off = 0;
    for c in string.chars() {
        draw_sprite(buf, x + x_off, y, font::get_sprite(c));
        x_off += 8;
    }
}

