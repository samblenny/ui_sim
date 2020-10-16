extern crate blit;
use blit::fonts;

/// Frame buffer for sharing LCD state between javascript and wasm
pub static mut LCD_FRAME_BUF: blit::LcdFB = [0; blit::LCD_FRAME_BUF_SIZE];
pub static mut LCD_DIRTY: u32 = 0;

/// Status bar state
pub mod status {
    use super::fonts;

    pub static mut BATTERY: Battery = Battery::B75;
    pub static mut RADIO: Radio = Radio::R3;
    pub static mut TITLE: &str = &"home";
    pub static mut TIME: &str = &"12:34";

    /// Translate battery level to an icon char (Unicode Private Use Area)
    pub fn battery_icon() -> &'static str {
        unsafe {
            match BATTERY {
                Battery::B05 => fonts::pua::BATTERY_05,
                Battery::B25 => fonts::pua::BATTERY_25,
                Battery::B50 => fonts::pua::BATTERY_50,
                Battery::B75 => fonts::pua::BATTERY_75,
                Battery::B99 => fonts::pua::BATTERY_99,
            }
        }
    }

    pub fn radio_icon() -> &'static str {
        unsafe {
            match RADIO {
                Radio::R3 => fonts::pua::RADIO_3,
                Radio::R2 => fonts::pua::RADIO_2,
                Radio::R1 => fonts::pua::RADIO_1,
                Radio::R0 => fonts::pua::RADIO_0,
                Radio::ROff => fonts::pua::RADIO_OFF,
            }
        }
    }

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
}

/// Home screen state
pub mod home {
    pub static mut NOTE: &str = &"Hello, world!";

    /// Character and string buffers for a minimalist FIFO string editor
    pub const MAX_CHARS: usize = 14;
    pub const CHAR_BUF_SIZE: usize = MAX_CHARS;
    pub static mut CHAR_BUF: [char; CHAR_BUF_SIZE] = ['\0'; CHAR_BUF_SIZE];
    pub static mut CHAR_BUF_END: usize = 0;
    pub const UTF8_BUF_SIZE: usize = MAX_CHARS * 4;
    pub static mut UTF8_BUF: [u8; UTF8_BUF_SIZE] = [0; UTF8_BUF_SIZE];
    pub static mut UTF8_BUF_END: usize = 0;

    /// Accumulate buffer of recently typed characters
    pub fn buffer_keystroke(c: char) {
        unsafe {
            // Update the character buffer
            if CHAR_BUF_END < CHAR_BUF_SIZE {
                // Append character when character buffer is not yet full
                CHAR_BUF[CHAR_BUF_END] = c;
                CHAR_BUF_END += 1;
            } else {
                // When buffer is full, discard oldest, then append
                for i in 0..CHAR_BUF_SIZE - 1 {
                    CHAR_BUF[i] = CHAR_BUF[i + 1];
                }
                CHAR_BUF[CHAR_BUF_SIZE - 1] = c;
            }
            // Encode the character buffer as utf-8 into the utf8 buffer
            let mut end = 0;
            for c in CHAR_BUF[0..CHAR_BUF_END].iter() {
                let dest = &mut UTF8_BUF[end..end + 4];
                let result = c.encode_utf8(dest);
                end += result.len();
            }
            UTF8_BUF_END = end;
        }
    }

    /// Get string slice for buffer of recently typed characters
    pub fn buffer() -> &'static str {
        unsafe {
            match core::str::from_utf8(&UTF8_BUF[0..UTF8_BUF_END]) {
                Ok(s) => s,
                Err(_) => &"",
            }
        }
    }
}
