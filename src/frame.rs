//! Frames for the LED Matrix screen
#[cfg(feature = "rotate")]
#[path = "frame_rotate.rs"]
pub mod rotate;
#[cfg(feature = "offset")]
#[path = "frame_offset.rs"]
pub mod offset;
#[cfg(feature = "clip")]
#[path = "frame_clip.rs"]
pub mod clip;

use super::color::PixelColor;
use std::fmt;

/// A single frame on the screen.
/// Defaults to an inner capacity for 128 bytes, suitable for the 8x8 pixel screen.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct FrameLine(Vec<u8>);

impl FrameLine {
    //  Defaults to an empty vector with capacity for 128 bytes.
    fn new() -> Self {
        FrameLine(Vec::with_capacity(128))
    }

    /// Create a new `FrameLine` instance, given a slice of bytes.
    pub fn from_slice(bytes: &[u8]) -> Self {
        FrameLine(bytes.to_vec())
    }

    /// Create a new `FrameLine` instance, given a slice of `PixelColor`.
    pub fn from_pixels(pixels: &[PixelColor]) -> Self {
        pixels
            .iter()
            .fold(FrameLine::new(), |frame, px| frame.extend(px))
    }

    // Extend the inner vector of bytes by one `PixelColor`. This method
    // consumes the current `FrameLine` instance and returns a new one,
    // useful for using with `Iterator::fold`.
    fn extend(mut self, pixel: &PixelColor) -> Self {
        self.0.extend_from_slice(&pixel.rgb565());
        self
    }

    /// Returns the `FrameLine` as a slice of bytes.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Default for FrameLine {
    fn default() -> Self {
        FrameLine::new()
    }
}

/// A frame of pixels. This is the basic representation for the LED Matrix display.
#[derive(Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct PixelFrame(Vec<PixelColor>);

impl fmt::Debug for PixelFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rows = self.0
            .chunks(8)
            .fold(String::new(), |s, row| s + &format!("{:?}\n", row));
        write!(f, "PixelFrame:\n{}", rows)
    }
}

impl PixelFrame {
    /// Create a `FrameLine` representing the current `PixelFrame`.
    pub fn new(pixels: &[PixelColor]) -> Self {
        PixelFrame(pixels.to_vec())
    }
    /// Create a `FrameLine` representing the current `PixelFrame`.
    pub fn frame_line(&self) -> FrameLine {
        FrameLine::from_pixels(self.0.as_slice())
    }

    /// Returns a `Vec<Vec<PixelColor>>`, organized by rows, from top to bottom.
    pub fn as_rows(&self) -> Vec<Vec<PixelColor>> {
        self.0.chunks(8).map(|row| row.to_vec()).collect()
    }

    /// Returns a `Vec<Vec<PixelColor>>`, organized by columns, from left to right.
    pub fn as_columns(&self) -> Vec<Vec<PixelColor>> {
        let mut columns: Vec<Vec<PixelColor>> = vec![Vec::with_capacity(8); 8];
        for (idx, px) in self.0.iter().cloned().enumerate() {
            let col_idx = idx % 8;
            columns[col_idx].push(px);
        }
        columns
    }

    /// Create a new `PixelFrame` from a `Vec<Vec<PixelColor>>`, of 8 rows with 8 `PixelColor`s.
    pub fn from_rows(rows: Vec<Vec<PixelColor>>) -> Self {
        let pixels = rows.into_iter()
            .flat_map(|row| row.into_iter())
            .collect::<Vec<PixelColor>>();
        PixelFrame(pixels)
    }

    /// Create a new `PixelFrame` from a `Vec<Vec<PixelColor>>`, of 8 columns with 8 `PixelColor`s.
    pub fn from_columns(columns: Vec<Vec<PixelColor>>) -> Self {
        let mut pixels: Vec<PixelColor> = vec![PixelColor::BLACK; 64];
        for (col_idx, col) in columns.into_iter().enumerate() {
            for (row_idx, px) in col.into_iter().enumerate() {
                pixels[row_idx * 8 + col_idx] = px;
            }
        }
        PixelFrame(pixels)
    }
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
    fn test_rows() -> Vec<Vec<PixelColor>> {
        vec![vec![RED, ONE, RED, TWO, RED, ONE, RED, TWO]; 8]
    }
    fn test_columns() -> Vec<Vec<PixelColor>> {
        vec![
            vec![RED; 8],
            vec![ONE; 8],
            vec![RED; 8],
            vec![TWO; 8],
            vec![RED; 8],
            vec![ONE; 8],
            vec![RED; 8],
            vec![TWO; 8],
        ]
    }

    #[test]
    fn frame_line_is_created_from_slice_of_bytes() {
        let green: [u8; 8] = [0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07];
        let frame_line = FrameLine::from_slice(&green);
        assert_eq!(frame_line.as_slice(), &green);
    }

    #[cfg(not(feature = "big-endian"))]
    #[test]
    fn frame_line_is_created_from_slice_of_pixel_color() {
        let blue = PixelColor::from_rgb565([0x1F, 0x00]);
        let frame_line = FrameLine::from_pixels(&[blue, blue]);
        assert_eq!(frame_line.as_slice(), &[0x1F, 0x00, 0x1F, 0x00]);
    }

    #[cfg(feature = "big-endian")]
    #[test]
    fn frame_line_is_created_from_slice_of_pixel_color() {
        let blue = PixelColor::from_rgb565([0x00, 0x1F]);
        let frame_line = FrameLine::from_pixels(&[blue, blue]);
        assert_eq!(frame_line.as_slice(), &[0x00, 0x1F, 0x00, 0x1F]);
    }

    #[test]
    fn pixel_frame_is_created_from_a_slice_of_pixel_color() {
        let color_frame = vec![PixelColor::YELLOW; 64];
        let pixel_frame = PixelFrame::new(&color_frame);
        assert_eq!(pixel_frame.0, color_frame);
    }

    #[test]
    fn pixel_frame_creates_a_frame_line_of_the_current_state() {
        let color_frame = vec![PixelColor::GREEN; 64];
        let pixel_frame = PixelFrame::new(&color_frame);
        assert_eq!(
            pixel_frame.frame_line(),
            FrameLine::from_pixels(&color_frame)
        );
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
        assert_eq!(PixelFrame::from_rows(test_rows()), pixel_frame);
    }

    #[test]
    fn pixel_frame_is_created_from_columns_of_pixel_color() {
        let pixel_frame = PixelFrame::new(PIXEL_FRAME);
        assert_eq!(PixelFrame::from_columns(test_columns()), pixel_frame);
    }
}
