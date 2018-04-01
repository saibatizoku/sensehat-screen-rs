#[cfg(feature = "fonts")]
extern crate font8x8;
extern crate framebuffer;
extern crate glob;

// 8x8 fonts
#[cfg(feature = "fonts")]
mod fonts;

use framebuffer::{Framebuffer, FramebufferError};

#[cfg(feature = "fonts")]
pub use self::fonts::*;

const BLACK: PixelColor = PixelColor {
    red: 0,
    green: 0,
    blue: 0,
};

/// A single LED pixel color, with RGB565 rendering.
#[derive(Debug, Default, PartialEq)]
pub struct PixelColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl PixelColor {
    /// Create a new LED pixel color.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Create a new LED pixel color from a pair of RGB565-encoded bytes.
    pub fn from_rgb565(color: [u8; 2]) -> Self {
        let red = ((color[1] >> 3) & 0x1F) << 3;
        let green = (color[1] & 0b0000_0111) << 5 | (color[0] & 0b1110_0000) >> 3;
        let blue = (color[0] & 0b0001_1111) << 3;
        PixelColor::new(red, green, blue)
    }

    /// Encodes the current LED pixel color into a pair of RGB565-encoded bytes.
    pub fn rgb565(&self) -> [u8; 2] {
        let r = u16::from((self.red >> 3) & 0x1F);
        let g = u16::from((self.green >> 2) & 0x3F);
        let b = u16::from((self.blue >> 3) & 0x1F);
        let rgb = (r << 11) + (g << 5) + b;
        let lsb = (rgb & 0x00FF) as u8;
        let msb = (rgb.swap_bytes() & 0x00FF) as u8;
        [lsb, msb]
    }
}

/// A single frame on the screen.
/// Defaults to an inner capacity for 128 bytes, suitable for the 8x8 pixel screen.
#[derive(Debug)]
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

    /// Create a new `FrameLine` instance, given a slice of `&PixelColor`.
    pub fn from_pixels(pixels: &[&PixelColor]) -> Self {
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

/// Framebuffered 8x8 LED screen.
#[derive(Debug)]
pub struct Screen {
    framebuffer: Framebuffer,
}

impl Screen {
    /// Open the framebuffer to the screen at the given file-system path.
    pub fn open(path: &str) -> Result<Self, FramebufferError> {
        let framebuffer = Framebuffer::new(path)?;
        Ok(Screen { framebuffer })
    }

    /// Write the contents of a `FrameLine` into the framebuffer. This will
    /// render the frameline on the screen.
    pub fn write_frame(&mut self, frame: &FrameLine) {
        self.framebuffer.write_frame(frame.as_slice());
    }
}

/// Render a font symbol with a `PixelColor` into a `FrameLine`.
#[cfg(feature = "fonts")]
pub fn font_to_frame(symbol: &[u8; 8], color: &PixelColor) -> FrameLine {
    let pixels: Vec<&PixelColor> = symbol.iter().fold(Vec::new(), |mut px, x| {
        for bit in 0..8 {
            match *x & 1 << bit {
                0 => px.push(&BLACK),
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
    fn color_pixel_encodes_rgb_into_2_bytes_rgb565_with_losses() {
        // black 5-bit, 6-bit, 5-bit resolution
        assert_eq!(
            PixelColor::from_rgb565([0x00, 0x00]),
            PixelColor::new(0x00, 0x00, 0x00)
        );
        // white 5-bit, 6-bit, 5-bit resolution
        assert_eq!(
            PixelColor::from_rgb565([0xFF, 0xFF]),
            PixelColor::new(0xF8, 0xFC, 0xF8)
        );
        // 100% green - 6-bit resolution
        assert_eq!(
            PixelColor::from_rgb565([0xE0, 0x07]),
            PixelColor::new(0x00, 0xFC, 0x00)
        );
    }

    // TODO: x86 linux stores `WORDS` as little-endian, meaning the low-byte is
    // sent before the high-byte. Change dealing with [u8; 2] and replace with a
    // single u16.
    #[test]
    fn color_pixel_converts_rgb_into_2_bytes_rgb565() {
        let white_pixel = PixelColor::new(0xFF, 0xFF, 0xFF);
        assert_eq!(white_pixel.rgb565(), [0xFF, 0xFF]);

        let red_pixel = PixelColor::new(0xFF, 0x00, 0x00);
        assert_eq!(red_pixel.rgb565(), [0x00, 0xF8]);

        let green_pixel = PixelColor::new(0x00, 0xFF, 0x00);
        assert_eq!(green_pixel.rgb565(), [0xE0, 0x07]);

        let blue_pixel = PixelColor::new(0x00, 0x00, 0xFF);
        assert_eq!(blue_pixel.rgb565(), [0x1F, 0x00]);
    }

    #[test]
    fn frame_line_is_created_from_slice_of_bytes() {
        let green: [u8; 8] = [0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07, 0xE0, 0x07];
        let frame_line = FrameLine::from_slice(&green);
        assert_eq!(frame_line.as_slice(), &green);
    }

    #[test]
    fn frame_line_is_created_from_slice_of_pixel_color_reference() {
        let blue = PixelColor::from_rgb565([0x1F, 0x00]);
        let frame_line = FrameLine::from_pixels(&[&blue, &blue]);
        assert_eq!(frame_line.as_slice(), &[0x1F, 0x00, 0x1F, 0x00]);
    }
}
