use super::fonts::{pua, Font};
use super::state::{home, status};
use super::{blit, fonts, kbd, state};

/// Screen bounds
pub const SCREEN_W: usize = blit::LCD_PX_PER_LINE;
pub const SCREEN_H: usize = blit::LCD_LINES;

/// Status bar height and Y bounds
pub const STATUS_H: usize = fonts::bold::MAX_HEIGHT as usize;
pub const STATUS_Y0: usize = 0;
pub const STATUS_Y1: usize = STATUS_Y0 + STATUS_H;

/// Keyboard height and Y bounds
pub const KBD_KEY_H: usize = 33;
pub const KBD_H: usize = (KBD_KEY_H * 6) + 1;
pub const KBD_Y0: usize = SCREEN_H - KBD_H;
pub const KBD_Y1: usize = SCREEN_H;

/// Main content area height and Y bounds
#[allow(dead_code)]
pub const MAIN_H: usize = KBD_Y0 - STATUS_Y1;
pub const MAIN_Y0: usize = STATUS_Y1;
pub const MAIN_Y1: usize = KBD_Y0;

/// Home screen with status bar, main content box, and keyboard
pub fn home_screen(mut fb: &mut blit::LcdFB) {
    // Status bar: view title, battery level icon, wifi strength icon, clock
    let mut cr = blit::ClipRegion {
        x0: 0,
        x1: SCREEN_W,
        y0: STATUS_Y0,
        y1: STATUS_Y1,
    };
    blit::clear_region(&mut fb, cr);
    cr.x0 = 4;
    blit::string_bold_left(&mut fb, cr, unsafe { status::TITLE });
    cr.x0 = 33 * 6 - 6;
    blit::string_bold_left(&mut fb, cr, status::battery_icon());
    cr.x0 = 33 * 7 - 3;
    blit::string_bold_left(&mut fb, cr, status::radio_icon());
    cr.x0 = 33 * 8 - 2;
    blit::string_bold_left(&mut fb, cr, unsafe { status::TIME });
    // Main content area: 2px clear pad, 1px black border, clear fill, note in center
    let yr = blit::YRegion(MAIN_Y0, MAIN_Y1);
    blit::outline_region(&mut fb, yr);
    let mut cr = blit::ClipRegion {
        x0: 5,
        x1: SCREEN_W,
        y0: yr.0 + 5,
        y1: MAIN_Y1,
    };
    blit::string_bold_left(&mut fb, cr, unsafe { home::NOTE });
    cr.y0 += fonts::bold::MAX_HEIGHT as usize;
    blit::string_regular_left(&mut fb, cr, unsafe { home::NOTE });
    cr.y0 += fonts::regular::MAX_HEIGHT as usize;
    blit::string_small_left(&mut fb, cr, unsafe { home::NOTE });
    cr.y0 += fonts::small::MAX_HEIGHT as usize * 2;
    blit::string_regular_left(&mut fb, cr, home::buffer());
    // Onscreen keyboard
    keyboard(&mut fb, blit::YRegion(KBD_Y0, KBD_Y1));
    state::lcd::set_dirty();
}

/// Fill a full width screen region bounded by y0..y1 with a blank keyboard
fn keyboard(fb: &mut blit::LcdFB, yr: blit::YRegion) {
    if yr.1 - yr.0 != KBD_H || yr.1 > blit::LCD_LINES {
        return;
    }
    // Blit patterns for the three different styles of key rows
    let fkey_row = [
        0xe0000000, 0x00000000, 0x08000000, 0x00000000, 0x03ffffff, 0xffffffff, 0xffc00000,
        0x00000000, 0x00100000, 0x00000000, 0x0007ffff,
    ];
    let alphanumeric_row = [
        0xe0000000, 0x10000000, 0x08000000, 0x04000000, 0x02000000, 0x01800000, 0x00400000,
        0x00200000, 0x00100000, 0x00080000, 0x0007ffff,
    ];
    let spacebar_row = [
        0xffffffff, 0xf0000000, 0x08000000, 0x04000000, 0x00000000, 0x00000000, 0x00000000,
        0x00200000, 0x00100000, 0x000fffff, 0xffffffff,
    ];
    // Blit 1 row of F-keys
    let mut y = yr.0;
    blit::line_fill_clear(fb, y);
    for i in 1..KBD_KEY_H {
        blit::line_fill_pattern(fb, y + i, &fkey_row);
    }
    // Blit 4 rows of alphanumeric keys
    for _ in 0..4 {
        y += KBD_KEY_H;
        blit::line_fill_clear(fb, y);
        for i in 1..KBD_KEY_H {
            blit::line_fill_pattern(fb, y + i, &alphanumeric_row);
        }
    }
    // Blit the spacebar row
    y += KBD_KEY_H;
    blit::line_fill_clear(fb, y);
    for i in 1..KBD_KEY_H {
        blit::line_fill_pattern(fb, y + i, &spacebar_row);
    }
    blit::line_fill_clear(fb, y + KBD_KEY_H);
    // Add keycap labels
    keyboard_key_caps(fb, yr);
}

/// Label key caps for the onscreen keyboard using XOR blit
fn keyboard_key_caps(fb: &mut blit::LcdFB, yr: blit::YRegion) {
    if yr.1 - yr.0 != KBD_H || yr.1 > blit::LCD_LINES {
        return;
    }
    let y0 = yr.0;
    let mut cr = blit::ClipRegion {
        x0: 0,
        x1: SCREEN_W,
        y0: yr.0,
        y1: yr.1,
    };
    let f = Font::new(fonts::GlyphSet::Regular);
    let lut = kbd::cur_map_lut();
    for i in 0..KEY_LABEL_CR_LUT.len() {
        // If this key postion gets an onscreen label...
        if let KeyL::CR(key_cr) = KEY_LABEL_CR_LUT[i] {
            cr.y0 = y0 + key_cr.y0;
            // And the current key map gives a label for this key
            // ...then blit the label
            if let kbd::R::C(c) = lut[i] {
                let w = blit::char_width(c, f);
                cr.x0 = key_cr.x0 + ((key_cr.x1 - key_cr.x0) >> 1) - (w >> 1);
                blit::xor_char(fb, cr, c, f);
            } else {
                let label = match lut[i] {
                    kbd::R::Shift => &"shift",
                    kbd::R::AltL | kbd::R::AltR => pua::SHIFT_ARROW,
                    kbd::R::Enter => pua::ENTER_SYMBOL,
                    kbd::R::Bksp => pua::BACKSPACE_SYMBOL,
                    _ => &"",
                };
                let w = blit::string_width(&label, f);
                cr.x0 = key_cr.x0 + ((key_cr.x1 - key_cr.x0) >> 1) - (w >> 1);
                cr.y0 = y0 + key_cr.y0;
                blit::string_regular_left(fb, cr, &label);
            }
        }
    }
}

/// Invert a key (minus 1px border) to indicate it is pressed
pub fn keyboard_invert_key(fb: &mut blit::LcdFB, key_index: usize) {
    if key_index >= kbd::MAP_SIZE {
        return;
    }
    if let KeyL::CR(cr_rel) = KEY_LABEL_CR_LUT[key_index] {
        let cr = blit::ClipRegion {
            x0: cr_rel.x0 + 3,
            x1: cr_rel.x1,
            y0: KBD_Y0 + cr_rel.y0,
            y1: KBD_Y0 + cr_rel.y1 - 3,
        };
        blit::invert_region(fb, cr);
    }
    state::lcd::set_dirty();
}

/// Draw test patern of stripes
#[allow(dead_code)]
pub fn stripes(fb: &mut blit::LcdFB) {
    let mut i = 0;
    let mut pattern: u32 = 0xffffff03;
    for _line in 0..blit::LCD_LINES {
        for _word in 0..blit::LCD_WORDS_PER_LINE - 1 {
            fb[i] = pattern;
            i += 1;
        }
        fb[i] = pattern & 0xffff0000;
        i += 1;
        pattern = pattern.rotate_right(1);
    }
    state::lcd::set_dirty();
}

/// Holds ClipRegion for positioning keycap labels in onscreen keyboard
enum KeyL {
    CR(blit::ClipRegion),
    None,
}

/// Calculate key label positions based on row and column within keyboard.
/// Includes padding for top, left, and center gutters plus key outlines.
const fn keypos(col: usize, row: usize, width: usize) -> KeyL {
    let mut x0 = 1 + (col * KBD_KEY_H);
    if col >= 5 {
        x0 += 1;
    }
    let mut x1 = x0 + (width * KBD_KEY_H);
    if col < 5 && col + width > 5 {
        x1 += 1;
    }
    let y0 = 2 + row * KBD_KEY_H;
    KeyL::CR(blit::ClipRegion {
        x0,
        x1,
        y0,
        y1: y0 + KBD_KEY_H,
    })
}

/// X,Y coordinates for onscreen keyboard keycap labels
const KEY_LABEL_CR_LUT: [KeyL; kbd::MAP_SIZE] = [
    KeyL::None,      // P2 (up)
    KeyL::None,      // P5 (left)
    KeyL::None,      // PC (click)
    KeyL::None,      // P6 (right)
    keypos(0, 0, 2), // P3 (F1)
    keypos(2, 0, 2), // P4 (F2)
    KeyL::None,      // P9 (down)
    keypos(6, 0, 2), // P7 (F3)
    keypos(8, 0, 2), // P8 (F4)
    keypos(0, 1, 1), // P13 Number row
    keypos(1, 1, 1), // P14
    keypos(2, 1, 1), // P15
    keypos(3, 1, 1), // P16
    keypos(4, 1, 1), // P17
    keypos(5, 1, 1), // P18
    keypos(6, 1, 1), // P19
    keypos(7, 1, 1), // P20
    keypos(8, 1, 1), // P21
    keypos(9, 1, 1), // P22
    keypos(0, 2, 1), // P23 Upper letter row
    keypos(1, 2, 1), // P24
    keypos(2, 2, 1), // P25
    keypos(3, 2, 1), // P26
    keypos(4, 2, 1), // P27
    keypos(5, 2, 1), // P28
    keypos(6, 2, 1), // P29
    keypos(7, 2, 1), // P30
    keypos(8, 2, 1), // P31
    keypos(9, 2, 1), // P32
    keypos(0, 3, 1), // P33 Home letter row
    keypos(1, 3, 1), // P34
    keypos(2, 3, 1), // P35
    keypos(3, 3, 1), // P36
    keypos(4, 3, 1), // P37
    keypos(5, 3, 1), // P38
    keypos(6, 3, 1), // P39
    keypos(7, 3, 1), // P40
    keypos(8, 3, 1), // P41
    keypos(9, 3, 1), // P42
    keypos(0, 4, 1), // P43 Lower letter row
    keypos(1, 4, 1), // P44
    keypos(2, 4, 1), // P45
    keypos(3, 4, 1), // P46
    keypos(4, 4, 1), // P47
    keypos(5, 4, 1), // P48
    keypos(6, 4, 1), // P49
    keypos(7, 4, 1), // P50
    keypos(8, 4, 1), // P51
    keypos(9, 4, 1), // P52
    keypos(1, 5, 1), // P53 Bottom row
    keypos(2, 5, 1), // P54
    keypos(3, 5, 4), // P55 (Spacebar)
    keypos(7, 5, 1), // P56
    keypos(8, 5, 1), // P57
];
