use font8x8::{BASIC_UNICODE, BLOCK_UNICODE, BOX_UNICODE, GREEK_UNICODE, HIRAGANA_UNICODE,
              LATIN_UNICODE};
use std::collections::HashMap;
use std::string::FromUtf16Error;

fn default_hashmap() -> HashMap<u16, [u8; 8]> {
    BASIC_UNICODE
        .iter()
        .cloned()
        .chain(LATIN_UNICODE.iter().cloned())
        .chain(BLOCK_UNICODE.iter().cloned())
        .chain(BOX_UNICODE.iter().cloned())
        .chain(GREEK_UNICODE.iter().cloned())
        .chain(HIRAGANA_UNICODE.iter().cloned())
        .collect()
}

/// A set of font symbols that can be printed on a `Screen`.
pub struct FontCollection(HashMap<u16, [u8; 8]>);

impl FontCollection {
    /// Create a default `FontCollection`, containing the Unicode constants
    /// from the [font8x8](https://github.com/saibatizoku/font8x8-rs) crate, except for
    /// `MISC_UNICODE`, and `SGA_UNICODE` (which are non-standard).
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
        let valid = s.encode_utf16()
            .filter(|c| self.0.contains_key(&c))
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
pub struct FontString(Vec<u16>);

impl FontString {
    pub fn to_string(&self) -> String {
        String::from_utf16(&self.0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
