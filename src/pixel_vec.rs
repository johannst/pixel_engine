use crate::PixelBuffer;

/// Implementation of [PixelBuffer](crate::PixelBuffer) based on [Vec](std::vec::Vec).
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
