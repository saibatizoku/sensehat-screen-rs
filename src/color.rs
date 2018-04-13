//! RGB color for LED pixels, with RGB565 rendering support.
/// A single LED pixel color, with RGB565 rendering.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct PixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl PixelColor {
    pub const BLACK: PixelColor = PixelColor {
        red: 0,
        green: 0,
        blue: 0,
    };

    pub const RED: PixelColor = PixelColor {
        red: 0xFF,
        green: 0,
        blue: 0,
    };

    pub const BLUE: PixelColor = PixelColor {
        red: 0,
        green: 0,
        blue: 0xFF,
    };

    pub const GREEN: PixelColor = PixelColor {
        red: 0,
        green: 0xFF,
        blue: 0,
    };

    pub const WHITE: PixelColor = PixelColor {
        red: 0xFF,
        green: 0xFF,
        blue: 0xFF,
    };

    pub const YELLOW: PixelColor = PixelColor {
        red: 0xFF,
        green: 0xFF,
        blue: 0,
    };

    pub const CYAN: PixelColor = PixelColor {
        red: 0,
        green: 0xFF,
        blue: 0xFF,
    };

    pub const MAGENTA: PixelColor = PixelColor {
        red: 0xFF,
        green: 0,
        blue: 0xFF,
    };

    /// Create a new LED pixel color.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Create a new LED pixel color from a pair of RGB565-encoded bytes.
    pub fn from_rgb565(color: [u8; 2]) -> Self {
        let rgb565: Rgb565 = color.into();
        rgb565.into()
    }

    #[cfg(not(feature = "big-endian"))]
    /// Encodes the current LED pixel color into a pair of RGB565-encoded bytes.
    pub fn rgb565(&self) -> [u8; 2] {
        Rgb565::from(self).split_le()
    }

    #[cfg(feature = "big-endian")]
    /// Encodes the current LED pixel color into a pair of RGB565-encoded bytes.
    pub fn rgb565(&self) -> [u8; 2] {
        Rgb565::from(self).split_be()
    }

    /// Sets the brightness of this colour.
    ///
    /// The `scale` value should be between 0 and 1. Values outside this range
    /// are clamped.
    pub fn dim(self, mut scale: f32) -> PixelColor {
        if scale > 1.0 {
            scale = 1.0;
        }
        if scale < 0.0 {
            scale = 0.0;
        }
        fn scale_byte(b: u8, scale: f32) -> u8 {
            (f32::from(b) * scale) as u8
        }
        PixelColor {
            red: scale_byte(self.red, scale),
            green: scale_byte(self.green, scale),
            blue: scale_byte(self.blue, scale),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Rgb565(u16);

impl Rgb565 {
    fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        let r = u16::from((red >> 3) & 0x1F);
        let g = u16::from((green >> 2) & 0x3F);
        let b = u16::from((blue >> 3) & 0x1F);
        let rgb = (r << 11) + (g << 5) + b;
        Rgb565(rgb)
    }

    fn to_rgb(self) -> (u8, u8, u8) {
        let red = (((self.0 & 0b1111_1000_0000_0000) >> 11) << 3) as u8;
        println!("red: {:02X}", red);
        let green = (((self.0 & 0b0000_0111_1110_0000) >> 5) << 2) as u8;
        println!("green: {:02X}", green);
        let blue = ((self.0 & 0b0000_0000_0001_1111) << 3) as u8;
        println!("blue: {:02X}", blue);
        (red, green, blue)
    }

    #[cfg(not(feature = "big-endian"))]
    fn from_le(bytes: [u8; 2]) -> Self {
        let lo = (bytes[1] as u16) << 8;
        let hi = bytes[0] as u16;
        println!("le bytes: {:02X} {:02X}", bytes[0], bytes[1]);
        println!("lo: {:04X}", lo);
        println!("hi: {:04X}", hi);
        println!("mix: {:04X}", hi | lo);
        Rgb565(hi | lo)
    }

    #[cfg(feature = "big-endian")]
    fn from_be(bytes: [u8; 2]) -> Self {
        let lo = (bytes[0] as u16) << 8;
        let hi = bytes[1] as u16;
        println!("be bytes: {:02X} {:02X}", bytes[0], bytes[1]);
        println!("lo: {:04X}", lo);
        println!("hi: {:04X}", hi);
        println!("mix: {:04X}", hi | lo);
        Rgb565(hi | lo)
    }

    #[cfg(not(feature = "big-endian"))]
    fn split_le(self) -> [u8; 2] {
        let lo = (self.0 & 0x00FF) as u8;
        let hi = (self.0.swap_bytes() & 0x00FF) as u8;
        [lo, hi]
    }

    #[cfg(feature = "big-endian")]
    fn split_be(self) -> [u8; 2] {
        let lo = (self.0 & 0x00FF) as u8;
        let hi = (self.0.swap_bytes() & 0x00FF) as u8;
        [hi, lo]
    }
}

#[cfg(not(feature = "big-endian"))]
impl Into<[u8; 2]> for Rgb565 {
    fn into(self) -> [u8; 2] {
        Rgb565::split_le(self)
    }
}

#[cfg(not(feature = "big-endian"))]
impl From<[u8; 2]> for Rgb565 {
    fn from(bytes: [u8; 2]) -> Self {
        Rgb565::from_le(bytes)
    }
}

#[cfg(feature = "big-endian")]
impl Into<[u8; 2]> for Rgb565 {
    fn into(self) -> [u8; 2] {
        Rgb565::split_be(self)
    }
}

#[cfg(feature = "big-endian")]
impl From<[u8; 2]> for Rgb565 {
    fn from(bytes: [u8; 2]) -> Self {
        Rgb565::from_be(bytes)
    }
}

impl From<(u8, u8, u8)> for Rgb565 {
    fn from(color: (u8, u8, u8)) -> Self {
        Rgb565::from_rgb(color.0, color.1, color.2)
    }
}

impl Into<(u8, u8, u8)> for Rgb565 {
    fn into(self) -> (u8, u8, u8) {
        self.to_rgb()
    }
}

impl From<PixelColor> for Rgb565 {
    fn from(color: PixelColor) -> Self {
        Rgb565::from_rgb(color.red, color.green, color.blue)
    }
}

impl Into<PixelColor> for Rgb565 {
    fn into(self) -> PixelColor {
        let rgb565 = self.to_rgb();
        PixelColor::new(rgb565.0, rgb565.1, rgb565.2)
    }
}

impl<'a> From<&'a PixelColor> for Rgb565 {
    fn from(color: &'a PixelColor) -> Self {
        Rgb565::from_rgb(color.red, color.green, color.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(feature = "big-endian"))]
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

    #[cfg(not(feature = "big-endian"))]
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

    #[cfg(not(feature = "big-endian"))]
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

    #[cfg(not(feature = "big-endian"))]
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
