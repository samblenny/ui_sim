#![no_std]

pub mod fonts;

/// LCD Frame buffer bounds
pub const LCD_WORDS_PER_LINE: usize = 11;
pub const LCD_PX_PER_LINE: usize = 336;
pub const LCD_LINES: usize = 536;
pub const LCD_FRAME_BUF_SIZE: usize = LCD_WORDS_PER_LINE * LCD_LINES;

/// Type for passing frame buffer references
pub type LcdFB = [u32; LCD_FRAME_BUF_SIZE];

/// Text styles
#[derive(Copy, Clone)]
pub enum TStyle {
    BoldLeft,
    RegularCenter,
}

/// Paint a string
pub fn string(fb: &mut LcdFB, x: usize, y: usize, s: &str, style: TStyle) {}

/// Clear a full width screen region bounded by y0..y1
pub fn clear_region(fb: &mut LcdFB, y0: usize, y1: usize) {
    if y1 > LCD_LINES || y0 >= y1 {
        return;
    }
    for y in y0..y1 {
        line_fill_clear(fb, y);
    }
}

/// Outline a full width screen region, bounded by y0..y1, with pad and border box
pub fn outline_region(fb: &mut LcdFB, y0: usize, y1: usize) {
    if y1 > LCD_LINES || y0 + 6 >= y1 {
        return;
    }
    line_fill_clear(fb, y0);
    line_fill_clear(fb, y0 + 1);
    line_fill_padded_solid(fb, y0 + 2);
    for y in y0 + 3..y1 - 3 {
        line_fill_padded_border(fb, y);
    }
    line_fill_padded_solid(fb, y1 - 3);
    line_fill_clear(fb, y1 - 2);
    line_fill_clear(fb, y1 - 1);
}

/// Fill a full width screen region bounded by y0..y1 with a blank keyboard
pub fn blank_keyboard(fb: &mut LcdFB, y0: usize, y1: usize) {}

/// Clear a line of the screen
fn line_fill_clear(fb: &mut LcdFB, y: usize) {
    if y >= LCD_LINES {
        return;
    }
    let base = y * LCD_WORDS_PER_LINE;
    for i in 0..=9 {
        fb[base + i] = 0xffff_ffff;
    }
    fb[base + 10] = 0xffff_0000;
}

/// Fill a line of the screen with black, padded with clear to left and right
fn line_fill_padded_solid(fb: &mut LcdFB, y: usize) {
    if y >= LCD_LINES {
        return;
    }
    let base = y * LCD_WORDS_PER_LINE;
    fb[base] = 0xc000_0000;
    for i in 1..=9 {
        fb[base + i] = 0x0000_0000;
    }
    fb[base + 10] = 0x0003_0000;
}

/// Fill a line of the screen with clear, bordered by black, padded with clear
fn line_fill_padded_border(fb: &mut LcdFB, y: usize) {
    if y >= LCD_LINES {
        return;
    }
    let base = y * LCD_WORDS_PER_LINE;
    fb[base] = 0xdfff_ffff;
    for i in 1..=9 {
        fb[base + i] = 0xffff_ffff;
    }
    fb[base + 10] = 0xfffb_0000;
}

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
