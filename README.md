A Rust library for the Raspberry Pi Sense HAT LED Screen
========================================================

[![crates.io](https://img.shields.io/crates/v/sensehat-screen.svg)](https://crates.io/crates/sensehat-screen)
[![docs](https://docs.rs/sensehat-screen/badge.svg)](https://docs.rs/sensehat-screen)


The [Raspberry Pi Sense HAT](https://www.raspberrypi.org/products/sense-hat/) has an 8×8 RGB LED matrix that provides its own driver for the Linux framebuffer.

This library provides a thread-safe, strong-typed, high-level API for the LED matrix, treating it as you would any other screen on a Linux box.

# Usage

To use this crate with the default features, add this to your `Cargo.toml`:
```cargo
[dependencies]
sensehat-screen = "0.1"
```

or, to manually specify the features::

```cargo
[dependencies]
sensehat-screen = { version = "0.1", default-features = false, features = ["fonts"] }
```

Then you can use it with your crate:

```rust
extern crate sensehat_screen

use sensehat_screen::{FontCollection, FrameLine, PixelColor, Screen};
```

# Example

The following program shows how to:

* Open the framebuffer file-descriptor for the LED matrix screen (`screen`)
* Define a pixel color (`red_pixel`)
* Define a vector of each pixel color that makes up the screen (`all_64_pixels`)
* Turn that vector into a valid screen frame
* Show the frame on the screen

```
extern crate sensehat_screen;

use sensehat_screen::{FrameLine, PixelColor, Screen};

fn main() {
    let mut screen = Screen::new("/dev/fb1")
        .expect("Could not open the framebuffer for the screen");

    let red_pixel = PixelColor::new(255, 0, 0); // The pixel color's RGB components are each in the range of 0 <= c < 256.

    let all_64_pixels = vec![&red_pixel; 64];   // A single vector of 8 x 8 = 64 pixel colors (rows are grouped by chunks of 8)

    let all_red_screen = FrameLine::from_pixels(&all_64_pixels); // a screen frame

    screen.write_frame(&all_red_screen); // show the frame on the LED matrix
}
```

# Source Code Examples

* [Blinking RGB shapes](./examples/blink.rs)
* [A letter from ゆにち (Yunichi) to Toño](./examples/letter.rs)


# Features

`default`
---------
By default, the `linux-framebuffer`, `fonts`, and `serde-support` features are included.

`linux-framebuffer`
-------------------
Use the Linux framebuffer to write to the LED matrix.

`fonts`
-------
A collection of legacy 8x8 fonts, renderable on the LED matrix.

`serde-support`
---------------
Enables support for serialization/deserialization with `serde`.

Feature Wish List
=================
* [X] `linux-framebuffer` - Use the Linux framebuffer to write to the LED matrix.
* [X] `fonts` - A collection of legacy 8x8 fonts, renderable on the LED matrix.
* [X] `serde-support` - Enables support for serialization/deserialization with `serde`.
* [ ] `images` - Load 8x8 images to the LED matrix with `image`.
* [ ] `cgmath` - Computer-graphics utilities for manipulating the matrix mathematically.
