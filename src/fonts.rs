//! 8x8 font collection
use super::{
    color::{BackgroundColor, StrokeColor}, FrameLine, PixelColor, PixelFrame,
};

use font8x8::{
    FontUtf16, Utf16Fonts, BASIC_FONTS, BLOCK_FONTS, BOX_FONTS, GREEK_FONTS, HIRAGANA_FONTS,
    LATIN_FONTS,
};
use std::collections::HashMap;
use std::string::FromUtf16Error;

fn default_hashmap() -> HashMap<u16, FontUtf16> {
    BASIC_FONTS.to_vec()
               .into_iter()
               .chain(LATIN_FONTS.to_vec().into_iter())
               .chain(BLOCK_FONTS.to_vec().into_iter())
               .chain(BOX_FONTS.to_vec().into_iter())
               .chain(GREEK_FONTS.to_vec().into_iter())
               .chain(HIRAGANA_FONTS.to_vec().into_iter())
               .collect()
}

/// A set of font symbols that can be printed on a `Screen`.
#[derive(Clone, Debug, PartialEq)]
//#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct FontCollection(HashMap<u16, FontUtf16>);

impl FontCollection {
    /// Create a default `FontCollection`, containing the Unicode constants
    /// from the [font8x8](https://github.com/saibatizoku/font8x8-rs) crate, except for
    /// `MISC_FONTS`, and `SGA_FONTS` (which are non-standard).
    pub fn new() -> Self {
        FontCollection(default_hashmap())
    }

    /// Create a `FontCollection` with a custom HashMap of font symbols.
    pub fn from_hashmap(hashmap: HashMap<u16, FontUtf16>) -> Self {
        FontCollection(hashmap)
    }

    /// Get an `Option` with the symbol's byte rendering.
    pub fn get(&self, symbol: u16) -> Option<&FontUtf16> {
        self.0.get(&symbol)
    }

    /// Search if collection has a symbol by its unicode key.
    pub fn contains_key(&self, symbol: u16) -> bool {
        self.0.contains_key(&symbol)
    }

    /// Sanitize a `&str` and create a new `FontString`.
    pub fn sanitize_str(&self, s: &str) -> Result<FontString, FromUtf16Error> {
        let valid = s.encode_utf16()
                     .filter(|c| self.0.contains_key(c))
                     .map(|sym| *self.get(sym).unwrap())
                     .collect::<Vec<FontUtf16>>();
        Ok(FontString(valid))
    }
}

impl Default for FontCollection {
    fn default() -> Self {
        FontCollection::new()
    }
}

/// A `FontString` is a collection of `FontUtf16` which can be rendered to frames for the LED
/// Matrix.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontString(Vec<FontUtf16>);

impl FontString {
    /// Create an empty `FontString`.
    pub fn new() -> Self {
        FontString(Default::default())
    }

    /// Render the font string as a collection of unicode value points, `Vec<u16>`.
    pub fn encode_utf16(&self) -> Vec<u16> {
        self.0.iter().map(|font| font.utf16()).collect::<Vec<u16>>()
    }

    /// Render the font string as a `String`.
    pub fn to_string(&self) -> String {
        String::from_utf16(&self.encode_utf16()).unwrap()
    }
}

/// A font that can be rendered as a `PixelFrame` with a `stroke` color, and a `background` color.
#[derive(Debug, PartialEq)]
pub struct FontFrame {
    /// `UTF16` font
    font: FontUtf16,
    /// Color for the font stroke
    stroke: PixelColor,
    /// Color for the font background
    background: PixelColor,
}

impl FontFrame {
    /// Create a new font frame with a `stroke` color, and a `background` color.
    pub fn new(font: FontUtf16, stroke: PixelColor, background: PixelColor) -> Self {
        FontFrame { font,
                    stroke,
                    background, }
    }

    /// The `PixelFrame` for this font.
    pub fn pixel_frame(&self) -> PixelFrame {
        let pixels = font_to_pixel_color_array_with_bg(&self.font.byte_array(),
                                                       self.stroke,
                                                       self.background);
        pixels.into()
    }
}

impl From<FontFrame> for PixelFrame {
    fn from(font: FontFrame) -> Self {
        font.pixel_frame()
    }
}

impl BackgroundColor for FontFrame {
    fn set_background_color(&mut self, color: PixelColor) {
        self.background = color;
    }
    fn get_background_color(&self) -> PixelColor {
        self.background
    }
}

impl StrokeColor for FontFrame {
    fn set_stroke_color(&mut self, color: PixelColor) {
        self.stroke = color;
    }
    fn get_stroke_color(&self) -> PixelColor {
        self.stroke
    }
}

/// Display the contents of a `FontCollection` on `stdout`.
pub fn print_collection(collection: &FontCollection) {
    for key in collection.0.keys() {
        println!("'\\u{{{:04x}}}' {:?}",
                 key,
                 String::from_utf16_lossy(&[*key]));
    }
}

// Render a font symbol with a stroke color and a background color.
fn font_to_pixel_color_array_with_bg(symbol: &[u8; 8],
                                     color: PixelColor,
                                     background: PixelColor)
                                     -> [PixelColor; 64] {
    let mut pixels = [background; 64];
    for (row_idx, encoded_row) in symbol.iter().enumerate() {
        for col_idx in 0..8 {
            if (*encoded_row & 1 << col_idx) > 0 {
                pixels[row_idx * 8 + col_idx] = color;
            }
        }
    }
    pixels
}

// Render a font symbol with a `PixelColor` into a `[PixelColor; 64]`.
fn font_to_pixel_color_array(symbol: &[u8; 8], color: PixelColor) -> [PixelColor; 64] {
    font_to_pixel_color_array_with_bg(symbol, color, Default::default())
}

/// Render a font symbol with a `PixelColor` into a `FrameLine`.
pub fn font_to_pixel_frame(symbol: &[u8; 8], color: PixelColor) -> PixelFrame {
    let pixels = font_to_pixel_color_array(symbol, color);
    PixelFrame::new(&pixels)
}

/// Render a font symbol with a `PixelColor` into a `FrameLine`.
pub fn font_to_frame(symbol: &[u8; 8], color: PixelColor) -> FrameLine {
    let pixels = font_to_pixel_color_array(symbol, color);
    FrameLine::from_pixels(&pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLK: PixelColor = PixelColor::BLACK;
    const RED: PixelColor = PixelColor::RED;
    const GRN: PixelColor = PixelColor::GREEN;
    const BLU: PixelColor = PixelColor::BLUE;
    const YLW: PixelColor = PixelColor::YELLOW;
    const BASIC_FONT: [PixelColor; 64] = [ BLU, BLU, BLK, BLK, BLK, BLU, BLU, BLK, //
                                           BLU, BLU, BLU, BLK, BLU, BLU, BLU, BLK, //
                                           BLU, BLU, BLU, BLU, BLU, BLU, BLU, BLK, //
                                           BLU, BLU, BLU, BLU, BLU, BLU, BLU, BLK, //
                                           BLU, BLU, BLK, BLU, BLK, BLU, BLU, BLK, //
                                           BLU, BLU, BLK, BLK, BLK, BLU, BLU, BLK, //
                                           BLU, BLU, BLK, BLK, BLK, BLU, BLU, BLK, //
                                           BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, //
                                          ];
    const BOX_FONT: [PixelColor; 64] = [ BLK, BLK, BLK, GRN, BLK, BLK, BLK, BLK, //
                                         BLK, BLK, BLK, GRN, BLK, BLK, BLK, BLK, //
                                         BLK, BLK, BLK, GRN, BLK, BLK, BLK, BLK, //
                                         BLK, BLK, BLK, GRN, GRN, GRN, GRN, GRN, //
                                         GRN, GRN, GRN, GRN, GRN, GRN, GRN, GRN, //
                                         BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, //
                                         BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, //
                                         BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, //
                                        ];
    const BOX_FONT_BG: [PixelColor; 64] = [ YLW, YLW, YLW, BLU, YLW, YLW, YLW, YLW, //
                                            YLW, YLW, YLW, BLU, YLW, YLW, YLW, YLW, //
                                            YLW, YLW, YLW, BLU, YLW, YLW, YLW, YLW, //
                                            YLW, YLW, YLW, BLU, BLU, BLU, BLU, BLU, //
                                            BLU, BLU, BLU, BLU, BLU, BLU, BLU, BLU, //
                                            YLW, YLW, YLW, YLW, YLW, YLW, YLW, YLW, //
                                            YLW, YLW, YLW, YLW, YLW, YLW, YLW, YLW, //
                                            YLW, YLW, YLW, YLW, YLW, YLW, YLW, YLW, //
                                           ];
    const HIRAGANA_FONT: [PixelColor; 64] = [ BLK, BLK, BLK, RED, BLK, BLK, BLK, BLK, //
                                              BLK, RED, RED, RED, RED, RED, RED, BLK, //
                                              BLK, BLK, BLK, RED, BLK, BLK, BLK, BLK, //
                                              BLK, BLK, RED, RED, RED, RED, BLK, BLK, //
                                              BLK, BLK, BLK, BLK, BLK, BLK, RED, BLK, //
                                              BLK, BLK, BLK, BLK, BLK, BLK, RED, BLK, //
                                              BLK, BLK, BLK, RED, RED, RED, BLK, BLK, //
                                              BLK, BLK, BLK, BLK, BLK, BLK, BLK, BLK, //
                                           ];

    #[test]
    fn font_collection_sanitizes_text_by_filtering_known_unicode_points() {
        let font_set = FontCollection::new();
        let valid_text = font_set.sanitize_str("hola niño @¶øþ¥").unwrap();
        assert_eq!(valid_text.to_string(), "hola niño @¶øþ¥");
    }

    #[test]
    fn font_collection_sanitizes_text_by_removing_symbols_not_in_set() {
        let font_set = FontCollection::new();
        let invalid_text = font_set.sanitize_str("ŧ←→ł").unwrap();
        assert_eq!(invalid_text.to_string(), "");

        let font_set = FontCollection::from_hashmap(HashMap::new());
        let invalid_text = font_set.sanitize_str("hola niño @¶øþ¥").unwrap();
        assert_eq!(invalid_text.to_string(), "");
    }

    #[test]
    fn font_collection_gets_optional_symbol_by_unicode_key() {
        let font_set = FontCollection::new();
        let symbol = font_set.get('ñ' as u16);
        assert!(symbol.is_some());
    }

    #[test]
    fn font_collection_searches_for_symbols_by_unicode_key() {
        let font_set = FontCollection::new();
        let has_symbol = font_set.contains_key('ñ' as u16);
        assert!(has_symbol);
    }

    #[test]
    fn font_string_new_method_starts_emtpy_instance() {
        let font_string = FontString::new();
        assert_eq!(font_string.0, Vec::new());
    }

    #[test]
    fn font_string_encode_utf16_method_returns_vec_of_u16() {
        let font_set = FontCollection::new();
        let font_string = font_set.sanitize_str("┷│││┯").unwrap();
        assert_eq!(font_string.encode_utf16(),
                   vec![0x2537, 0x2502, 0x2502, 0x2502, 0x252F]);
    }

    #[test]
    fn font_string_to_string_method_returns_string() {
        let font_set = FontCollection::new();
        let font_string = font_set.sanitize_str("┷│││┯").unwrap();
        assert_eq!(font_string.to_string(), "┷│││┯".to_string());
    }

    #[test]
    fn fn_font_to_pixel_color_array_with_bg_creates_new_array() {
        let font_set = FontCollection::new();
        let font = font_set.get('┶' as u16).unwrap();
        let px_array = font_to_pixel_color_array_with_bg(&font.byte_array(),
                                                         PixelColor::BLUE,
                                                         PixelColor::YELLOW);
        for (idx, px) in px_array.into_iter().enumerate() {
            assert_eq!(*px, BOX_FONT_BG[idx]);
        }
    }

    #[test]
    fn fn_font_to_pixel_color_array_creates_new_array() {
        let font_set = FontCollection::new();
        let font = font_set.get('M' as u16).unwrap();
        let px_array = font_to_pixel_color_array(&font.byte_array(), PixelColor::BLUE);
        for (idx, px) in px_array.into_iter().enumerate() {
            assert_eq!(*px, BASIC_FONT[idx]);
        }
    }

    #[test]
    fn fn_font_to_pixel_frame_takes_a_byte_array_and_pixel_color() {
        let font_set = FontCollection::new();
        let chi_font = font_set.get('ち' as u16).unwrap();
        let px_frame = font_to_pixel_frame(&chi_font.byte_array(), PixelColor::RED);
        assert_eq!(px_frame, PixelFrame::from(HIRAGANA_FONT));
    }

    #[test]
    fn fn_font_to_frame_takes_a_byte_array_and_pixel_color() {
        let font_set = FontCollection::new();
        let box_font = font_set.get('┶' as u16).unwrap();
        let px_frame_line = font_to_frame(&box_font.byte_array(), PixelColor::GREEN);
        assert_eq!(px_frame_line, PixelFrame::from(BOX_FONT).frame_line());
    }

    #[test]
    fn font_frames_are_created_from_ut16_font_a_stroke_and_a_background_color() {
        let font_set = FontCollection::new();
        let letter_a = font_set.get('a' as u16).unwrap();
        let font_frame = FontFrame::new(letter_a.clone(), PixelColor::WHITE, PixelColor::BLACK);
        assert_eq!(font_frame,
                   FontFrame { font: *letter_a,
                               stroke: PixelColor::WHITE,
                               background: PixelColor::BLACK });
    }

    #[test]
    fn font_frames_is_represented_as_a_pixel_frame() {
        let font_set = FontCollection::new();
        let hiragana_font = font_set.get('ち' as u16).unwrap();
        let font_frame = FontFrame::new(hiragana_font.clone(), PixelColor::RED, PixelColor::BLACK);
        let px_frame = font_frame.pixel_frame();
        assert_eq!(px_frame, PixelFrame::from(HIRAGANA_FONT));
    }

    #[test]
    fn pixel_frame_implements_from_font_frame_trait() {
        let font_set = FontCollection::new();
        let hiragana_font = font_set.get('ち' as u16).unwrap();
        let font_frame = FontFrame::new(hiragana_font.clone(), PixelColor::RED, PixelColor::BLACK);
        let px_frame = PixelFrame::from(font_frame);
        assert_eq!(px_frame, PixelFrame::from(HIRAGANA_FONT));
    }

    #[test]
    fn font_frame_sets_background_color() {
        let font_set = FontCollection::new();
        let letter_a = font_set.get('a' as u16).unwrap();
        let mut font_frame = FontFrame::new(letter_a.clone(), PixelColor::WHITE, PixelColor::BLACK);
        font_frame.set_background_color(PixelColor::RED);
        assert_eq!(font_frame,
                   FontFrame { font: *letter_a,
                               stroke: PixelColor::WHITE,
                               background: PixelColor::RED });
    }

    #[test]
    fn font_frame_gets_background_color() {
        let font_set = FontCollection::new();
        let letter_a = font_set.get('a' as u16).unwrap();
        let font_frame = FontFrame::new(letter_a.clone(), PixelColor::WHITE, PixelColor::GREEN);
        assert_eq!(font_frame.get_background_color(), PixelColor::GREEN);
    }

    #[test]
    fn font_frame_sets_stroke_color() {
        let font_set = FontCollection::new();
        let letter_a = font_set.get('a' as u16).unwrap();
        let mut font_frame = FontFrame::new(letter_a.clone(), PixelColor::WHITE, PixelColor::BLACK);
        font_frame.set_stroke_color(PixelColor::YELLOW);
        assert_eq!(font_frame,
                   FontFrame { font: *letter_a,
                               stroke: PixelColor::YELLOW,
                               background: PixelColor::BLACK });
    }

    #[test]
    fn font_frame_gets_stroke_color() {
        let font_set = FontCollection::new();
        let letter_a = font_set.get('a' as u16).unwrap();
        let font_frame = FontFrame::new(letter_a.clone(), PixelColor::BLUE, PixelColor::WHITE);
        assert_eq!(font_frame.get_stroke_color(), PixelColor::BLUE);
    }
}
