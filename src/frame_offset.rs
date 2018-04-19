//! `PixelFrame` offset in left/right/top/bottom directions.
use super::PixelFrame;
use super::PixelColor;

/// Methods enabled by the `offset` feature.
impl PixelFrame {
    /// Offset the PixelFrame by a number of pixels in any of the possible directions:
    ///
    /// # Example
    /// ```
    /// # extern crate sensehat_screen;
    /// # use sensehat_screen::PixelFrame;
    /// # use sensehat_screen::frame::offset::Offset;
    /// # fn main() {
    ///     let frame: PixelFrame = Default::default();
    ///     let moved_1px_to_the_left = frame.offset(Offset::left(1));
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// If `offset` is out of bounds (> 8).
    pub fn offset(&self, offset: Offset) -> Self {
        match offset {
            Offset::Left(offset) => self.offset_left(offset),
            Offset::Right(offset) => self.offset_right(offset),
            Offset::Bottom(offset) => self.offset_bottom(offset),
            Offset::Top(offset) => self.offset_top(offset),
        }
    }

    // # Panics
    // If `offset` is out of bounds (> 8).
    fn offset_left(&self, offset: u8) -> Self {
        assert!(offset < 9);
        match offset {
            0 => self.clone(),
            8 => PixelFrame::new(&[PixelColor::BLACK; 64]),
            n => {
                let mut cols = Vec::with_capacity(8);
                cols.extend_from_slice(&self.as_columns()[n as usize..]);
                for _ in (8-n)..8 {
                    cols.extend_from_slice(&[vec![PixelColor::BLACK; 8]]);
                }
                PixelFrame::from_columns(cols)
            }
        }
    }

    fn offset_right(&self, offset: u8) -> Self {
        assert!(offset < 9);
        match offset {
            0 => self.clone(),
            8 => PixelFrame::new(&[PixelColor::BLACK; 64]),
            n => {
                let mut cols = Vec::with_capacity(8);
                for _ in 0..n as usize {
                    cols.extend_from_slice(&[vec![PixelColor::BLACK; 8]]);
                }
                cols.extend_from_slice(&self.as_columns()[..(8 - n as usize)]);
                PixelFrame::from_columns(cols)
            }
        }
    }

    fn offset_bottom(&self, offset: u8) -> Self {
        assert!(offset < 9);
        match offset {
            0 => self.clone(),
            8 => PixelFrame::new(&[PixelColor::BLACK; 64]),
            n => {
                let mut rows = Vec::with_capacity(8);
                for _ in 0..n as usize {
                    rows.extend_from_slice(&[vec![PixelColor::BLACK; 8]]);
                }
                rows.extend_from_slice(&self.as_rows()[..(8 - n as usize)]);
                PixelFrame::from_rows(rows)
            }
        }
    }

    fn offset_top(&self, offset: u8) -> Self {
        assert!(offset < 9);
        match offset {
            0 => self.clone(),
            8 => PixelFrame::new(&[PixelColor::BLACK; 64]),
            n => {
                let mut rows = Vec::with_capacity(8);
                rows.extend_from_slice(&self.as_rows()[n as usize..]);
                for _ in (8-n)..8 {
                    rows.extend_from_slice(&[vec![PixelColor::BLACK; 8]]);
                }
                PixelFrame::from_rows(rows)
            }
        }
    }
}

/// Offset for `PixelFrame` displacement in a given direction
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Offset {
    Left(u8),
    Right(u8),
    Bottom(u8),
    Top(u8),
}

impl Offset {
    /// Offset by `offset` pixels to the left of the LED Matrix.
    ///
    /// # Panics
    /// If `offset` is greater than 8.
    pub fn left(offset: u8) -> Self {
        assert!(offset < 9);
        Offset::Left(offset)
    }

    /// Offset by `offset` pixels to the right of the LED Matrix.
    ///
    /// # Panics
    /// If `offset` is greater than 8.
    pub fn right(offset: u8) -> Self {
        assert!(offset < 9);
        Offset::Right(offset)
    }

    /// Offset by `offset` pixels to the bottom of the LED Matrix.
    ///
    /// # Panics
    /// If `offset` is greater than 8.
    pub fn bottom(offset: u8) -> Self {
        assert!(offset < 9);
        Offset::Bottom(offset)
    }

    /// Offset by `offset` pixels to the top of the LED Matrix.
    ///
    /// # Panics
    /// If `offset` is greater than 8.
    pub fn top(offset: u8) -> Self {
        assert!(offset < 9);
        Offset::Top(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DARK: PixelColor = PixelColor::BLACK;
    const BLUE: PixelColor = PixelColor::BLUE;

    const FRAME_ZERO: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
    ];

    const SYMBOL_FRAME: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
    ];

    // RIGHT
    const OFFSET_RIGHT_ONE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, BLUE, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_TWO: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, BLUE, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_THREE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, BLUE, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_FOUR: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, BLUE, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_FIVE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_SIX: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_SEVEN: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
    ];

    // LEFT
    const OFFSET_LEFT_ONE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE, DARK,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, DARK,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, DARK,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
    ];

    const OFFSET_LEFT_TWO: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, BLUE, DARK, DARK, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, DARK, DARK, //
        DARK, DARK, DARK, BLUE, BLUE, BLUE, DARK, DARK, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
    ];

    const OFFSET_LEFT_THREE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, BLUE, DARK, DARK, DARK, //
        DARK, DARK, DARK, BLUE, BLUE, DARK, DARK, DARK, //
        DARK, DARK, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
    ];

    const OFFSET_LEFT_FOUR: [PixelColor; 64] = [
        DARK, DARK, DARK, BLUE, DARK, DARK, DARK, DARK, //
        DARK, DARK, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
    ];

    const OFFSET_LEFT_FIVE: [PixelColor; 64] = [
        DARK, DARK, BLUE, DARK, DARK, DARK, DARK, DARK, //
        DARK, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
    ];

    const OFFSET_LEFT_SIX: [PixelColor; 64] = [
        DARK, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
    ];

    const OFFSET_LEFT_SEVEN: [PixelColor; 64] = [
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    // BOTTOM
    const OFFSET_TOP_ONE: [PixelColor; 64] = [
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_TWO: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_THREE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_FOUR: [PixelColor; 64] = [
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_FIVE: [PixelColor; 64] = [
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_SIX: [PixelColor; 64] = [
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_SEVEN: [PixelColor; 64] = [
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    // BOTTOM
    const OFFSET_BOTTOM_ONE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
    ];

    const OFFSET_BOTTOM_TWO: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_THREE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_FOUR: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_FIVE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_SIX: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_SEVEN: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
    ];

    #[test]
    fn pixel_frame_offsets_to_the_left() {
        let symbol = PixelFrame::new(&SYMBOL_FRAME);
        assert_eq!(
            symbol.offset(Offset::left(0)),
            PixelFrame::new(&SYMBOL_FRAME)
        );
        assert_eq!(
            symbol.offset(Offset::left(1)),
            PixelFrame::new(&OFFSET_LEFT_ONE)
        );
        assert_eq!(
            symbol.offset(Offset::left(2)),
            PixelFrame::new(&OFFSET_LEFT_TWO)
        );
        assert_eq!(
            symbol.offset(Offset::left(3)),
            PixelFrame::new(&OFFSET_LEFT_THREE)
        );
        assert_eq!(
            symbol.offset(Offset::left(4)),
            PixelFrame::new(&OFFSET_LEFT_FOUR)
        );
        assert_eq!(
            symbol.offset(Offset::left(5)),
            PixelFrame::new(&OFFSET_LEFT_FIVE)
        );
        assert_eq!(
            symbol.offset(Offset::left(6)),
            PixelFrame::new(&OFFSET_LEFT_SIX)
        );
        assert_eq!(
            symbol.offset(Offset::left(7)),
            PixelFrame::new(&OFFSET_LEFT_SEVEN)
        );
        assert_eq!(symbol.offset(Offset::left(8)), PixelFrame::new(&FRAME_ZERO));
    }

    #[test]
    #[should_panic]
    fn pixel_frame_panics_when_offset_to_the_left_is_greater_than_8() {
        let symbol = PixelFrame::new(&SYMBOL_FRAME);
        let _ = symbol.offset(Offset::left(9));
    }

    #[test]
    fn pixel_frame_offsets_to_the_right() {
        let symbol = PixelFrame::new(&SYMBOL_FRAME);
        assert_eq!(
            symbol.offset(Offset::right(0)),
            PixelFrame::new(&SYMBOL_FRAME)
        );
        assert_eq!(
            symbol.offset(Offset::right(1)),
            PixelFrame::new(&OFFSET_RIGHT_ONE)
        );
        assert_eq!(
            symbol.offset(Offset::right(2)),
            PixelFrame::new(&OFFSET_RIGHT_TWO)
        );
        assert_eq!(
            symbol.offset(Offset::right(3)),
            PixelFrame::new(&OFFSET_RIGHT_THREE)
        );
        assert_eq!(
            symbol.offset(Offset::right(4)),
            PixelFrame::new(&OFFSET_RIGHT_FOUR)
        );
        assert_eq!(
            symbol.offset(Offset::right(5)),
            PixelFrame::new(&OFFSET_RIGHT_FIVE)
        );
        assert_eq!(
            symbol.offset(Offset::right(6)),
            PixelFrame::new(&OFFSET_RIGHT_SIX)
        );
        assert_eq!(
            symbol.offset(Offset::right(7)),
            PixelFrame::new(&OFFSET_RIGHT_SEVEN)
        );
        assert_eq!(
            symbol.offset(Offset::right(8)),
            PixelFrame::new(&FRAME_ZERO)
        );
    }

    #[test]
    #[should_panic]
    fn pixel_frame_panics_when_offset_to_the_right_is_greater_than_8() {
        let symbol = PixelFrame::new(&SYMBOL_FRAME);
        let _ = symbol.offset(Offset::right(9));
    }

    #[test]
    fn pixel_frame_offsets_to_the_bottom() {
        let symbol = PixelFrame::new(&SYMBOL_FRAME);
        assert_eq!(
            symbol.offset(Offset::bottom(0)),
            PixelFrame::new(&SYMBOL_FRAME)
        );
        assert_eq!(
            symbol.offset(Offset::bottom(1)),
            PixelFrame::new(&OFFSET_BOTTOM_ONE)
        );
        assert_eq!(
            symbol.offset(Offset::bottom(2)),
            PixelFrame::new(&OFFSET_BOTTOM_TWO)
        );
        assert_eq!(
            symbol.offset(Offset::bottom(3)),
            PixelFrame::new(&OFFSET_BOTTOM_THREE)
        );
        assert_eq!(
            symbol.offset(Offset::bottom(4)),
            PixelFrame::new(&OFFSET_BOTTOM_FOUR)
        );
        assert_eq!(
            symbol.offset(Offset::bottom(5)),
            PixelFrame::new(&OFFSET_BOTTOM_FIVE)
        );
        assert_eq!(
            symbol.offset(Offset::bottom(6)),
            PixelFrame::new(&OFFSET_BOTTOM_SIX)
        );
        assert_eq!(
            symbol.offset(Offset::bottom(7)),
            PixelFrame::new(&OFFSET_BOTTOM_SEVEN)
        );
        assert_eq!(
            symbol.offset(Offset::bottom(8)),
            PixelFrame::new(&FRAME_ZERO)
        );
    }

    #[test]
    #[should_panic]
    fn pixel_frame_panics_when_offset_to_the_bottom_is_greater_than_8() {
        let symbol = PixelFrame::new(&SYMBOL_FRAME);
        let _ = symbol.offset(Offset::bottom(9));
    }

    #[test]
    fn pixel_frame_offsets_to_the_top() {
        let symbol = PixelFrame::new(&SYMBOL_FRAME);
        assert_eq!(
            symbol.offset(Offset::top(0)),
            PixelFrame::new(&SYMBOL_FRAME)
        );
        assert_eq!(
            symbol.offset(Offset::top(1)),
            PixelFrame::new(&OFFSET_TOP_ONE)
        );
        assert_eq!(
            symbol.offset(Offset::top(2)),
            PixelFrame::new(&OFFSET_TOP_TWO)
        );
        assert_eq!(
            symbol.offset(Offset::top(3)),
            PixelFrame::new(&OFFSET_TOP_THREE)
        );
        assert_eq!(
            symbol.offset(Offset::top(4)),
            PixelFrame::new(&OFFSET_TOP_FOUR)
        );
        assert_eq!(
            symbol.offset(Offset::top(5)),
            PixelFrame::new(&OFFSET_TOP_FIVE)
        );
        assert_eq!(
            symbol.offset(Offset::top(6)),
            PixelFrame::new(&OFFSET_TOP_SIX)
        );
        assert_eq!(
            symbol.offset(Offset::top(7)),
            PixelFrame::new(&OFFSET_TOP_SEVEN)
        );
        assert_eq!(symbol.offset(Offset::top(8)), PixelFrame::new(&FRAME_ZERO));
    }

    #[test]
    #[should_panic]
    fn pixel_frame_panics_when_offset_to_the_top_is_greater_than_8() {
        let symbol = PixelFrame::new(&SYMBOL_FRAME);
        let _ = symbol.offset(Offset::top(9));
    }
}
