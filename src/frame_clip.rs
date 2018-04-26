//! Support for combining `PixelFrame`s.
//!
//! The `FrameClip` is the type that merges `PixelFrame` by rows or by columns
//!
//!
use super::PixelFrame;
use super::offset::Offset;

/// Methods enabled by the `clip` feature.
impl PixelFrame {
    /// Create a `FrameClip` with this and another `PixelFrame`.
    pub fn clip(&self, other: &PixelFrame) -> FrameClip {
        FrameClip::new(self.clone(), other.clone())
    }
}

/// A clip made of two `PixelFrame`s.
/// # Horizontal `FrameClip`
///
/// In horizontal clips, frames are placed side-by-side, and a `FrameClip` is specified
/// using either `Offset::Right(_)`, or `Offset::Left(_)` with the `FrameClip::offset` method.
///
///
/// `Offset::Right(_)`
/// ------------------
///
/// ```text
///                 |---------------|
///                 |    Right(8)   |
///                 |---------------|
///               |---------------| |
///               |    Right(7)   | |
///               |---------------| |
///             |---------------| | |
///             |    Right(6)   | | |
///             |---------------| | |
///           |---------------| | | |
///           |    Right(5)   | | | |
///           |---------------| | | |
///         |---------------| | | | |
///         |    Right(4)   | | | | |
///         |---------------| | | | |
///       |---------------| | | | | |
///       |    Right(3)   | | | | | |
///       |---------------| | | | | |
///     |---------------| | | | | | |
///     |    Right(2)   | | | | | | |
///     |---------------| | | | | | |
///   |---------------| | | | | | | |
///   |    Right(1)   | | | | | | | |
///   |---------------| | | | | | | |
/// |---------------| | | | | | | | |
/// |    Right(0)   | | | | | | | | |
/// |---------------| | | | | | | | |
///                 | | | | | | | | |
/// |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|  <- individual frame COLUMN index
/// |---------------|---------------|
/// |    frame 1    |    frame 2    |
/// |---------------|---------------|
/// ```
///
/// ### Example
///
/// ```rust
/// extern crate sensehat_screen;
/// use sensehat_screen::{FrameClip, PixelColor, PixelFrame};
/// use sensehat_screen::frame::offset::Offset;
///
/// fn main() {
///     let frame_1 = PixelFrame::new(&[PixelColor::YELLOW; 64]);
///     let frame_2 = PixelFrame::new(&[PixelColor::BLUE; 64]);
///     let clip = FrameClip::new(frame_1.clone(), frame_2.clone());
///     // Offset of `0`, shows the first frame.
///     assert_eq!(clip.offset(Offset::Right(0)), frame_1);
///
///     let offset_2_cols = clip.offset(Offset::Right(2)).as_columns();
///     // expect 2 yellow columns
///     assert_eq!(&offset_2_cols[..2], &[&[PixelColor::BLUE; 8]; 2]);
///     // expect 6 blue columns
///     assert_eq!(&offset_2_cols[2..], &[&[PixelColor::YELLOW; 8]; 6]);
///
///     // Offset of `8`, shows the second frame.
///     assert_eq!(clip.offset(Offset::Right(8)), frame_2);
/// }
/// ```
/// `Offset::Left(_)`
/// ----------------
/// ```text
///                 |---------------|
///                 |    Left(0)    |
///                 |---------------|
///               |---------------| |
///               |    Left(1)    | |
///               |---------------| |
///             |---------------| | |
///             |    Left(2)    | | |
///             |---------------| | |
///           |---------------| | | |
///           |    Left(3)    | | | |
///           |---------------| | | |
///         |---------------| | | | |
///         |    Left(4)    | | | | |
///         |---------------| | | | |
///       |---------------| | | | | |
///       |    Left(5)    | | | | | |
///       |---------------| | | | | |
///     |---------------| | | | | | |
///     |    Left(6)    | | | | | | |
///     |---------------| | | | | | |
///   |---------------| | | | | | | |
///   |    Left(7)    | | | | | | | |
///   |---------------| | | | | | | |
/// |---------------| | | | | | | | |
/// |    Left(8)    | | | | | | | | |
/// |---------------| | | | | | | | |
///                 | | | | | | | | |
/// |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|  <- individual frame COLUMN index
/// |---------------|---------------|
/// |      frame 2  |      frame 1  |
/// |---------------|---------------|
///
/// ```
///
/// ### Example
///
/// ```rust
/// extern crate sensehat_screen;
/// use sensehat_screen::{FrameClip, PixelColor, PixelFrame};
/// use sensehat_screen::frame::offset::Offset;
///
/// fn main() {
///     let frame_1 = PixelFrame::new(&[PixelColor::YELLOW; 64]);
///     let frame_2 = PixelFrame::new(&[PixelColor::BLUE; 64]);
///     let clip = FrameClip::new(frame_1.clone(), frame_2.clone());
///     // Offset of `0`, shows the first frame.
///     assert_eq!(clip.offset(Offset::Left(0)), frame_1);
///
///     let offset_4_cols = clip.offset(Offset::Left(4)).as_columns();
///     // expect 4 yellow columns
///     assert_eq!(&offset_4_cols[..4], &[&[PixelColor::YELLOW; 8]; 4]);
///     // expect 4 blue columns
///     assert_eq!(&offset_4_cols[4..], &[&[PixelColor::BLUE; 8]; 4]);
///
///     // Offset of `8`, shows the second frame.
///     assert_eq!(clip.offset(Offset::Left(8)), frame_2);
/// }
/// ```
///
/// Vertical `FrameClip`
/// ====================
///
/// In vertical clips, frames are placed above-and-belowe each other, and a `FrameClip` is specified
/// using either `Offset::Bottom(_)`, or `Offset::Top(_)` with the `FrameClip::offset` method.
///
/// `Offset::Bottom(_)`
/// -------------------
/// ```text
///                 |---------------|
///                 |   Bottom(0)   |
///                 |---------------|
///               |---------------| |
///               |   Bottom(1)   | |
///               |---------------| |
///             |---------------| | |
///             |   Bottom(2)   | | |
///             |---------------| | |
///           |---------------| | | |
///           |   Bottom(3)   | | | |
///           |---------------| | | |
///         |---------------| | | | |
///         |   Bottom(4)   | | | | |
///         |---------------| | | | |
///       |---------------| | | | | |
///       |   Bottom(5)   | | | | | |
///       |---------------| | | | | |
///     |---------------| | | | | | |
///     |   Bottom(6)   | | | | | | |
///     |---------------| | | | | | |
///   |---------------| | | | | | | |
///   |   Bottom(7)   | | | | | | | |
///   |---------------| | | | | | | |
/// |---------------| | | | | | | | |
/// |   Bottom(8)   | | | | | | | | |
/// |---------------| | | | | | | | |
///                 | | | | | | | | |
/// |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|  <- individual frame ROW index
/// |---------------|---------------|
/// |      frame 2  |      frame 1  |
/// |---------------|---------------|
///
/// ```
///
/// ### Example
///
/// ```rust
/// extern crate sensehat_screen;
/// use sensehat_screen::{FrameClip, PixelColor, PixelFrame};
/// use sensehat_screen::frame::offset::Offset;
///
/// fn main() {
///     let frame_1 = PixelFrame::new(&[PixelColor::MAGENTA; 64]);
///     let frame_2 = PixelFrame::new(&[PixelColor::WHITE; 64]);
///     let clip = FrameClip::new(frame_1.clone(), frame_2.clone());
///     // Offset of `0`, shows the first frame.
///     assert_eq!(clip.offset(Offset::Bottom(0)), frame_1);
///
///     let offset_2_rows = clip.offset(Offset::Bottom(2)).as_rows();
///     // expect 2 yellow rows
///     assert_eq!(&offset_2_rows[..2], &[&[PixelColor::WHITE; 8]; 2]);
///     // expect 6 blue rows
///     assert_eq!(&offset_2_rows[2..], &[&[PixelColor::MAGENTA; 8]; 6]);
///
///     // Offset of `8`, shows the second frame.
///     assert_eq!(clip.offset(Offset::Bottom(8)), frame_2);
/// }
/// ```
///
/// `Offset::Top(_)`
/// ----------------
/// ```text
///                 |---------------|
///                 |    Top(8)     |
///                 |---------------|
///               |---------------| |
///               |    Top(7)     | |
///               |---------------| |
///             |---------------| | |
///             |    Top(6)     | | |
///             |---------------| | |
///           |---------------| | | |
///           |    Top(5)     | | | |
///           |---------------| | | |
///         |---------------| | | | |
///         |    Top(4)     | | | | |
///         |---------------| | | | |
///       |---------------| | | | | |
///       |    Top(3)     | | | | | |
///       |---------------| | | | | |
///     |---------------| | | | | | |
///     |    Top(2)     | | | | | | |
///     |---------------| | | | | | |
///   |---------------| | | | | | | |
///   |    Top(1)     | | | | | | | |
///   |---------------| | | | | | | |
/// |---------------| | | | | | | | |
/// |    Top(0)     | | | | | | | | |
/// |---------------| | | | | | | | |
///                 | | | | | | | | |
/// |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|  <- individual frame ROW index
/// |---------------|---------------|
/// |    frame 1    |    frame 2    |
/// |---------------|---------------|
/// ```
///
/// ### Example
///
/// ```rust
/// extern crate sensehat_screen;
/// use sensehat_screen::{FrameClip, PixelColor, PixelFrame};
/// use sensehat_screen::frame::offset::Offset;
///
/// fn main() {
///     let frame_1 = PixelFrame::new(&[PixelColor::GREEN; 64]);
///     let frame_2 = PixelFrame::new(&[PixelColor::BLACK; 64]);
///     let clip = FrameClip::new(frame_1.clone(), frame_2.clone());
///     // Offset of `0`, shows the first frame.
///     assert_eq!(clip.offset(Offset::Top(0)), frame_1);
///
///     let offset_7_rows = clip.offset(Offset::Top(7)).as_rows();
///     // expect 1 green rows
///     assert_eq!(&offset_7_rows[..1], &[&[PixelColor::GREEN; 8]; 1]);
///     // expect 7 blue rows
///     assert_eq!(&offset_7_rows[1..], &[&[PixelColor::BLACK; 8]; 7]);
///
///     // Offset of `8`, shows the second frame.
///     assert_eq!(clip.offset(Offset::Left(8)), frame_2);
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct FrameClip {
    first: PixelFrame,
    second: PixelFrame,
}

impl FrameClip {
    /// Create a new `FrameClip` from two `PixelFrame`s.
    pub fn new(first: PixelFrame, second: PixelFrame) -> Self {
        FrameClip { first, second }
    }

    /// Offset position for which to create the clipped `PixelFrame`.
    pub fn offset(&self, offset: Offset) -> PixelFrame {
        match offset {
            Offset::Left(offset) => self.offset_left(offset),
            Offset::Right(offset) => self.offset_right(offset),
            Offset::Bottom(offset) => self.offset_bottom(offset),
            Offset::Top(offset) => self.offset_top(offset),
        }
    }

    /// Reverse the order of the inner `PixelFrame`s.
    pub fn reverse(self) -> Self {
        FrameClip::new(self.second, self.first)
    }

    // # Panics
    // If `offset` is out of bounds (> 8).
    fn offset_left(&self, offset: u8) -> PixelFrame {
        assert!(offset < 9);
        match offset {
            0 => self.first.clone(),
            8 => self.second.clone(),
            n => {
                let mut cols = Vec::with_capacity(8);
                cols.extend_from_slice(&self.first.as_columns()[n as usize..]);
                cols.extend_from_slice(&self.second.as_columns()[..n as usize]);
                PixelFrame::from_columns(cols)
            }
        }
    }

    fn offset_right(&self, offset: u8) -> PixelFrame {
        assert!(offset < 9);
        match offset {
            0 => self.first.clone(),
            8 => self.second.clone(),
            n => {
                let mut cols = Vec::with_capacity(8);
                cols.extend_from_slice(&self.second.as_columns()[(8 - n as usize)..]);
                cols.extend_from_slice(&self.first.as_columns()[..(8 - n as usize)]);
                PixelFrame::from_columns(cols)
            }
        }
    }

    fn offset_bottom(&self, offset: u8) -> PixelFrame {
        assert!(offset < 9);
        match offset {
            0 => self.first.clone(),
            8 => self.second.clone(),
            n => {
                let mut rows = Vec::with_capacity(8);
                rows.extend_from_slice(&self.second.as_rows()[(8 - n as usize)..]);
                rows.extend_from_slice(&self.first.as_rows()[..(8 - n as usize)]);
                PixelFrame::from_rows(rows)
            }
        }
    }

    fn offset_top(&self, offset: u8) -> PixelFrame {
        assert!(offset < 9);
        match offset {
            0 => self.first.clone(),
            8 => self.second.clone(),
            n => {
                let mut rows = Vec::with_capacity(8);
                rows.extend_from_slice(&self.first.as_rows()[n as usize..]);
                rows.extend_from_slice(&self.second.as_rows()[..n as usize]);
                PixelFrame::from_rows(rows)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color::PixelColor;

    const DARK: PixelColor = PixelColor::BLACK;
    const BLUE: PixelColor = PixelColor::BLUE;

    const FRAME_ONE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
    ];

    const FRAME_TWO: [PixelColor; 64] = [
        BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, //
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
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_TWO: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, DARK, DARK, BLUE, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_THREE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, BLUE, DARK, DARK, BLUE, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       BLUE, BLUE, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_RIGHT_FOUR: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, BLUE, DARK, DARK, BLUE, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        BLUE, BLUE, BLUE, DARK, DARK, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_FIVE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, BLUE, DARK, DARK, BLUE, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_SIX: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, BLUE, DARK, DARK, BLUE, //
        BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
    ];

    const OFFSET_RIGHT_SEVEN: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, DARK, DARK, DARK, BLUE, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
    ];

    // LEFT
    const OFFSET_LEFT_ONE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        ];

    const OFFSET_LEFT_TWO: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, DARK, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, DARK, BLUE, BLUE, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        ];

    const OFFSET_LEFT_THREE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, BLUE, BLUE, DARK, DARK, //
        DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, DARK, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, DARK, BLUE, BLUE, DARK, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        ];

    const OFFSET_LEFT_FOUR: [PixelColor; 64] = [
        DARK, DARK, DARK, BLUE, BLUE, DARK, DARK, DARK, //
        DARK, DARK, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, DARK, BLUE, BLUE, DARK, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        ];

    const OFFSET_LEFT_FIVE: [PixelColor; 64] = [
        DARK, DARK, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        DARK, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        BLUE, DARK, BLUE, BLUE, DARK, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        ];

    const OFFSET_LEFT_SIX: [PixelColor; 64] = [
        DARK, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        DARK, BLUE, BLUE, DARK, BLUE, BLUE, BLUE, BLUE, //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, //
        ];

    const OFFSET_LEFT_SEVEN: [PixelColor; 64] = [
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK,  //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE,  //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK,  //
        BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK,  //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK,  //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
        BLUE, BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
        BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    // TOP
    const OFFSET_TOP_ONE: [PixelColor; 64] = [
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_TWO: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, DARK,  //
    ];

    const OFFSET_TOP_THREE: [PixelColor; 64] = [
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, DARK,  //
       BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_FOUR: [PixelColor; 64] = [
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, DARK,  //
       BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_FIVE: [PixelColor; 64] = [
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, DARK,  //
       BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK,  //
    ];

    const OFFSET_TOP_SIX: [PixelColor; 64] = [
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, DARK,  //
       BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK,  //
    ];

    const OFFSET_TOP_SEVEN: [PixelColor; 64] = [
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, DARK,  //
       BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK,  //
       BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
    ];

    // BOTTOM
    const OFFSET_BOTTOM_ONE: [PixelColor; 64] = [
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, BLUE,  //
    ];

    const OFFSET_BOTTOM_TWO: [PixelColor; 64] = [
       BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_THREE: [PixelColor; 64] = [
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK,  //
       BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_FOUR: [PixelColor; 64] = [
       BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK,  //
       BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_FIVE: [PixelColor; 64] = [
       BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK,  //
       BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_SIX: [PixelColor; 64] = [
       BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK,  //
       BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
       DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, BLUE,  //
    ];

    const OFFSET_BOTTOM_SEVEN: [PixelColor; 64] = [
       BLUE, BLUE, DARK, DARK, DARK, DARK, BLUE, DARK,  //
       BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK, DARK,  //
       BLUE, DARK, BLUE, BLUE, BLUE, BLUE, BLUE, DARK,  //
       BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,  //
       DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE,  //
    ];

    #[test]
    fn frame_clip_offsets_to_the_left() {
        let symbol = PixelFrame::new(&FRAME_ONE);
        let symbol_two = PixelFrame::new(&FRAME_TWO);
        let clip = symbol.clip(&symbol_two);
        assert_eq!(clip.offset(Offset::left(0)), PixelFrame::new(&FRAME_ONE));
        assert_eq!(
            clip.offset(Offset::left(1)),
            PixelFrame::new(&OFFSET_LEFT_ONE)
        );
        assert_eq!(
            clip.offset(Offset::left(2)),
            PixelFrame::new(&OFFSET_LEFT_TWO)
        );
        assert_eq!(
            clip.offset(Offset::left(3)),
            PixelFrame::new(&OFFSET_LEFT_THREE)
        );
        assert_eq!(
            clip.offset(Offset::left(4)),
            PixelFrame::new(&OFFSET_LEFT_FOUR)
        );
        assert_eq!(
            clip.offset(Offset::left(5)),
            PixelFrame::new(&OFFSET_LEFT_FIVE)
        );
        assert_eq!(
            clip.offset(Offset::left(6)),
            PixelFrame::new(&OFFSET_LEFT_SIX)
        );
        assert_eq!(
            clip.offset(Offset::left(7)),
            PixelFrame::new(&OFFSET_LEFT_SEVEN)
        );
        assert_eq!(clip.offset(Offset::left(8)), symbol_two);
    }

    #[test]
    #[should_panic]
    fn frame_clip_panics_when_offset_to_the_left_is_greater_than_8() {
        let symbol = PixelFrame::new(&FRAME_ONE);
        let symbol_two = PixelFrame::new(&FRAME_TWO);
        let clip = symbol.clip(&symbol_two);
        let _ = clip.offset(Offset::left(9));
    }

    #[test]
    fn frame_clip_offsets_to_the_right() {
        let symbol = PixelFrame::new(&FRAME_ONE);
        let symbol_two = PixelFrame::new(&FRAME_TWO);
        let clip = symbol.clip(&symbol_two);
        assert_eq!(clip.offset(Offset::right(0)), PixelFrame::new(&FRAME_ONE));
        assert_eq!(
            clip.offset(Offset::right(1)),
            PixelFrame::new(&OFFSET_RIGHT_ONE)
        );
        assert_eq!(
            clip.offset(Offset::right(2)),
            PixelFrame::new(&OFFSET_RIGHT_TWO)
        );
        assert_eq!(
            clip.offset(Offset::right(3)),
            PixelFrame::new(&OFFSET_RIGHT_THREE)
        );
        assert_eq!(
            clip.offset(Offset::right(4)),
            PixelFrame::new(&OFFSET_RIGHT_FOUR)
        );
        assert_eq!(
            clip.offset(Offset::right(5)),
            PixelFrame::new(&OFFSET_RIGHT_FIVE)
        );
        assert_eq!(
            clip.offset(Offset::right(6)),
            PixelFrame::new(&OFFSET_RIGHT_SIX)
        );
        assert_eq!(
            clip.offset(Offset::right(7)),
            PixelFrame::new(&OFFSET_RIGHT_SEVEN)
        );
        assert_eq!(clip.offset(Offset::right(8)), symbol_two);
    }

    #[test]
    #[should_panic]
    fn frame_clip_panics_when_offset_to_the_right_is_greater_than_8() {
        let symbol = PixelFrame::new(&FRAME_ONE);
        let symbol_two = PixelFrame::new(&FRAME_TWO);
        let clip = symbol.clip(&symbol_two);
        let _ = clip.offset(Offset::right(9));
    }

    #[test]
    fn frame_clip_offsets_to_the_bottom() {
        let symbol = PixelFrame::new(&FRAME_ONE);
        let symbol_two = PixelFrame::new(&FRAME_TWO);
        let clip = symbol.clip(&symbol_two);
        assert_eq!(clip.offset(Offset::bottom(0)), PixelFrame::new(&FRAME_ONE));
        assert_eq!(
            clip.offset(Offset::bottom(1)),
            PixelFrame::new(&OFFSET_BOTTOM_ONE)
        );
        assert_eq!(
            clip.offset(Offset::bottom(2)),
            PixelFrame::new(&OFFSET_BOTTOM_TWO)
        );
        assert_eq!(
            clip.offset(Offset::bottom(3)),
            PixelFrame::new(&OFFSET_BOTTOM_THREE)
        );
        assert_eq!(
            clip.offset(Offset::bottom(4)),
            PixelFrame::new(&OFFSET_BOTTOM_FOUR)
        );
        assert_eq!(
            clip.offset(Offset::bottom(5)),
            PixelFrame::new(&OFFSET_BOTTOM_FIVE)
        );
        assert_eq!(
            clip.offset(Offset::bottom(6)),
            PixelFrame::new(&OFFSET_BOTTOM_SIX)
        );
        assert_eq!(
            clip.offset(Offset::bottom(7)),
            PixelFrame::new(&OFFSET_BOTTOM_SEVEN)
        );
        assert_eq!(clip.offset(Offset::bottom(8)), symbol_two);
    }

    #[test]
    #[should_panic]
    fn frame_clip_panics_when_offset_to_the_bottom_is_greater_than_8() {
        let symbol = PixelFrame::new(&FRAME_ONE);
        let symbol_two = PixelFrame::new(&FRAME_TWO);
        let clip = symbol.clip(&symbol_two);
        let _ = clip.offset(Offset::bottom(9));
    }

    #[test]
    fn frame_clip_offsets_to_the_top() {
        let symbol = PixelFrame::new(&FRAME_ONE);
        let symbol_two = PixelFrame::new(&FRAME_TWO);
        let clip = symbol.clip(&symbol_two);
        assert_eq!(clip.offset(Offset::top(0)), PixelFrame::new(&FRAME_ONE));
        assert_eq!(
            clip.offset(Offset::top(1)),
            PixelFrame::new(&OFFSET_TOP_ONE)
        );
        assert_eq!(
            clip.offset(Offset::top(2)),
            PixelFrame::new(&OFFSET_TOP_TWO)
        );
        assert_eq!(
            clip.offset(Offset::top(3)),
            PixelFrame::new(&OFFSET_TOP_THREE)
        );
        assert_eq!(
            clip.offset(Offset::top(4)),
            PixelFrame::new(&OFFSET_TOP_FOUR)
        );
        assert_eq!(
            clip.offset(Offset::top(5)),
            PixelFrame::new(&OFFSET_TOP_FIVE)
        );
        assert_eq!(
            clip.offset(Offset::top(6)),
            PixelFrame::new(&OFFSET_TOP_SIX)
        );
        assert_eq!(
            clip.offset(Offset::top(7)),
            PixelFrame::new(&OFFSET_TOP_SEVEN)
        );
        assert_eq!(clip.offset(Offset::top(8)), symbol_two);
    }

    #[test]
    #[should_panic]
    fn frame_clip_panics_when_offset_to_the_top_is_greater_than_8() {
        let symbol = PixelFrame::new(&FRAME_ONE);
        let symbol_two = PixelFrame::new(&FRAME_TWO);
        let clip = symbol.clip(&symbol_two);
        let _ = clip.offset(Offset::top(9));
    }
}
