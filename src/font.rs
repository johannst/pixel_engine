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

const SPRITE_G: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, C, C, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_H: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_I: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_J: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, C, U, U, C, U, U, U],
        [U, C, C, C, C, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_K: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, C, C, C, U, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_L: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_M: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, C, U, C, C, U, U],
        [U, C, U, C, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_N: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, U, U, U, C, U],
        [U, C, U, C, U, U, C, U],
        [U, C, U, U, C, U, C, U],
        [U, C, U, U, U, C, C, U],
        [U, C, U, U, U, U, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_O: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_P: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, C, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_Q: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, C, U, C, U],
        [U, C, U, U, U, C, C, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_R: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, U, U],
        [U, C, U, U, C, U, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, U, U, U, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_S: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, C, U],
        [U, U, U, U, U, U, C, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_T: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_U: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_V: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, U, C, U, U, C, U, U],
        [U, U, C, U, U, C, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_W: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, U, C, U, C, U, U],
        [U, C, C, U, C, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_X: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, U, C, U],
        [U, U, C, U, U, C, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, C, U, U, C, U, U],
        [U, C, U, U, U, U, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_Y: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, U, U, U, C, U, U],
        [U, U, C, U, C, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_Z: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, C, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, C, U, U, U, U, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_SPACE: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_COMMA: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_0: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, C, C, C, C, U, U],
        [U, C, U, U, U, C, C, U],
        [U, C, U, U, C, U, C, U],
        [U, C, U, C, U, U, C, U],
        [U, C, C, U, U, U, C, U],
        [U, U, C, C, C, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_1: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, C, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, C, C, C, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_2: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, C, U, U, C, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, C, U, U, U, U, U],
        [U, U, C, C, C, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_3: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, C, U, U, C, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, U, C, U, U],
        [U, U, C, U, U, C, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_4: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, U, C, C, U, U],
        [U, U, U, C, U, C, U, U],
        [U, U, C, U, U, C, U, U],
        [U, C, U, U, U, C, U, U],
        [U, C, C, C, C, C, C, U],
        [U, U, U, U, U, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_5: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, C, C, C, C, U, U],
        [U, U, C, U, U, U, U, U],
        [U, U, C, C, C, U, U, U],
        [U, U, U, U, U, C, U, U],
        [U, U, U, U, U, C, U, U],
        [U, U, C, C, C, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_6: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, C, C, C, U, U],
        [U, U, C, U, U, U, U, U],
        [U, U, C, C, C, U, U, U],
        [U, U, C, U, U, C, U, U],
        [U, U, C, U, U, C, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_7: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, C, C, C, C, U, U],
        [U, U, U, U, U, C, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, U, C, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, C, U, U, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_8: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, C, C, C, C, U, U],
        [U, C, U, U, U, U, C, U],
        [U, C, C, C, C, C, C, U],
        [U, C, U, U, U, U, C, U],
        [U, C, U, U, U, U, C, U],
        [U, U, C, C, C, C, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_9: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, C, U, U, C, U, U],
        [U, U, C, U, U, C, U, U],
        [U, U, U, C, C, C, U, U],
        [U, U, U, U, U, C, U, U],
        [U, U, C, C, C, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_LSBRACKET: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, C, U, U, U],
        [U, U, U, U, U, U, U, U],
    ],
};

const SPRITE_RSBRACKET: Sprite = Sprite {
    lines: [
        [U, U, U, U, U, U, U, U],
        [U, U, C, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, U, C, U, U, U, U],
        [U, U, C, C, U, U, U, U],
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
        'G' => &SPRITE_G,
        'H' => &SPRITE_H,
        'I' => &SPRITE_I,
        'J' => &SPRITE_J,
        'K' => &SPRITE_K,
        'L' => &SPRITE_L,
        'M' => &SPRITE_M,
        'N' => &SPRITE_N,
        'O' => &SPRITE_O,
        'P' => &SPRITE_P,
        'Q' => &SPRITE_Q,
        'R' => &SPRITE_R,
        'S' => &SPRITE_S,
        'T' => &SPRITE_T,
        'U' => &SPRITE_U,
        'V' => &SPRITE_V,
        'W' => &SPRITE_W,
        'X' => &SPRITE_X,
        'Y' => &SPRITE_Y,
        'Z' => &SPRITE_Z,
        ' ' => &SPRITE_SPACE,
        ',' => &SPRITE_COMMA,
        '0' => &SPRITE_0,
        '1' => &SPRITE_1,
        '2' => &SPRITE_2,
        '3' => &SPRITE_3,
        '4' => &SPRITE_4,
        '5' => &SPRITE_5,
        '6' => &SPRITE_6,
        '7' => &SPRITE_7,
        '8' => &SPRITE_8,
        '9' => &SPRITE_9,
        '[' => &SPRITE_LSBRACKET,
        ']' => &SPRITE_RSBRACKET,
        _ => {
            dbg!(letter);
            unimplemented!();
        }
    }
}
