//! A Rust library for the Raspberry Pi Sense HAT LED Screen
//! ========================================================
//!
//! The [Raspberry Pi Sense HAT](https://www.raspberrypi.org/products/sense-hat/) has an 8×8 RGB LED matrix that provides its own driver for the Linux framebuffer.
//!
//! This library provides a thread-safe, strong-typed, high-level API for the LED matrix, treating
//! it as you would any other screen on a Linux box.
//!
//! For examples, please check the
//! [examples/](https://github.com/saibatizoku/sensehat-screen-rs/tree/master/examples) folder in the source code.
//!
//! Basics
//! ------
//! * [`Screen`](./screen/struct.Screen.html) is the representation of the LED Matrix, it stores a `PixelFrame`.
//!
//! * [`PixelFrame`](./frame/struct.PixelFrame.html) is a collection of 64 `PixelColor`, representing the 8-row by 8-column LED
//! Matrix.
//! * [`PixelColor`](./color/struct.PixelColor.html) is a 24-bit representation of an RGB color, encoded in three bytes.
//!
//! Low-level constructs
//! --------------------
//! * [`Rgb565`](./color/struct.Rgb565.html) is a 16-bit representation of an RGB color, encoded in two bytes. This is the
//! format. `Rgb565` converts into/from `PixelColor`.
//! supported by the LED Matrix's framebuffer device.
//! * [`FrameLine`](./frame/struct.FrameLine.html) is the raw-byte rendering of the `PixelFrame`,
//! properly encoded and ready to be written into the framebuffer device.
//!
//! Frame operations
//! ----------------
//! * [`Rotate`](./frame/rotate/enum.Rotate.html)
//!
//!   Requires `feature = "rotate"`, which is enabled by default.
//!
//!   Rotate the PixelFrame by `Rotate::None`, `Rotate::Ccw90`, `Rotate::Ccw180`, or
//!   `Rotate::Ccw270`, that correspond to a counter-clockwise angle of `0°`, `90°`, `180°`, and `270°`,
//!   respectively.
//!
//! * [`Offset`](./frame/offset/enum.Offset.html)
//!
//!   Requires `feature = "offset"`, which is enabled by default.
//!
//!   Offset the PixelFrame by `Offset::left(n)`, `Offset::right(n)`,
//!   `Offset::bottom(n)`, or `Offset::top(n)`, where `n` is an integer between in the `0..=8` range.
//!
//!   `Offset` with a value of `n = 0`, return a clone of the `PixelFrame`.
//!
//!   `Offset` with a value of `n = 8`, return a `PixelFrame` offset out of view, represented with black pixels (LEDs are off).
//!
//! * [`Clip`](./frame/clip/struct.Clip.html)
//!
//!   Requires `feature = "clip"`, which is enabled by default.
//!
//!   Creates a clip of two `PixelFrame`s, by defining an
//!   `Offset`. See the [clip documentation](./frame/clip/struct.Clip.html) for more details.
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
pub use self::fonts::{font_to_frame, font_to_pixel_frame, FontCollection, FontString};

pub use self::frame::{FrameLine, PixelFrame};
#[cfg(feature = "clip")]
pub use self::frame::clip::Clip;
#[cfg(feature = "offset")]
pub use self::frame::offset::Offset;
#[cfg(feature = "rotate")]
pub use self::frame::rotate::Rotate;

#[cfg(feature = "linux-framebuffer")]
pub use framebuffer::FramebufferError;
#[cfg(feature = "linux-framebuffer")]
pub use self::screen::Screen;
