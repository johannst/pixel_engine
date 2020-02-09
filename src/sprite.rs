use crate::Pixel;

/// 2D structure of [Pixels](crate::Pixel) representing a Sprite.
pub struct Sprite {
    width: usize,
    pixels: Vec<Pixel>,
}

impl std::convert::From<(Vec<Pixel>, usize)> for Sprite {
    /// Convert from tuple `(pixels, width)` to Sprite.
    ///
    /// # Panics
    /// Panics if number of pixels is not divisible by width
    /// `arg.0.len() % arg.1 != 0`.
    ///
    /// # Examples
    /// ```rust
    /// use pixel_engine::Pixel::{C, U};
    /// use pixel_engine::Sprite;
    ///
    /// // creates Sprite with 2x2 pixels
    /// let sprite = Sprite::from((vec![C, C, U, U], 2));
    /// ```
    fn from(arg: (Vec<Pixel>, usize)) -> Self {
        assert!(arg.0.len() % arg.1 == 0);
        Sprite {
            width: arg.1,
            pixels: arg.0,
        }
    }
}

impl Sprite {
    /// Create new Sprite from `pixels` with `width` number of
    /// columns.
    ///
    /// # Panics
    /// Panics if number of pixels is not divisible by width
    /// `pixels.len() % width != 0`.
    ///
    /// # Examples
    /// ```rust
    /// use pixel_engine::Pixel::{C, U};
    /// use pixel_engine::Sprite;
    ///
    /// // creates Sprite with 2x2 pixels
    /// let sprite = Sprite::new(vec![C, C, U, U], 2);
    /// ```
    pub fn new(pixels: Vec<Pixel>, width: usize) -> Sprite {
        Sprite::from((pixels, width))
    }

    /// Create iterator over lines of sprite.
    ///
    /// # Examples
    /// ```rust
    /// use pixel_engine::Pixel::{C, U};
    /// use pixel_engine::Sprite;
    ///
    /// let sprite = Sprite::from((vec![C, C, U, U], 2));
    /// let mut lines = sprite.lines();
    ///
    /// assert_eq!(Some([C, C].as_ref()), lines.next());
    /// assert_eq!(Some([U, U].as_ref()), lines.next());
    /// assert_eq!(None, lines.next());
    /// ```
    pub fn lines(&self) -> impl Iterator<Item = &[Pixel]> + '_ {
        self.pixels.chunks_exact(self.width)
    }
}
