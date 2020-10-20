use super::blit;
use super::fonts;
use super::kbd;

/// Battery charge levels
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Battery {
    B05,
    B25,
    B50,
    B75,
    B99,
}

/// Radio signal strength levels
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Radio {
    R3,
    R2,
    R1,
    R0,
    ROff,
}

/// LCD frame buffer
pub struct FrameBuf {
    pub buf: blit::LcdFB,
    is_dirty: u32,
}
impl FrameBuf {
    pub const fn new() -> FrameBuf {
        FrameBuf {
            buf: [0; blit::LCD_FRAME_BUF_SIZE],
            is_dirty: 0,
        }
    }

    /// Mark frame buffer as dirty (needs repaint)
    pub fn set_dirty(&mut self) {
        self.is_dirty = 1;
    }

    /// Mark frame buffer as clean (does not need repaint)
    pub fn clear_dirty(&mut self) {
        self.is_dirty = 0;
    }

    /// Return non-zero if the frame buffer needs a repaint
    pub fn dirty(&self) -> u32 {
        self.is_dirty
    }
}

/// Size of the buffer backing TextBuf
pub const MAX_CHARS: usize = 14;
pub const CHAR_BUF_SIZE: usize = MAX_CHARS;
pub const UTF8_BUF_SIZE: usize = MAX_CHARS * 4;

/// Status bar data, home screen text buffer, keyboard modkeys, etc.
pub struct Context<'a> {
    pub status_battery: Battery,
    pub status_radio: Radio,
    pub status_title: &'a str,
    pub status_time: &'a str,
    // Home screen sample text
    pub note: &'a str,
    // Character and string buffer for a minimalist FIFO string editor
    pub char_buf: [char; CHAR_BUF_SIZE],
    pub char_buf_end: usize,
    pub utf8_buf: [u8; UTF8_BUF_SIZE],
    pub utf8_buf_end: usize,
    // Keyboard
    pub kbd_layout: kbd::Layout,
    pub kbd_modkey: kbd::ModKey,
    // Demo Animation
    pub demo_frame: usize,
}
impl Context<'_> {
    /// Initialize a GUI context object
    pub const fn new() -> Context<'static> {
        Context {
            status_battery: Battery::B75,
            status_radio: Radio::R3,
            status_title: &"home",
            status_time: &"12:34",
            note: &"Hello, world!",
            char_buf: ['\0'; CHAR_BUF_SIZE],
            char_buf_end: 0,
            utf8_buf: [0; UTF8_BUF_SIZE],
            utf8_buf_end: 0,
            kbd_layout: kbd::Layout::Azerty,
            kbd_modkey: kbd::ModKey::Base,
            demo_frame: 0,
        }
    }

    /// Change the battery level (intended for UI demonstration)
    pub fn cycle_battery(&mut self) {
        self.status_battery = match self.status_battery {
            Battery::B05 => Battery::B25,
            Battery::B25 => Battery::B50,
            Battery::B50 => Battery::B75,
            Battery::B75 => Battery::B99,
            Battery::B99 => Battery::B05,
        };
    }

    /// Change the radio signal strength (intended for UI demonstration)
    pub fn cycle_radio(&mut self) {
        self.status_radio = match self.status_radio {
            Radio::R3 => Radio::ROff,
            Radio::R2 => Radio::R3,
            Radio::R1 => Radio::R2,
            Radio::R0 => Radio::R1,
            Radio::ROff => Radio::R0,
        }
    }

    /// Translate battery level to an icon char (Unicode Private Use Area)
    pub fn battery_icon(&self) -> &'static str {
        match self.status_battery {
            Battery::B05 => fonts::pua::BATTERY_05,
            Battery::B25 => fonts::pua::BATTERY_25,
            Battery::B50 => fonts::pua::BATTERY_50,
            Battery::B75 => fonts::pua::BATTERY_75,
            Battery::B99 => fonts::pua::BATTERY_99,
        }
    }

    pub fn radio_icon(&self) -> &'static str {
        match self.status_radio {
            Radio::R3 => fonts::pua::RADIO_3,
            Radio::R2 => fonts::pua::RADIO_2,
            Radio::R1 => fonts::pua::RADIO_1,
            Radio::R0 => fonts::pua::RADIO_0,
            Radio::ROff => fonts::pua::RADIO_OFF,
        }
    }

    /// Accumulate buffer of recently typed characters
    pub fn buffer_keystroke(&mut self, c: char) {
        // Update the character buffer
        if self.char_buf_end < CHAR_BUF_SIZE {
            // Append character when character buffer is not yet full
            self.char_buf[self.char_buf_end] = c;
            self.char_buf_end += 1;
        } else {
            // When buffer is full, discard oldest, then append
            for i in 0..CHAR_BUF_SIZE - 1 {
                self.char_buf[i] = self.char_buf[i + 1];
            }
            self.char_buf[CHAR_BUF_SIZE - 1] = c;
        }
        // Encode the character buffer as utf-8 into the utf8 buffer
        let mut end = 0;
        for c in self.char_buf[0..self.char_buf_end].iter() {
            let dest = &mut self.utf8_buf[end..end + 4];
            let result = c.encode_utf8(dest);
            end += result.len();
        }
        self.utf8_buf_end = end;
    }

    /// Get string slice for buffer of recently typed characters
    pub fn buffer(&self) -> &str {
        match core::str::from_utf8(&self.utf8_buf[0..self.utf8_buf_end]) {
            Ok(s) => s,
            Err(_) => &"",
        }
    }
}
