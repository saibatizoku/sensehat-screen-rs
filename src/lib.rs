extern crate framebuffer;
extern crate glob;

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
            PixelColor::from_rgb565([0x07, 0xE0]),
            PixelColor::new(0x00, 0xFC, 0x00)
        );
    }

    #[test]
    fn color_pixel_converts_rgb_into_2_bytes_rgb565() {
        let white_pixel = PixelColor::new(0xFF, 0xFF, 0xFF);
        assert_eq!(white_pixel.rgb565(), [0xFF, 0xFF]);

        let red_pixel = PixelColor::new(0xFF, 0x00, 0x00);
        assert_eq!(red_pixel.rgb565(), [0xF8, 0x00]);

        let green_pixel = PixelColor::new(0x00, 0xFF, 0x00);
        assert_eq!(green_pixel.rgb565(), [0x07, 0xE0]);

        let blue_pixel = PixelColor::new(0x00, 0x00, 0xFF);
        assert_eq!(blue_pixel.rgb565(), [0x00, 0x1F]);
    }
}
