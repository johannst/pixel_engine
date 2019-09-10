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
        if self.line < 8 {
            self.line += 1;
            Some(&self.sprite.lines[self.line - 1])
        } else {
            None
        }
    }
}
