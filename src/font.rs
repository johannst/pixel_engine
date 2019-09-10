use crate::Pixel::*;
use crate::Sprite;

const SPRITE_A: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

pub fn get_sprite(letter: char) -> &'static Sprite {
    match letter {
        'A' => &SPRITE_A,
        _ => {
            unimplemented!();
        }
    }
}
