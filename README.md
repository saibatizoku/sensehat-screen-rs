A Rust library for the Raspberry Pi Sense HAT LED Screen
========================================================

[![crates.io](https://img.shields.io/crates/v/sensehat-screen.svg)](https://crates.io/crates/sensehat-screen)
[![docs](https://docs.rs/sensehat-screen/badge.svg)](https://docs.rs/sensehat-screen)


The [Raspberry Pi Sense HAT](https://www.raspberrypi.org/products/sense-hat/) has an 8×8 RGB LED matrix that provides its own driver for the Linux framebuffer.

This library provides a thread-safe implementation for using the LED matrix by writing directly to the framebuffer.

# Usage

Add this to your `Cargo.toml`:
```cargo
[dependencies]
sensehat-screen = "0.1"
```

Then you can use it with your crate:

```rust
extern crate sensehat-screen

use sensehat_screen::{FontCollection, FrameLine, PixelColor, Screen};
```

# Examples

* [Blinking RGB shapes](./examples/blink.rs)
* [A letter from ゆにち (Yunichi) to Toño](./examples/letter.rs)


# Features

"default"
---------
By default, the "fonts" feature is included.


"fonts"
-------
A collection of legacy 8x8 fonts, renderable on the LED matrix.

Feature Wish List
=================
* [X] `fonts` - A collection of legacy 8x8 fonts, renderable on the LED matrix.
* [ ] `images` - Load 8x8 images to the LED matrix.
* [ ] `cgmath` - Computer-graphics utilities for manipulating the matrix mathematically.
