# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `lazy_static = "1.0"` to Cargo.toml.
- Lazily-initiated static `fonts::FONT_HASHMAP` exported in top-level.
- Lazily-initiated static `fonts::FONT_COLLECTION` exported in top-level.

### Changed
- Update to `font8x8 = 0.2` in Cargo.toml.
- Replace `FontUtf16` with `FontUnicode` type.
- Replace `Utf16Fonts` with `UnicodeFonts` trait.
- Replace `u16` with `char`.
- Replace `FontString::encode_utf16` with `FontString::chars` method (breaking change).
## [v0.1.11] - 2018-06-17
### Fixed
- URLs to `examples/scroll-*.rs` on README.

## [v0.1.10] - 2018-06-17
### Added
- `BackgroundColor` trait defines setter/getter of background color.
- `StrokeColor` trait defines setter/getter of stroke color.
- Unit tests and documentation for:
  * `FontString::new`
  * `FontString::encode_utf16`
  * `FontString::to_string`
- `scroll` trait to join a collection of `PixelFrame`s into a single `Scroll`.
- `Scroll` type to wrap an inner `Vec<PixelFrame>`.
  * `Scroll::new(&[PixelFrame])` method initializes a scroll.
  * `Scroll::clips` method returns a `Vec<Clip>` made from the inner `Vec<PixelFrame>`.
  * `Scroll::frames` method returns a slice of the inner `Vec<PixelFrame>`.
  * `Scroll::reverse` method reverses the order of the inner `Vec<PixelFrame>`.
  * `Scroll::len` method the length of the inner `Vec<PixelFrame>`.
- Tests and implementation for new `FontFrame` type, which is a font with stroke and background colors:
  * `FontFrame::new`
  * `FontFrame::pixel_frame`
- `PixelFrame` implements `From<FontFrame>`.
- `FontFrame` implements `BackgroundColor` trait.
- `FontFrame` implements `StrokeColor` trait.
- New `basic` features are a subset of the current the `default` feature list.
- Add Travis CI integration with `.travis.yml`.
- Badge for Travis CI in README.md
- Add "Requirements" section to README.md.
- Specify Rust stable v1.26 as the minimum version requirement on README.md
- Tests for `Scroll` instances that create `FrameSequence` with a given `FrameDirection`.
  * `Scroll::right_to_left` for `FrameDirection::RightToLeft`.
  * `Scroll::left_to_right` for `FrameDirection::LeftToRight`.
  * `Scroll::top_to_bottom` for `FrameDirection::TopToBottom`.
  * `Scroll::bottom_to_top` for `FrameDirection::BottomToTop`.
- Add "Changelog" section to README.md, linking to this document.
- Add `FrameSequence` type.
- Add `examples/scroll-top-bottom.rs`.
- Add `examples/scroll-bottom-top.rs`.
- Add `examples/scroll-left-right.rs`.
- Add `examples/scroll-right-left.rs`.
- Add descriptions to examples on README.md.
- Add scroll examples to README.md

### Changed
- `FontString` wraps `Vec<font8x8::FontUtf16>` instead of `Vec<u16>`.
- Updated `examples/letter.rs` to use new `FontString::encode_utf16` method.
- Refactored `basic` features in the `default` feature list.
- `sensehat_screen::{Offset, clip_pixel_frames_offset_left, clip_pixel_frames_offset_top}` now have the `#[cfg(any(feature = "offset", feature = "clip"))]` attribute.

### Removed
- `rotate_bytes_left` and `rotate_bytes_right` implementation functions in `src/frame_clip.rs` are no longer needed.
- Duplicate badge for Cargo Docs from `README.md`.

## [v0.1.9] - 2018-05-01
### Added
- "Contribute" section in `CHANGELOG.md` to invite people to contribute to this library.
- `rustfmt.toml`, sets the expected `indent_style` to be `Visual`
- `impl<'a> From<&'a [PixelColor; 64]> for PixelFrame`
- `impl From<[PixelColor; 64]> for PixelFrame`
- `impl Into<[PixelColor; 64]> for PixelFrame`
- `impl Index<usize> for PixelFrame`, to get pixel colors using `let color = pixel_frame[idx]`-style syntax.
- `impl IndexMut<usize> for PixelFrame`, to set pixel colors using `pixel_frame[idx] = color`-style syntax.

### Changed
- Ran `cargo fmt` with `index_style` set to `Visual`

## [0.1.8] - 2018-04-30
### Fixed
- `font_to_frame` and `font_to_pixel_frame` methods were using incorrect indexes.

### Changed
- Update `README.md` section for examples, making it friendlier to read.

## [0.1.7] - 2018-04-29
### Added
- `CHANGELOG.md` has a header, and conforms to [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).
- `clip` feature to combine PixelFrames
- `frame::Clip` struct that combines two PixelFrame and returns a clip of them. Fully documented and tested.
- `PixelFrame::clip` method to create a clip frame on-the-go.
- `PixelFrame::build_clip` method to build a clip, suitable for loops, and when multiple clips of the same frames are needed.
- `examples/text-clip.rs` shows how to scroll two frames, by creating clip and rendering them on the screen.
- `Rotate` enum provided a strongly-typed rotation angle for frames.
- Top-level documentation describes what crate types do in `src/lib.rs`.
- Top-level re-export of `Rotate`, `Offset`, and `Clip`.
- Fine-tuning of frame clipping doctests.
- `PixelFrame::transpose` method to transpose LED Matrix indexes.
- `PixelFrame::flip_h` method to horizontally flip the LED Matrix.
- `PixelFrame::flip_v` method to vertically flip the LED Matrix.
- `PixelFrame::reverse` method to reverse the LED Matrix indexes.

### Changed
- `default` features now also include: `clip`.
- `offset` feature now depends on `clip`.
- refactor font-related code into `src/fonts.rs`.
- rename `PixelColor::from_rgb565` to `PixelColor::from_rgb565_bytes` for clarity.
- replace inner `Vec<u8>` with `[Rgb565; 64]` in `FrameLine`.
- replace inner `Vec<PixelColor>` with `[PixelColor; 64]` in `PixelFrame`.
- rename `FrameLine::as_slice` to `FrameLine::as_bytes`
- `PixelFrame::as_rows` method now returns `&[[PixelColor; 8]; 8]`.
- `PixelFrame::as_columns` method now returns `&[[PixelColor; 8]; 8]`.
- `PixelFrame::from_rows` method now takes `[[PixelColor; 8]; 8]`.
- `PixelFrame::from_columns` method now takes `[[PixelColor; 8]; 8]`.
- Optimize `rotate`, `offset`, and `clip`  methods with slice methods and iterators.

## [0.1.6] - 2018-04-22
### Added
- `offset` trait to render `PixelFrame` with offset.
- `PixelFrame::offset` method creates a PixelFrame with the visible parts of the offset original. Features comprehensive testing.
- `frame::Offset` enum wraps an offset from `0..=8` in four directions:
    * `Offset::Left(_)`
    * `Offset::Right(_)`
    * `Offset::Top(_)`
    * `Offset::Bottom(_)`

  which may only be created by the associated methods:

    * `Offset::left(offset: u8)`
    * `Offset::right(offset: u8)`
    * `Offset::top(offset: u8)`
    * `Offset::bottom(offset: u8)`

  all with comprehensive testing.

- `PixelFrame::as_rows` method. Features comprehensive testing.
- `PixelFrame::as_columns` method. Features comprehensive testing.
- `PixelFrame::from_rows` method. Features comprehensive testing.
- `PixelFrame::from_columns` method. Features comprehensive testing.

### Changed
- `default` features now also include: `rotate`, and `offset`.
- Updated features on `README.md`.
- Lots of module refactoring with no change to API.

## [0.1.5] - 2018-04-15
### Added
- `Rgb565` type to separate concerns from `PixelColor`, and properly encode/decode RGB565 colors.
- `big-endian` feature to encode Rgb565 colors in big-endian format.
- `Rgb565` implements From<_> and Into<_> for `PixelColor`, `&PixelColor`, `(u8, u8, u8)`, and [u8; 2]`.
- `PixelFrame` type to represent a single LED matrix screen frame.
- `rotate` feature to rotate a `PixelFrame` by steps of 90-degrees.
- `examples/rotating-symbol.rs` to demo rotating font symbols with color.

### Changed
- Framebuffer-related code refactored into `sensehat_screen::screen` module.
- `FrameLine`-related code refactored into `sensehat_screen::frame` module.
- Make crate modules public.
- `FrameLine` defaults to little-endian format.
- Update `README.md` and `src/lib.rs` documentation.

## [0.1.4] - 2018-04-08
### Added
- Implement `Copy` trait for `PixelColor`.
- `PixelColor::dim` method to dim the color by a factor between 0 and 1.
- Color constants:

    * `RED`
    * `BLUE`
    * `MAGENTA`
    * `BLACK`
    * `RED`
    * `BLUE`
    * `GREEN`
    * `WHITE`
    * `YELLOW`
    * `CYAN`
    * `MAGENTA`

- Update pixel color usage on tests, examples, and `README.md`
- Speed-up printing letter in `examples/letter.rs`, showing one character every 800ms.

## [0.1.3] - 2018-04-05
### Added
- Doctest with example on `src/lib.rs`.
- Update README with a copy of the `src/lib.rs` doctest example.
- Implement `Debug` and `Default` trait for `FontString`.
- Implement `PartialEq` trait for `PixelColor`, `FrameLine`, `FontCollection`, `FontString`.
- Implement `Clone` trait for `PixelColor`, `FrameLine`, `FontCollection`, `FontString`.

### Changed
- Renamed private constant from `BLACK` to `LED_OFF`.

## [0.1.2] - 2018-04-02
### Changed
- updated `README.md` with the newest features

## [0.1.1] - 2018-04-02
### Added
- `serde-support` feature in Cargo.toml to enable `serde` support.
- `linux-framebuffer` feature in Cargo.toml to optionally use the Screen.
- `default` feature includes `serde-support`, and `linux-framebuffer`
- `CHANGELOG.md` for semantic-versioning of changes (this file).

## [0.1.0] - 2018-04-01
### Added
- `PixelColor` type to represent an RGB color, renderable into RGB565.
- `FrameLine` type to write bytes into the `Screen` framebuffer.
- `Screen` type to open the framebuffer and write frames.
- `FontCollection` and `FontString` types, to manipulate fonts compatible with the `Screen`.
- `font_to_frame` convenience function to render a font as a `FrameLine`.
- `print_collection` convenience function to list a `FontCollection` on stdout.
- `fonts` feature in Cargo.toml. Conditional compilation attributes for font types and functions.
- `default` feature includes `fonts`
- `examples/blink.rs` shows how to manually write frames to the screen.
- `examples/letter.rs` shows how to render a sanitized-text on the screen.
