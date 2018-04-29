## [Unreleased]

## [0.1.6] 2018-04-22
### Changed
- `default` features now also include: `rotate`, and `offset`.
- Updated features on `README.md`.
- Lots of module refactoring with no change to API.

### Added
- `offset` trait to render `PixelFrame` with offset.
- `PixelFrame::offset` method creates a PixelFrame with the visible parts of the offset original. Features comprehensive testing.
- `frame::Offset` enum wraps an offset from `0..=8` in four directions:
    * `Offset::Left(u8)`
    * `Offset::Right(u8)`
    * `Offset::Top(u8)`
    * `Offset::Bottom(u8)`
- `PixelFrame::as_rows` method. Features comprehensive testing.
- `PixelFrame::as_columns` method. Features comprehensive testing.
- `PixelFrame::from_rows` method. Features comprehensive testing.
- `PixelFrame::from_rows` method. Features comprehensive testing.

## [0.1.5] 2018-04-15
### Changed
- Framebuffer-related code refactored into `sensehat_screen::screen` module.
- `FrameLine`-related code refactored into `sensehat_screen::frame` module.
- Make crate modules public.
- `FrameLine` defaults to little-endian format.
- Update `README.md` and `src/lib.rs` documentation.

### Added
- `Rgb565` type to separate concerns from `PixelColor`, and properly encode/decode RGB565 colors.
- `big-endian` feature to encode Rgb565 colors in big-endian format.
- `Rgb565` implements From<_> and Into<_> for `PixelColor`, `&PixelColor`, `(u8, u8, u8)`, and [u8; 2]`.
- `PixelFrame` type to represent a single LED matrix screen frame.
- `rotate` feature to rotate a `PixelFrame` by steps of 90-degrees.
- `examples/rotating-symbol.rs` to demo rotating font symbols with color.

## [0.1.4] 2018-04-08
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
