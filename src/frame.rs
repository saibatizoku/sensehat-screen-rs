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

use super::color::{PixelColor, Rgb565};
use std::fmt::{self, Write};

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
        let colors = bytes
            .chunks(2)
            .map(|chunk| Rgb565::from([chunk[0], chunk[1]]))
            .enumerate()
            .fold(
                [Rgb565::default(); 64],
                |mut color_array, (index, color)| {
                    color_array[index] = color;
                    color_array
                },
            );
        FrameLine(colors)
    }

    /// Create a new `FrameLine` instance, given a slice of `PixelColor`.
    pub fn from_pixels(pixels: &[PixelColor; 64]) -> Self {
        let colors = pixels.iter().map(Rgb565::from).enumerate().fold(
            [Rgb565::default(); 64],
            |mut color_array, (index, color)| {
                color_array[index] = color;
                color_array
            },
        );
        FrameLine(colors)
    }

    /// Returns the `FrameLine` as a slice of bytes.
    pub fn as_bytes(&self) -> [u8; 128] {
        self.0
            .iter()
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
            write!(&mut s, "{}", "\n[").unwrap();
            for &px in row {
                let rgbu16: u16 = px.into();
                write!(&mut s, " {:04X}", rgbu16).unwrap();
            }
            write!(&mut s, "{}", " ]").unwrap();
            s
        });
        write!(f, "FrameLine:\n{}", rows)
    }
}

impl PartialEq for FrameLine {
    fn eq(&self, other: &FrameLine) -> bool {
        self.0
            .iter()
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
            write!(&mut s, "{:?}\n", row).unwrap();
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
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
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

    /// Returns a `Vec<Vec<PixelColor>>`, organized by rows, from top to bottom.
    pub fn as_rows(&self) -> [[PixelColor; 8]; 8] {
        self.0.chunks(8).map(|row| {
            row.iter().enumerate().fold([PixelColor::default(); 8], |mut pxrow, (idx, &px)| {
                pxrow[idx] = px;
                pxrow
            })
        }).enumerate().fold([[PixelColor::default(); 8]; 8], |mut rows, (idx, row)| {
            rows[idx] = row;
            rows
        })
    }

    /// Returns a `Vec<Vec<PixelColor>>`, organized by columns, from left to right.
    pub fn as_columns(&self) -> [[PixelColor; 8]; 8] {
        let mut columns: [[PixelColor; 8]; 8] = [[PixelColor::default(); 8]; 8];
        for (idx, px) in self.0.iter().cloned().enumerate() {
            let col_idx = idx % 8;
            let row_idx = idx / 8;
            columns[col_idx][row_idx] = px;
        }
        columns
    }

    /// Create a new `PixelFrame` from a `Vec<Vec<PixelColor>>`, of 8 rows with 8 `PixelColor`s.
    pub fn from_rows(rows: &[[PixelColor; 8]; 8]) -> Self {
        let pixels = rows.into_iter()
            .flat_map(|row| row.into_iter())
            .enumerate()
            .fold([PixelColor::default(); 64], |mut pxs, (idx, &px)| {
                pxs[idx] = px;
                pxs
            });
        PixelFrame(pixels)
    }

    /// Create a new `PixelFrame` from a `Vec<Vec<PixelColor>>`, of 8 columns with 8 `PixelColor`s.
    pub fn from_columns(columns: &[[PixelColor; 8]; 8]) -> Self {
        let mut pixels = [PixelColor::BLACK; 64];
        for (col_idx, col) in columns.into_iter().enumerate() {
            for (row_idx, &px) in col.into_iter().enumerate() {
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
    fn test_rows() -> [[PixelColor; 8]; 8] {
        [[RED, ONE, RED, TWO, RED, ONE, RED, TWO]; 8]
    }
    fn test_columns() -> [[PixelColor; 8]; 8] {
        [
            [RED; 8], [ONE; 8], [RED; 8], [TWO; 8], [RED; 8], [ONE; 8], [RED; 8], [TWO; 8]
        ]
    }

    #[test]
    fn frame_line_is_created_from_slice_of_bytes() {
        let color: [u8; 128] = [0xE0; 128];
        let frame_line = FrameLine::from_slice(&color);
        frame_line
            .as_bytes()
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
        pixel_frame
            .0
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
        assert_eq!(PixelFrame::from_rows(&test_rows()), pixel_frame);
    }

    #[test]
    fn pixel_frame_is_created_from_columns_of_pixel_color() {
        let pixel_frame = PixelFrame::new(PIXEL_FRAME);
        assert_eq!(PixelFrame::from_columns(&test_columns()), pixel_frame);
    }
}
