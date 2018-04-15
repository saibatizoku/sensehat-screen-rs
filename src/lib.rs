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
//! ```rust,no_run
//! extern crate sensehat_screen;
//!
//! #[cfg(feature = "default")]
//! use sensehat_screen::{FrameLine, PixelColor, Screen};
//!
//! #[cfg(not(feature = "default"))]
//! fn main() {
//!     unimplemented!("This examples needs the 'default' features.");
//! }
//!
//! #[cfg(feature = "default")]
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
#[cfg(feature = "serde-support")]
extern crate serde;
#[cfg(feature = "serde-support")]
#[macro_use]
extern crate serde_derive;

// RGB color with RGB565 support
pub mod color;
// Screen frames
pub mod frame;
// 8x8 fonts
#[cfg(feature = "fonts")]
pub mod fonts;
#[cfg(feature = "linux-framebuffer")]
#[path = "framebuffer.rs"]
pub mod screen;

// Re-exports
pub use self::color::PixelColor;

#[cfg(feature = "fonts")]
pub use self::fonts::{FontCollection, FontString};

pub use self::frame::{FrameLine, PixelFrame};

#[cfg(feature = "linux-framebuffer")]
pub use framebuffer::FramebufferError;
#[cfg(feature = "linux-framebuffer")]
pub use self::screen::Screen;

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
