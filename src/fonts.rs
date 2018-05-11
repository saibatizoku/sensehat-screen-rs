//! 8x8 font collection
use super::{FrameLine, PixelColor, PixelFrame};

use font8x8::{FontUtf16, Utf16Fonts, BASIC_FONTS, BLOCK_FONTS, BOX_FONTS, GREEK_FONTS,
              HIRAGANA_FONTS, LATIN_FONTS};
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
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
pub struct FontCollection(HashMap<u16, [u8; 8]>);

impl FontCollection {
    /// Create a default `FontCollection`, containing the Unicode constants
    /// from the [font8x8](https://github.com/saibatizoku/font8x8-rs) crate, except for
    /// `MISC_FONTS`, and `SGA_FONTS` (which are non-standard).
    pub fn new() -> Self {
        FontCollection(default_hashmap())
    }

    /// Create a `FontCollection` with a custom HashMap of font symbols.
    pub fn from_hashmap(hashmap: HashMap<u16, [u8; 8]>) -> Self {
        FontCollection(hashmap)
    }

    /// Get an `Option` with the symbol's byte rendering.
    pub fn get(&self, symbol: u16) -> Option<&[u8; 8]> {
        self.0.get(&symbol)
    }

    /// Search if collection has a symbol by its unicode key.
    pub fn contains_key(&self, symbol: u16) -> bool {
        self.0.contains_key(&symbol)
    }

    /// Sanitize a `&str` and create a new `FontString`.
    pub fn sanitize_str(&self, s: &str) -> Result<FontString, FromUtf16Error> {
        let valid = s.encode_utf16().filter(|c| self.0.contains_key(c))
                     .collect::<Vec<u16>>();
        Ok(FontString(valid))
    }
}

impl Default for FontCollection {
    fn default() -> Self {
        FontCollection::new()
    }
}

/// A string of font symbols valid for rendering. `FontString` instances can only be created by a `FontCollection` instance.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FontString(Vec<u16>);

impl FontString {
    /// Render the font string as unicode slice, `&[u16]`.
    pub fn to_slice(&self) -> &[u16] {
        &self.0
    }

    /// Render the font string as a `String`.
    pub fn to_string(&self) -> String {
        String::from_utf16(&self.0).unwrap()
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

// Render a font symbol with a `PixelColor` into a `[PixelColor; 64]`.
fn font_to_pixel_color_array(symbol: &[u8; 8], color: PixelColor) -> [PixelColor; 64] {
    let mut pixels = [PixelColor::default(); 64];
    for (row_idx, encoded_row) in symbol.iter().enumerate() {
        for col_idx in 0..8 {
            if (*encoded_row & 1 << col_idx) > 0 {
                pixels[row_idx * 8 + col_idx] = color;
            }
        }
    }
    pixels
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
    fn font_to_pixel_color_array_with_bg_creates_new_array() {
        let font_set = FontCollection::new();
        let font = font_set.get('┶' as u16).unwrap();
        let px_array = font_to_pixel_color_array_with_bg(&font.byte_array(), PixelColor::BLUE, PixelColor::YELLOW);
        for (idx, px) in px_array.into_iter().enumerate() {
            assert_eq!(*px, BOX_FONT_BG[idx]);
        }
    }

    #[test]
    fn font_to_pixel_color_array_creates_new_array() {
        let font_set = FontCollection::new();
        let font = font_set.get('M' as u16).unwrap();
        let px_array = font_to_pixel_color_array(&font, PixelColor::BLUE);
        for (idx, px) in px_array.into_iter().enumerate() {
            assert_eq!(*px, BASIC_FONT[idx]);
        }
    }

    #[test]
    fn font_to_pixel_frame_fn_takes_a_byte_array_and_pixel_color() {
        let font_set = FontCollection::new();
        let chi_font = font_set.get('ち' as u16).unwrap();
        let px_frame = font_to_pixel_frame(&chi_font, PixelColor::RED);
        assert_eq!(px_frame, PixelFrame::from(HIRAGANA_FONT));
    }

    #[test]
    fn font_to_frame_fn_takes_a_byte_array_and_pixel_color() {
        let font_set = FontCollection::new();
        let box_font = font_set.get('┶' as u16).unwrap();
        let px_frame_line = font_to_frame(&box_font, PixelColor::GREEN);
        assert_eq!(px_frame_line, PixelFrame::from(BOX_FONT).frame_line());
    }
}
