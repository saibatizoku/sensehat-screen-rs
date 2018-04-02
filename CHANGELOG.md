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
