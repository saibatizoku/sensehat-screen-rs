#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "little-endian")]
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

    #[cfg(feature = "big-endian")]
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

    #[cfg(feature = "little-endian")]
    #[test]
    fn convert_rgb565_to_byte_array() {
        let bytes = [0xFF, 0xFF];
        assert_eq!(Rgb565::from(bytes), Rgb565(0xFFFF));
        let bytes = [0xE0, 0x07];
        assert_eq!(Rgb565::from(bytes), Rgb565(0x07E0));
        let bytes = [0x1F, 0x00];
        assert_eq!(Rgb565::from(bytes), Rgb565(0x001F));
    }

    #[cfg(feature = "big-endian")]
    #[test]
    fn convert_rgb565_to_byte_array() {
        let bytes = [0xFF, 0xFF];
        assert_eq!(Rgb565::from(bytes), Rgb565(0xFFFF));
        let bytes = [0x07, 0xE0];
        assert_eq!(Rgb565::from(bytes), Rgb565(0x07E0));
        let bytes = [0x00, 0x1F];
        assert_eq!(Rgb565::from(bytes), Rgb565(0x001F));
    }

    #[cfg(feature = "little-endian")]
    #[test]
    fn convert_byte_array_to_rgb565() {
        let rgb: [u8; 2] = Rgb565(0x07E0).into();
        assert_eq!(rgb, [0xE0, 0x07]);
    }

    #[cfg(feature = "big-endian")]
    #[test]
    fn convert_byte_array_to_rgb565() {
        let rgb: [u8; 2] = Rgb565(0x07E0).into();
        assert_eq!(rgb, [0x07, 0xE0]);
    }

    #[cfg(feature = "little-endian")]
    #[test]
    fn color_pixel_converts_rgb_into_2_bytes_rgb565() {
        let white_pixel = PixelColor::WHITE;
        assert_eq!(white_pixel.rgb565(), [0xFF, 0xFF]);

        let red_pixel = PixelColor::RED;
        assert_eq!(red_pixel.rgb565(), [0x00, 0xF8]);

        let green_pixel = PixelColor::GREEN;
        assert_eq!(green_pixel.rgb565(), [0xE0, 0x07]);

        let blue_pixel = PixelColor::BLUE;
        assert_eq!(blue_pixel.rgb565(), [0x1F, 0x00]);
    }

    #[cfg(feature = "big-endian")]
    #[test]
    fn color_pixel_converts_rgb_into_2_bytes_rgb565() {
        let white_pixel = PixelColor::WHITE;
        assert_eq!(white_pixel.rgb565(), [0xFF, 0xFF]);

        let red_pixel = PixelColor::RED;
        assert_eq!(red_pixel.rgb565(), [0xF8, 0x00]);

        let green_pixel = PixelColor::GREEN;
        assert_eq!(green_pixel.rgb565(), [0x07, 0xE0]);

        let blue_pixel = PixelColor::BLUE;
        assert_eq!(blue_pixel.rgb565(), [0x00, 0x1F]);
    }
}
