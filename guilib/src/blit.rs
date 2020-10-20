use super::fonts;
use super::fonts::{Font, GlyphHeader};

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
/// Range is yr.0..yr.1 (yr.0 included, yr.1 excluded)
#[derive(Copy, Clone)]
pub struct YRegion(pub usize, pub usize);

/// For specifying a region of pixels in the frame buffer
/// Ranges are x0..x1 and y0..y1 (x0 & y0 are included, x1 & y1 are excluded)
#[derive(Copy, Clone)]
pub struct ClipRegion {
    pub x0: usize,
    pub x1: usize,
    pub y0: usize,
    pub y1: usize,
}

/// Blit string with: XOR, bold font, align xr left yr top
pub fn string_bold_left(fb: &mut LcdFB, mut cr: ClipRegion, s: &str) {
    let f = Font::new(fonts::GlyphSet::Bold);
    for c in s.chars() {
        cr.x0 += xor_char(fb, cr, c, f);
    }
}

/// Blit string with: XOR, regular font, align xr left yr top
pub fn string_regular_left(fb: &mut LcdFB, mut cr: ClipRegion, s: &str) {
    let f = Font::new(fonts::GlyphSet::Regular);
    for c in s.chars() {
        cr.x0 += xor_char(fb, cr, c, f);
    }
}

/// Blit string with: XOR, small font, align xr left yr top
pub fn string_small_left(fb: &mut LcdFB, mut cr: ClipRegion, s: &str) {
    let f = Font::new(fonts::GlyphSet::Small);
    for c in s.chars() {
        cr.x0 += xor_char(fb, cr, c, f);
    }
}

/// Calculate the width of all glpyhs and padding for a string
pub fn string_width(s: &str, f: Font) -> usize {
    let mut w = 0;
    for c in s.chars() {
        w += char_width(c, f) + 3;
    }
    // Subtle padding math: 3px between chars, 1px at left and right ends
    w - 1
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
pub fn xor_char(fb: &mut LcdFB, cr: ClipRegion, c: char, f: Font) -> usize {
    if cr.y1 > LCD_LINES || cr.x1 > LCD_PX_PER_LINE || cr.x0 >= cr.x1 {
        return 0;
    }
    // Look up glyph and unpack its header
    let gpo = (f.glyph_pattern_offset)(c);
    let gh = GlyphHeader::new((f.glyph_data)(gpo));
    if gh.w > 32 {
        return 0;
    }
    // Add 1px pad to left
    let x0 = cr.x0 + 1;
    // Calculate word alignment for destination buffer
    let x1 = x0 + gh.w;
    let dest_low_word = x0 >> 5;
    let dest_high_word = x1 >> 5;
    let px_in_dest_low_word = 32 - (x0 & 0x1f);
    // Blit it
    let y0 = cr.y0 + gh.y_offset;
    let y_max = if (y0 + gh.h) <= cr.y1 {
        gh.h
    } else {
        cr.y1 - y0
    };
    for y in 0..y_max {
        // Unpack pixels for this glyph row.
        // px_in_low_word can include some or all of the pixels for this row of
        // the pattern. It may also include pixels for the next row, or, in the
        // case of the last row, it may include padding bits.
        let px_offset = y * gh.w;
        let low_word = gpo + 1 + (px_offset >> 5);
        let px_in_low_word = 32 - (px_offset & 0x1f);
        let mut pattern = (f.glyph_data)(low_word);
        // Mask and align pixels from low word of glyph data array
        pattern <<= 32 - px_in_low_word;
        pattern >>= 32 - gh.w;
        if gh.w > px_in_low_word {
            // When pixels for this row span two words in the glyph data array,
            // get pixels from the high word too
            let px_in_high_word = gh.w - px_in_low_word;
            let mut pattern_h = (f.glyph_data)(low_word + 1);
            pattern_h >>= 32 - px_in_high_word;
            pattern |= pattern_h;
        }
        // XOR glyph pixels onto destination buffer
        let base = (y0 + y) * LCD_WORDS_PER_LINE;
        fb[base + dest_low_word] ^= pattern << (32 - px_in_dest_low_word);
        if px_in_dest_low_word < gh.w {
            fb[base + dest_high_word] ^= pattern >> px_in_dest_low_word;
        }
    }
    let width_of_blitted_pixels = (x0 + gh.w + 2) - cr.x0;
    return width_of_blitted_pixels;
}

/// Calculate the width of glpyh for a char
pub fn char_width(c: char, f: Font) -> usize {
    let gpo = (f.glyph_pattern_offset)(c);
    let gh = GlyphHeader::new((f.glyph_data)(gpo));
    gh.w
}

/// Clear a screen region bounded by (cr.x0,cr.y0)..(cr.x0,cr.y1)
pub fn clear_region(fb: &mut LcdFB, cr: ClipRegion) {
    if cr.y1 > LCD_LINES || cr.y0 >= cr.y1 || cr.x1 > LCD_PX_PER_LINE || cr.x0 >= cr.x1 {
        return;
    }
    // Calculate word alignment for destination buffer
    let dest_low_word = cr.x0 >> 5;
    let dest_high_word = cr.x1 >> 5;
    let px_in_dest_low_word = 32 - (cr.x0 & 0x1f);
    let px_in_dest_high_word = cr.x1 & 0x1f;
    // Blit it
    for y in cr.y0..cr.y1 {
        let base = y * LCD_WORDS_PER_LINE;
        fb[base + dest_low_word] |= 0xffffffff << (32 - px_in_dest_low_word);
        for w in dest_low_word + 1..dest_high_word {
            fb[base + w] = 0xffffffff;
        }
        if dest_low_word < dest_high_word {
            fb[base + dest_high_word] |= 0xffffffff >> (32 - px_in_dest_high_word);
        }
    }
}

/// Invert a screen region bounded by (cr.x0,cr.y0)..(cr.x0,cr.y1)
pub fn invert_region(fb: &mut LcdFB, cr: ClipRegion) {
    if cr.y1 > LCD_LINES || cr.y0 >= cr.y1 || cr.x1 > LCD_PX_PER_LINE || cr.x0 >= cr.x1 {
        return;
    }
    // Calculate word alignment for destination buffer
    let dest_low_word = cr.x0 >> 5;
    let dest_high_word = cr.x1 >> 5;
    let px_in_dest_low_word = 32 - (cr.x0 & 0x1f);
    let px_in_dest_high_word = cr.x1 & 0x1f;
    // Blit it
    for y in cr.y0..cr.y1 {
        let base = y * LCD_WORDS_PER_LINE;
        fb[base + dest_low_word] ^= 0xffffffff << (32 - px_in_dest_low_word);
        for w in dest_low_word + 1..dest_high_word {
            fb[base + w] ^= 0xffffffff;
        }
        if dest_low_word < dest_high_word {
            fb[base + dest_high_word] ^= 0xffffffff >> (32 - px_in_dest_high_word);
        }
    }
}

/// Outline a full width screen region with pad and border box
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
    fb[base + 10] = 0x0000_ffff;
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
    fb[base] = 0x0000_0003;
    for i in 1..=9 {
        fb[base + i] = 0x0000_0000;
    }
    fb[base + 10] = 0x0000_c000;
}

/// Fill a line of the screen with clear, bordered by black, padded with clear
fn line_fill_padded_border(fb: &mut LcdFB, y: usize) {
    if y >= LCD_LINES {
        return;
    }
    let base = y * LCD_WORDS_PER_LINE;
    fb[base] = 0xffff_fffb;
    for i in 1..=9 {
        fb[base + i] = 0xffff_ffff;
    }
    fb[base + 10] = 0x0000_dfff;
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
