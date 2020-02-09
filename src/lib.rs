#[macro_use]
extern crate lazy_static;

mod draw;
mod font;
mod pixel_vec;
mod sprite;

pub use draw::*;
pub use pixel_vec::PixelVec;
pub use sprite::Sprite;

/// Enum indicating pixel state.
///
/// `C` indicates that a pixel is colored and should be drawn.
/// `U` indicates that a pixel is un-colored and should not be drawn.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Pixel {
    C,
    U,
}

/// Trait representing 2D buffer.
///
/// Pixels are encoded as [u32](std::u32).
pub trait PixelBuffer {
    /// Return number of `columns` of the 2D buffer.
    fn width(&self) -> usize;
    /// Return number of `lines` of the 2D buffer.
    fn height(&self) -> usize;
    /// Return linear slice to pixel buffer.
    fn buffer(&mut self) -> &mut [u32];
}

/// Optional scaling factor for drawing.
#[derive(Clone, Copy, Debug)]
pub enum PixelScale {
    X1,
    X2,
    X4,
    X8,
}

impl std::convert::From<PixelScale> for usize {
    fn from(scale: PixelScale) -> Self {
        match scale {
            PixelScale::X1 => 1,
            PixelScale::X2 => 2,
            PixelScale::X4 => 4,
            PixelScale::X8 => 8,
        }
    }
}
