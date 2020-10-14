pub mod bold;
pub mod regular;
pub mod small;

/// Strings with Unicode Private Use Area characters for UI Sprites
pub mod pua {
    pub const BATTERY_05: &str = &"\u{E700}";
    pub const BATTERY_25: &str = &"\u{E701}";
    pub const BATTERY_50: &str = &"\u{E702}";
    pub const BATTERY_75: &str = &"\u{E703}";
    pub const BATTERY_99: &str = &"\u{E704}";
    pub const RADIO_3: &str = &"\u{E705}";
    pub const RADIO_2: &str = &"\u{E706}";
    pub const RADIO_1: &str = &"\u{E707}";
    pub const RADIO_0: &str = &"\u{E708}";
    pub const RADIO_OFF: &str = &"\u{E709}";
    pub const SHIFT_ARROW: &str = &"\u{E70A}";
    pub const BACKSPACE_SYMBOL: &str = &"\u{E70B}";
    pub const ENTER_SYMBOL: &str = &"\u{E70C}";
}

/// Holds header data for a font glyph
pub struct GlyphHeader {
    pub w: usize,
    pub h: usize,
    pub y_offset: usize,
}
impl GlyphHeader {
    /// Unpack glyph header of format: (w:u8)<<16 | (h:u8)<<8 | yOffset:u8
    pub fn new(header: u32) -> GlyphHeader {
        let w = ((header << 8) >> 24) as usize;
        let h = ((header << 16) >> 24) as usize;
        let y_offset = ((header << 24) >> 24) as usize;
        GlyphHeader { w, h, y_offset }
    }
}
