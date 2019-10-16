use crate::Pixel::*;
use crate::Sprite;

use std::collections::HashMap;

lazy_static! {
    static ref FONT: HashMap<char, Sprite> = {
        let mut m = HashMap::new();
        m.insert(
            'A',
            Sprite {
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
            },
        );
        m.insert(
            'B',
            Sprite {
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
            },
        );
        m.insert(
            'C',
            Sprite {
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
            },
        );
        m.insert(
            'D',
            Sprite {
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
            },
        );
        m.insert(
            'E',
            Sprite {
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
            },
        );
        m.insert(
            'F',
            Sprite {
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
            },
        );
        m.insert(
            'G',
            Sprite {
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
            },
        );
        m.insert(
            'H',
            Sprite {
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
            },
        );
        m.insert(
            'I',
            Sprite {
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
            },
        );
        m.insert(
            'J',
            Sprite {
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
            },
        );
        m.insert(
            'K',
            Sprite {
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
            },
        );
        m.insert(
            'L',
            Sprite {
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
            },
        );
        m.insert(
            'M',
            Sprite {
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
            },
        );
        m.insert(
            'N',
            Sprite {
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
            },
        );
        m.insert(
            'O',
            Sprite {
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
            },
        );
        m.insert(
            'P',
            Sprite {
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
            },
        );
        m.insert(
            'Q',
            Sprite {
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
            },
        );
        m.insert(
            'R',
            Sprite {
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
            },
        );
        m.insert(
            'S',
            Sprite {
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
            },
        );
        m.insert(
            'T',
            Sprite {
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
            },
        );
        m.insert(
            'U',
            Sprite {
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
            },
        );
        m.insert(
            'V',
            Sprite {
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
            },
        );
        m.insert(
            'W',
            Sprite {
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
            },
        );
        m.insert(
            'X',
            Sprite {
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
            },
        );
        m.insert(
            'Y',
            Sprite {
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
            },
        );
        m.insert(
            'Z',
            Sprite {
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
            },
        );
        m.insert(
            '0',
            Sprite {
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
            },
        );
        m.insert(
            '1',
            Sprite {
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
            },
        );
        m.insert(
            '2',
            Sprite {
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
            },
        );
        m.insert(
            '3',
            Sprite {
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
            },
        );
        m.insert(
            '4',
            Sprite {
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
            },
        );
        m.insert(
            '5',
            Sprite {
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
            },
        );
        m.insert(
            '6',
            Sprite {
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
            },
        );
        m.insert(
            '7',
            Sprite {
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
            },
        );
        m.insert(
            '8',
            Sprite {
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
            },
        );
        m.insert(
            '9',
            Sprite {
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
            },
        );
        m.insert(
            ' ',
            Sprite {
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
            },
        );
        m.insert(
            ',',
            Sprite {
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
            },
        );
        m.insert(
            ':',
            Sprite {
                lines: [
                    [U, U, U, U, U, U, U, U],
                    [U, U, U, C, C, U, U, U],
                    [U, U, U, C, C, U, U, U],
                    [U, U, U, U, U, U, U, U],
                    [U, U, U, U, U, U, U, U],
                    [U, U, U, C, C, U, U, U],
                    [U, U, U, C, C, U, U, U],
                    [U, U, U, U, U, U, U, U],
                ],
            },
        );
        m.insert(
            '-',
            Sprite {
                lines: [
                    [U, U, U, U, U, U, U, U],
                    [U, U, U, U, U, U, U, U],
                    [U, U, U, U, U, U, U, U],
                    [U, U, C, C, C, C, U, U],
                    [U, U, U, U, U, U, U, U],
                    [U, U, U, U, U, U, U, U],
                    [U, U, U, U, U, U, U, U],
                    [U, U, U, U, U, U, U, U],
                ],
            },
        );
        m.insert(
            '[',
            Sprite {
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
            },
        );
        m.insert(
            ']',
            Sprite {
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
            },
        );

        m
    };
}

pub fn get_sprite(letter: char) -> &'static Sprite {
    match FONT.get(&letter) {
        Some(s) => s,
        None => {
            dbg!(letter);
            unimplemented!();
        }
    }
}
