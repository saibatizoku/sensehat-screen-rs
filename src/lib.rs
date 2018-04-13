//! A Rust library for the Raspberry Pi Sense HAT LED Screen
//! ========================================================
//!
//! The [Raspberry Pi Sense HAT](https://www.raspberrypi.org/products/sense-hat/) has an 8×8 RGB LED matrix that provides its own driver for the Linux framebuffer.
//!
//! This library provides a thread-safe, strong-typed, high-level API for the LED matrix, treating
//! it as you would any other screen on a Linux box.
//!
//!
//! # Example
//!
//! The following program shows how to:
//!
//! * Open the framebuffer file-descriptor for the LED matrix screen (`screen`)
//! * Define a pixel color (`red_pixel`)
//! * Define a vector of each pixel color that makes up the screen (`all_64_pixels`)
//! * Turn that vector into a valid screen frame
//! * Show the frame on the screen
//!
//! ```norun
//! extern crate sensehat_screen;
//!
//! use sensehat_screen::{FrameLine, PixelColor, Screen};
//!
//! fn main() {
//!     let mut screen = Screen::open("/dev/fb1").expect("Could not open the framebuffer for the screen");
//!
//!     let red_pixel = PixelColor::new(255, 0, 0); // rgb colors are in the range of 0 <= c < 256.
//!
//!     let all_64_pixels = vec![red_pixel; 64];   // A single vector of 8 x 8 = 64 pixel colors (rows are grouped by chunks of 8)
//!
//!     let all_red_screen = FrameLine::from_pixels(&all_64_pixels); // a screen frame
//!
//!     screen.write_frame(&all_red_screen); // show the frame on the LED matrix
//! }
//! ```
//!
//!
//! # Features
//!
//! `default`
//! ---------
//! By default, the `linux-framebuffer`, `fonts`, and `serde-support` features are included.
//!
//! `linux-framebuffer`
//! -------------------
//! Use the Linux framebuffer to write to the LED matrix.
//!
//! `fonts`
//! -------
//! A collection of legacy 8x8 fonts, renderable on the LED matrix.
//!
//! `serde-support`
//! ---------------
//! Enables support for serialization/deserialization with `serde`.
//!
#[cfg(feature = "fonts")]
extern crate font8x8;
#[cfg(feature = "linux-framebuffer")]
extern crate framebuffer;
extern crate glob;
#[cfg(feature = "serde-support")]
extern crate serde;
#[cfg(feature = "serde-support")]
#[macro_use]
extern crate serde_derive;

// RGB color with RGB565 support
pub mod color;
// 8x8 fonts
#[cfg(feature = "fonts")]
pub mod fonts;
#[cfg(feature = "linux-framebuffer")]
#[path = "framebuffer.rs"]
pub mod screen;

pub use self::color::PixelColor;
#[cfg(feature = "fonts")]
pub use self::fonts::{FontCollection, FontString};
#[cfg(feature = "linux-framebuffer")]
pub use self::screen::Screen;

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

    #[cfg(any(feature = "little-endian", feature = "big-endian"))]
    /// Create a new `FrameLine` instance, given a slice of `PixelColor`.
    pub fn from_pixels(pixels: &[PixelColor]) -> Self {
        pixels
            .iter()
            .fold(FrameLine::new(), |frame, px| frame.extend(px))
    }

    #[cfg(any(feature = "little-endian", feature = "big-endian"))]
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

/// Render a font symbol with a `PixelColor` into a `FrameLine`.
#[cfg(feature = "fonts")]
pub fn font_to_frame(symbol: &[u8; 8], color: PixelColor) -> FrameLine {
    let pixels: Vec<PixelColor> = symbol.iter().fold(Vec::new(), |mut px, x| {
        for bit in 0..8 {
            match *x & 1 << bit {
                0 => px.push(PixelColor::BLACK),
                _ => px.push(color),
            }
        }
        px
    });
    FrameLine::from_pixels(&pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_line_is_created_from_slice_of_bytes() {
        let green: [u8; 8] = [0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07];
        let frame_line = FrameLine::from_slice(&green);
        assert_eq!(frame_line.as_slice(), &green);
    }

    #[cfg(feature = "little-endian")]
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
}
