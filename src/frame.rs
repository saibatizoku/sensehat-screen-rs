//! Frames for the LED Matrix screen
#[cfg(feature = "clip")]
#[path = "frame_clip.rs"]
pub mod clip;
#[cfg(feature = "offset")]
#[path = "frame_offset.rs"]
pub mod offset;
#[cfg(feature = "rotate")]
#[path = "frame_rotate.rs"]
pub mod rotate;

use super::color::{PixelColor, Rgb565};
use std::fmt::{self, Write};
use std::ops::{Index, IndexMut};

/// A single frame on the screen. Contains a private `[Rgb565; 64]`.
#[derive(Copy, Clone)]
pub struct FrameLine([Rgb565; 64]);

impl FrameLine {
    //  Defaults to an empty vector with capacity for 128 bytes.
    fn new() -> Self {
        FrameLine([Rgb565::default(); 64])
    }

    /// Create a new `FrameLine` instance, given a slice of bytes.
    pub fn from_slice(bytes: &[u8; 128]) -> Self {
        let colors = bytes.chunks(2)
                          .map(|chunk| Rgb565::from([chunk[0], chunk[1]]))
                          .enumerate()
                          .fold([Rgb565::default(); 64],
                                |mut color_array, (index, color)| {
                                    color_array[index] = color;
                                    color_array
                                });
        FrameLine(colors)
    }

    /// Create a new `FrameLine` instance, given a slice of `PixelColor`.
    pub fn from_pixels(pixels: &[PixelColor; 64]) -> Self {
        let colors = pixels.iter()
                           .map(Rgb565::from)
                           .enumerate()
                           .fold([Rgb565::default(); 64],
                                 |mut color_array, (index, color)| {
                                     color_array[index] = color;
                                     color_array
                                 });
        FrameLine(colors)
    }

    /// Returns the `FrameLine` as a slice of bytes.
    pub fn as_bytes(&self) -> [u8; 128] {
        self.0.iter()
            .cloned()
            .map(|color| {
                     let bytes: [u8; 2] = color.into();
                     bytes
                 })
            .enumerate()
            .fold([0u8; 128], |mut byte_array, (index, color)| {
                byte_array[index * 2] = color[0];
                byte_array[(index * 2) + 1] = color[1];
                byte_array
            })
    }
}

impl Default for FrameLine {
    fn default() -> Self {
        FrameLine::new()
    }
}

impl fmt::Debug for FrameLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rows = self.0.chunks(8).fold(String::new(), |mut s, row| {
            write!(&mut s, "\n[").unwrap();
            for &px in row {
                let rgbu16: u16 = px.into();
                write!(&mut s, " {:04X}", rgbu16).unwrap();
            }
            write!(&mut s, " ]").unwrap();
            s
        });
        write!(f, "FrameLine:\n{}", rows)
    }
}

impl PartialEq for FrameLine {
    fn eq(&self, other: &FrameLine) -> bool {
        self.0.iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

/// A frame of pixels. This is the basic representation for the LED Matrix display.
#[derive(Copy, Clone)]
pub struct PixelFrame([PixelColor; 64]);

impl fmt::Debug for PixelFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rows = self.0.chunks(8).fold(String::new(), |mut s, row| {
            writeln!(&mut s, "{:?}", row).unwrap();
            s
        });
        write!(f, "PixelFrame:\n{}", rows)
    }
}

impl Default for PixelFrame {
    fn default() -> Self {
        PixelFrame([PixelColor::BLACK; 64])
    }
}

impl PartialEq for PixelFrame {
    fn eq(&self, other: &PixelFrame) -> bool {
        self.0.iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl PixelFrame {
    pub const BLACK: PixelFrame = PixelFrame([PixelColor::BLACK; 64]);
    pub const RED: PixelFrame = PixelFrame([PixelColor::RED; 64]);
    pub const BLUE: PixelFrame = PixelFrame([PixelColor::BLUE; 64]);
    pub const GREEN: PixelFrame = PixelFrame([PixelColor::GREEN; 64]);
    pub const WHITE: PixelFrame = PixelFrame([PixelColor::WHITE; 64]);
    pub const YELLOW: PixelFrame = PixelFrame([PixelColor::YELLOW; 64]);
    pub const CYAN: PixelFrame = PixelFrame([PixelColor::CYAN; 64]);
    pub const MAGENTA: PixelFrame = PixelFrame([PixelColor::MAGENTA; 64]);
}

impl PixelFrame {
    /// Create a `FrameLine` representing the current `PixelFrame`.
    pub fn new(pixels: &[PixelColor; 64]) -> Self {
        PixelFrame(*pixels)
    }
    /// Create a `FrameLine` representing the current `PixelFrame`.
    pub fn frame_line(&self) -> FrameLine {
        let colors = self.0
                         .iter()
                         .enumerate()
                         .fold([PixelColor::BLACK; 64], |mut c, (idx, px)| {
                             c[idx] = *px;
                             c
                         });
        FrameLine::from_pixels(&colors)
    }

    /// Transpose the LED Matrix. Rows become columns.
    pub fn transpose(&mut self) {
        for row in 0..8 {
            for col in row..8 {
                let idx = row * 8 + col;
                let idx_transpose = col * 8 + row;
                self.0.swap(idx, idx_transpose);
            }
        }
    }

    /// Flip the LED Matrix horizontally.
    pub fn flip_h(&mut self) {
        for row in self.0.chunks_mut(8) {
            row.reverse();
        }
    }

    /// Flip the LED Matrix vertically.
    pub fn flip_v(&mut self) {
        self.reverse();
        self.flip_h();
    }

    /// Reverse the LED Matrix.
    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    /// Returns a `[[PixelColor; 8]; 8]`, organized by rows, from top to bottom.
    pub fn as_rows(&self) -> [[PixelColor; 8]; 8] {
        let pixels = self.0;
        let mut rows = [[PixelColor::default(); 8]; 8];
        pixels.chunks(8).enumerate().for_each(|(idx, row)| {
                                                  rows[idx].copy_from_slice(row);
                                              });
        rows
    }

    /// Returns a `[[PixelColor; 8]; 8]`, organized by columns, from left to right.
    pub fn as_columns(&self) -> [[PixelColor; 8]; 8] {
        let mut pixels = *self;
        pixels.transpose();
        let mut columns = [[PixelColor::default(); 8]; 8];
        pixels.0.chunks(8).enumerate().for_each(|(idx, col)| {
                                                    columns[idx].copy_from_slice(col);
                                                });
        columns
    }

    /// Create a new `PixelFrame` from a `[[PixelColor; 8]; 8]`, of 8 rows with 8 `PixelColor`s.
    pub fn from_rows(rows: &[[PixelColor; 8]; 8]) -> Self {
        let mut pixels = [PixelColor::default(); 64];
        for (row_idx, row) in rows.into_iter().enumerate() {
            for (col_idx, &px) in row.into_iter().enumerate() {
                pixels[row_idx * 8 + col_idx] = px;
            }
        }
        PixelFrame(pixels)
    }

    /// Create a new `PixelFrame` from a `[[PixelColor; 8]; 8]`, of 8 columns with 8 `PixelColor`s.
    pub fn from_columns(columns: &[[PixelColor; 8]; 8]) -> Self {
        let mut pixels = [PixelColor::default(); 64];
        for (col_idx, col) in columns.into_iter().enumerate() {
            for (row_idx, &px) in col.into_iter().enumerate() {
                pixels[row_idx * 8 + col_idx] = px;
            }
        }
        PixelFrame(pixels)
    }
}

impl<'a> From<&'a [PixelColor; 64]> for PixelFrame {
    fn from(array: &'a [PixelColor; 64]) -> Self {
        PixelFrame::new(array)
    }
}

impl From<[PixelColor; 64]> for PixelFrame {
    fn from(array: [PixelColor; 64]) -> Self {
        PixelFrame(array)
    }
}

impl Into<[PixelColor; 64]> for PixelFrame {
    fn into(self) -> [PixelColor; 64] {
        self.0
    }
}

impl Index<usize> for PixelFrame {
    type Output = PixelColor;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for PixelFrame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

fn clip_pixel_frames_offset_left(first: PixelFrame, second: PixelFrame, offset: u8) -> PixelFrame {
    unimplemented!();
}

fn clip_pixel_frames_offset_right(first: PixelFrame, second: PixelFrame, offset: u8) -> PixelFrame {
    unimplemented!();
}

fn clip_pixel_frames_offset_top(first: PixelFrame, second: PixelFrame, offset: u8) -> PixelFrame {
    unimplemented!();
}

fn clip_pixel_frames_offset_bottom(first: PixelFrame,
                                   second: PixelFrame,
                                   offset: u8)
                                   -> PixelFrame {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: PixelColor = PixelColor::RED;
    const ONE: PixelColor = PixelColor::WHITE;
    const TWO: PixelColor = PixelColor::BLUE;
    const PIXEL_FRAME: &[PixelColor; 64] = &[
        RED, ONE, RED, TWO, RED, ONE, RED, TWO, //
        RED, ONE, RED, TWO, RED, ONE, RED, TWO, //
        RED, ONE, RED, TWO, RED, ONE, RED, TWO, //
        RED, ONE, RED, TWO, RED, ONE, RED, TWO, //
        RED, ONE, RED, TWO, RED, ONE, RED, TWO, //
        RED, ONE, RED, TWO, RED, ONE, RED, TWO, //
        RED, ONE, RED, TWO, RED, ONE, RED, TWO, //
        RED, ONE, RED, TWO, RED, ONE, RED, TWO, //
    ];
    fn test_rows() -> [[PixelColor; 8]; 8] {
        [[RED, ONE, RED, TWO, RED, ONE, RED, TWO]; 8]
    }
    fn test_columns() -> [[PixelColor; 8]; 8] {
        [[RED; 8], [ONE; 8], [RED; 8], [TWO; 8], [RED; 8], [ONE; 8], [RED; 8], [TWO; 8]]
    }

    #[test]
    fn frame_line_is_created_from_slice_of_bytes() {
        let color: [u8; 128] = [0xE0; 128];
        let frame_line = FrameLine::from_slice(&color);
        frame_line.as_bytes()
                  .into_iter()
                  .zip(color.into_iter())
                  .for_each(|(a, b)| {
                                assert_eq!(a, b);
                            });
    }

    #[cfg(not(feature = "big-endian"))]
    #[test]
    fn frame_line_is_created_from_slice_of_pixel_color() {
        let blue = PixelColor::from_rgb565_bytes([0x1F, 0x00]);
        let frame_line = FrameLine::from_pixels(&[blue; 64]);
        frame_line.as_bytes().chunks(2).for_each(|chunk| {
                                                     assert_eq!([chunk[0], chunk[1]], [0x1F, 0x00]);
                                                 });
    }

    #[cfg(feature = "big-endian")]
    #[test]
    fn frame_line_is_created_from_slice_of_pixel_color() {
        let blue = PixelColor::from_rgb565_bytes([0x00, 0x1F]);
        let frame_line = FrameLine::from_pixels(&[blue; 64]);
        frame_line.as_bytes().chunks(2).for_each(|chunk| {
                                                     assert_eq!([chunk[0], chunk[1]], [0x00, 0x1F]);
                                                 });
    }

    #[test]
    fn pixel_frame_is_created_from_a_slice_of_pixel_color() {
        let color_frame = [PixelColor::YELLOW; 64];
        let pixel_frame = PixelFrame::new(&color_frame);
        pixel_frame.0
                   .into_iter()
                   .zip(color_frame.into_iter())
                   .for_each(|(a, b)| {
                                 assert_eq!(a, b);
                             });
    }

    #[test]
    fn pixel_frame_creates_a_frame_line_of_the_current_state() {
        let color_frame = [PixelColor::GREEN; 64];
        let pixel_frame = PixelFrame::new(&color_frame);
        assert_eq!(pixel_frame.frame_line(),
                   FrameLine::from_pixels(&color_frame));
    }

    #[test]
    fn pixel_frame_is_represented_as_rows_of_pixel_color() {
        let pixel_frame = PixelFrame::new(PIXEL_FRAME);
        assert_eq!(pixel_frame.as_rows(), test_rows());
    }

    #[test]
    fn pixel_frame_is_represented_as_columns_of_pixel_color() {
        let pixel_frame = PixelFrame::new(PIXEL_FRAME);
        assert_eq!(pixel_frame.as_columns(), test_columns());
    }

    #[test]
    fn pixel_frame_is_created_from_rows_of_pixel_color() {
        let pixel_frame = PixelFrame::new(PIXEL_FRAME);
        assert_eq!(PixelFrame::from_rows(&test_rows()), pixel_frame);
    }

    #[test]
    fn pixel_frame_is_created_from_columns_of_pixel_color() {
        let pixel_frame = PixelFrame::new(PIXEL_FRAME);
        assert_eq!(PixelFrame::from_columns(&test_columns()), pixel_frame);
    }
}
