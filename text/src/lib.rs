#![no_std]

pub mod fonts;

#[cfg(test)]
mod tests {
    use super::fonts;

    #[test]
    fn bold_font_at_sign() {
        let offset = fonts::bold::get_glyph_pattern_offset('@');
        assert_eq!(offset, 197);
        assert_eq!(fonts::bold::DATA[offset], 0x00121008);
    }

    #[test]
    fn regular_font_at_sign() {
        let offset = fonts::regular::get_glyph_pattern_offset('@');
        assert_eq!(offset, 182);
        assert_eq!(fonts::regular::DATA[offset], 0x00101008);
    }

    #[test]
    fn small_font_at_sign() {
        let offset = fonts::small::get_glyph_pattern_offset('@');
        assert_eq!(offset, 143);
        assert_eq!(fonts::small::DATA[offset], 0x000e1006);
    }
}
