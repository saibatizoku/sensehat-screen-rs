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
