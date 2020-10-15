#![no_std]

pub mod fonts;
use fonts::{Font, GlyphHeader};

/// LCD Frame buffer bounds
pub const LCD_WORDS_PER_LINE: usize = 11;
pub const LCD_PX_PER_LINE: usize = 336;
pub const LCD_LINES: usize = 536;
pub const LCD_FRAME_BUF_SIZE: usize = LCD_WORDS_PER_LINE * LCD_LINES;

/// For passing frame buffer references
pub type LcdFB = [u32; LCD_FRAME_BUF_SIZE];

/// For storing a full-row wide blit pattern
pub type BlitRow = [u32; LCD_WORDS_PER_LINE];

/// For specifying a vertical region contiguous rows in the frame buffer
#[derive(Copy, Clone)]
pub struct YRegion(pub usize, pub usize);

/// For specifiying horizontal bounds within a vertical region
#[derive(Copy, Clone)]
pub struct XRegion(pub usize, pub usize);

/// Blit string with: XOR, bold font, align xr left yr top
pub fn string_bold_left(fb: &mut LcdFB, mut xr: XRegion, yr: YRegion, s: &str) {
    let f = Font::new(fonts::GlyphSet::Bold);
    for c in s.chars() {
        xr.0 += xor_char(fb, xr, yr, c, f);
    }
}

/// Blit string with: XOR, regular font, align xr left yr top
pub fn string_regular_left(fb: &mut LcdFB, mut xr: XRegion, yr: YRegion, s: &str) {
    let f = Font::new(fonts::GlyphSet::Regular);
    for c in s.chars() {
        xr.0 += xor_char(fb, xr, yr, c, f);
    }
}

/// Blit string with: XOR, small font, align xr left yr top
pub fn string_small_left(fb: &mut LcdFB, mut xr: XRegion, yr: YRegion, s: &str) {
    let f = Font::new(fonts::GlyphSet::Small);
    for c in s.chars() {
        xr.0 += xor_char(fb, xr, yr, c, f);
    }
}

/// Blit a char with: XOR, align left:xr.0 top:yr.0, pad L:1px R:2px
/// Precondition: glyph pattern width must be 32px or less
/// Return: width in pixels of character + padding that were blitted (0 for error)
///
/// Examples of word alignment for source data (rows of glpyh pixels)
/// 1. Fits in one word:
///    row_width:8, row:1 => (data[0].bit_27)->(data[0].bit_24), mask:0x0f00_0000
///    | data[0]                                 |
///    | /- 8px -\ /- 8px -\ /- 8px -\ /- 8px -\ |
///    | 0123 4567           0123 4567           |
///    |           89ab cdef           89ab cdef |
///                ^^^^^^^^^
/// 2. Spans word boundary:
///    row_width:11, row:2 => (data[0].bit_09)->(data[1].bit_31), mask:[0x0000_03ff, 0x800_0000]
///    | data[0]                                 | data[1]    |
///    | /--- 11px --\/--- 11px ---\/---- 11px --+-\/-----... |
///    | 0123 4567 89a              67 89ab cdef | 0          |
///    |              b cdef 0123 45             |  123 45... |
///                                 ^^^^^^^^^^^^^^^^
///
/// Examples of word alignment for destination frame buffer:
/// 1. Fits in word: xr:1..7   => (data[0].bit_30)->(data[0].bit_26), mask:0x7c00_0000
/// 2. Spans words:  xr:30..36 => (data[0].bit_01)->(data[1].bit_29), mask:[0x0000_0003,0xe000_000]
///
pub fn xor_char(fb: &mut LcdFB, xr: XRegion, yr: YRegion, c: char, f: Font) -> usize {
    if yr.1 > LCD_LINES || xr.1 > LCD_PX_PER_LINE || xr.0 >= xr.1 {
        return 0;
    }
    // Look up glyph and unpack its header
    let gpo = (f.glyph_pattern_offset)(c);
    let gh = GlyphHeader::new((f.glyph_data)(gpo));
    if gh.w > 32 {
        return 0;
    }
    // Add 1px pad to left
    let x0 = xr.0 + 1;
    // Calculate word alignment for destination buffer
    let x1 = x0 + gh.w;
    let dest_low_word = x0 >> 5;
    let dest_high_word = x1 >> 5;
    let px_in_dest_low_word = 32 - (x0 & 0x1f);
    // Blit it
    let y0 = yr.0 + gh.y_offset;
    let y_max = if (y0 + gh.h) <= yr.1 { gh.h } else { yr.1 - y0 };
    for y in 0..y_max {
        // Unpack pixels for this glyph row
        let px_offset = y * gh.w;
        let low_word = gpo + 1 + (px_offset >> 5);
        let px_in_low_word = 32 - (px_offset & 0x1f);
        let mut pattern = (f.glyph_data)(low_word);
        if gh.w <= px_in_low_word {
            // Low word contains all pixels for this row
            pattern = pattern >> (px_in_low_word - gh.w);
            pattern = pattern << (32 - gh.w);
        } else {
            // Pixels for this row span two words
            pattern = pattern << (32 - px_in_low_word);
            let px_in_high_word = gh.w - px_in_low_word;
            let pattern_h = (f.glyph_data)(low_word + 1);
            pattern |= (pattern_h >> (32 - px_in_high_word)) << (32 - gh.w);
        }
        // XOR glyph pixels onto destination buffer
        let base = (y0 + y) * LCD_WORDS_PER_LINE;
        fb[base + dest_low_word] ^= pattern >> (32 - px_in_dest_low_word);
        if px_in_dest_low_word < gh.w {
            fb[base + dest_high_word] ^= pattern << px_in_dest_low_word;
        }
    }
    let width_of_blitted_pixels = (x0 + gh.w + 2) - xr.0;
    return width_of_blitted_pixels;
}

/// Clear a full width screen region bounded by y0..y1
pub fn clear_region(fb: &mut LcdFB, yr: YRegion) {
    if yr.1 > LCD_LINES || yr.0 >= yr.1 {
        return;
    }
    for y in yr.0..yr.1 {
        line_fill_clear(fb, y);
    }
}

/// Outline a full width screen region, bounded by y0..y1, with pad and border box
pub fn outline_region(fb: &mut LcdFB, yr: YRegion) {
    if yr.1 > LCD_LINES || yr.0 + 6 >= yr.1 {
        return;
    }
    line_fill_clear(fb, yr.0);
    line_fill_clear(fb, yr.0 + 1);
    line_fill_padded_solid(fb, yr.0 + 2);
    for y in yr.0 + 3..yr.1 - 3 {
        line_fill_padded_border(fb, y);
    }
    line_fill_padded_solid(fb, yr.1 - 3);
    line_fill_clear(fb, yr.1 - 2);
    line_fill_clear(fb, yr.1 - 1);
}

/// Clear a line of the screen
pub fn line_fill_clear(fb: &mut LcdFB, y: usize) {
    if y >= LCD_LINES {
        return;
    }
    let base = y * LCD_WORDS_PER_LINE;
    for i in 0..=9 {
        fb[base + i] = 0xffff_ffff;
    }
    fb[base + 10] = 0xffff_0000;
}

/// Fill a line of the screen with full-width pattern
pub fn line_fill_pattern(fb: &mut LcdFB, y: usize, pattern: &BlitRow) {
    if y >= LCD_LINES {
        return;
    }
    let base = y * LCD_WORDS_PER_LINE;
    for (i, v) in pattern.iter().enumerate() {
        fb[base + i] = *v;
    }
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
