//! Frames for the LED Matrix screen
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
pub struct PixelFrame(Vec<PixelColor>);

impl PixelFrame {
    fn rotate_left(self) -> Self {
        // brute-force
        let columns = (0..8).into_iter().map(|col_idx| {
            let col: Vec<PixelColor> = self.0
                .iter()
                .enumerate()
                .filter(|(idx, _)| idx % 8 == col_idx)
                .map(|(_, color)| *color)
                .collect();
            println!("col: {:?}", col.len());
            col
        });

        let transpose: Vec<PixelColor> = columns.flat_map(|col| col).collect();

        let flip_rows: Vec<PixelColor> = transpose
            .chunks(8)
            .rev()
            .flat_map(|row| row.to_vec())
            .collect();

        PixelFrame(flip_rows)
    }
    fn rotate_180(self) -> Self {
        let flip_180: Vec<PixelColor> = self.0.into_iter().rev().collect();
        PixelFrame(flip_180)
    }
    fn rotate_right(self) -> Self {
        let columns = (0..8).into_iter().map(|col_idx| {
            let col: Vec<PixelColor> = self.0
                .iter()
                .enumerate()
                .filter(|(idx, _)| idx % 8 == col_idx)
                .rev()
                .map(|(_, color)| *color)
                .collect();

            col
        });

        let flip_right: Vec<PixelColor> = columns.flat_map(|col| col).collect();

        PixelFrame(flip_right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DARK: PixelColor = PixelColor::BLACK;
    const BLUE: PixelColor = PixelColor::BLUE;

    const CHECKER_BASE: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, BLUE, DARK, BLUE, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, BLUE, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, DARK, //
        BLUE, DARK, DARK, DARK, DARK, DARK, BLUE, DARK, //
        BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, //
    ];

    const CHECKER_90_CW: [PixelColor; 64] = [
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, DARK, DARK, DARK, DARK, BLUE, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, BLUE, BLUE, DARK, DARK, DARK, BLUE, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
    ];

    const CHECKER_180: [PixelColor; 64] = [
        DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, BLUE, //
        DARK, BLUE, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, BLUE, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, BLUE, DARK, BLUE, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
    ];

    const CHECKER_90_CCW: [PixelColor; 64] = [
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, BLUE, BLUE, DARK, //
        BLUE, BLUE, DARK, DARK, DARK, DARK, DARK, DARK, //
        BLUE, BLUE, BLUE, BLUE, DARK, DARK, DARK, DARK, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, DARK, //
        DARK, DARK, BLUE, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, DARK, BLUE, //
        DARK, DARK, DARK, DARK, DARK, DARK, BLUE, BLUE, //
    ];

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

    #[test]
    fn pixel_frame_is_rotated_90_degrees_left() {
        let checker_base = PixelFrame(CHECKER_BASE.to_vec());
        let checker_left = PixelFrame(CHECKER_90_CCW.to_vec());
        assert_eq!(checker_base.rotate_left(), checker_left);
    }

    #[test]
    fn pixel_frame_is_rotated_by_180_degrees() {
        let checker_base = PixelFrame(CHECKER_BASE.to_vec());
        let checker_180 = PixelFrame(CHECKER_180.to_vec());
        assert_eq!(checker_base.rotate_180(), checker_180);
    }

    #[test]
    fn pixel_frame_is_rotated_by_180_degrees_by_two_90_deg_steps() {
        let checker_base = PixelFrame(CHECKER_BASE.to_vec());
        let checker_180 = PixelFrame(CHECKER_180.to_vec());
        assert_eq!(checker_base.rotate_left().rotate_left(), checker_180);
    }

    #[test]
    fn pixel_frame_is_rotated_90_degrees_right() {
        let checker_base = PixelFrame(CHECKER_BASE.to_vec());
        let checker_right = PixelFrame(CHECKER_90_CW.to_vec());
        assert_eq!(checker_base.rotate_right(), checker_right);
    }
}
