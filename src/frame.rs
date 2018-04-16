//! Frames for the LED Matrix screen
#[cfg(feature = "rotate")]
#[path = "frame_rotate.rs"]
pub mod rotate;

use super::color::PixelColor;

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

    /// Create a new `FrameLine` instance, given a slice of `PixelColor`.
    pub fn from_pixels(pixels: &[PixelColor]) -> Self {
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

/// A frame of pixels. This is the basic representation for the LED Matrix display.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct PixelFrame(Vec<PixelColor>);

impl PixelFrame {
    /// Create a `FrameLine` representing the current `PixelFrame`.
    pub fn new(pixels: &[PixelColor]) -> Self {
        PixelFrame(pixels.to_vec())
    }
    /// Create a `FrameLine` representing the current `PixelFrame`.
    pub fn frame_line(&self) -> FrameLine {
        FrameLine::from_pixels(self.0.as_slice())
    }
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

    #[cfg(not(feature = "big-endian"))]
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
