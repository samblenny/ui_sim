extern crate blit;

/// Screen bounds
#[allow(dead_code)]
pub const SCREEN_X: usize = 336;
pub const SCREEN_Y: usize = 536;

/// Status bar height and Y bounds
pub const STATUS_H: usize = blit::fonts::bold::MAX_HEIGHT as usize;
pub const STATUS_Y0: usize = 0;
pub const STATUS_Y1: usize = STATUS_Y0 + STATUS_H;

/// Keyboard height and Y bounds
pub const KBD_KEY_H: usize = 33;
pub const KBD_H: usize = (KBD_KEY_H * 6) + 1;
pub const KBD_Y0: usize = SCREEN_Y - KBD_H;
pub const KBD_Y1: usize = SCREEN_Y;

/// Main content area height and Y bounds
#[allow(dead_code)]
pub const MAIN_H: usize = KBD_Y0 - STATUS_Y1;
pub const MAIN_Y0: usize = STATUS_Y1;
pub const MAIN_Y1: usize = KBD_Y0;

/// Home screen with status bar, main content box, and keyboard
pub fn home_screen(
    mut fb: &mut blit::LcdFB,
    title: &str,
    wifi: &str,
    battery: &str,
    time: &str,
    note: &str,
) {
    // Status bar: view title, battery level icon, wifi strength icon, clock
    let yr = blit::YRegion(STATUS_Y0, STATUS_Y1);
    blit::clear_region(&mut fb, yr);
    let mut xr = blit::XRegion(4, blit::LCD_PX_PER_LINE);
    blit::string_bold_left(&mut fb, xr, yr, title);
    xr.0 = 33 * 6 - 6;
    blit::string_bold_left(&mut fb, xr, yr, battery);
    xr.0 = 33 * 7 - 3;
    blit::string_bold_left(&mut fb, xr, yr, wifi);
    xr.0 = 33 * 8 - 2;
    blit::string_bold_left(&mut fb, xr, yr, time);
    // Main content area: 2px clear pad, 1px black border, clear fill, note in center
    let mut yr = blit::YRegion(MAIN_Y0, MAIN_Y1);
    blit::outline_region(&mut fb, yr);
    xr.0 = 5;
    yr.0 += 5;
    blit::string_bold_left(&mut fb, xr, yr, note);
    yr.0 += blit::fonts::bold::MAX_HEIGHT as usize;
    blit::string_regular_left(&mut fb, xr, yr, note);
    yr.0 += blit::fonts::regular::MAX_HEIGHT as usize;
    blit::string_small_left(&mut fb, xr, yr, note);
    // Keyboard
    blank_keyboard(&mut fb, blit::YRegion(KBD_Y0, KBD_Y1));
}

/// Fill a full width screen region bounded by y0..y1 with a blank keyboard
fn blank_keyboard(fb: &mut blit::LcdFB, yr: blit::YRegion) {
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
}
