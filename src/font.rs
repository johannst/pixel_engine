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

const SPRITE_B: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_C: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_D: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_E: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_F: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

pub fn get_sprite(letter: char) -> &'static Sprite {
    match letter {
        'A' => &SPRITE_A,
        'B' => &SPRITE_B,
        'C' => &SPRITE_C,
        'D' => &SPRITE_D,
        'E' => &SPRITE_E,
        'F' => &SPRITE_F,
        _ => {
            unimplemented!();
        }
    }
}
