#![no_std]

mod blit;
mod fonts;
mod kbd;
mod state;
mod views;

/// Public API for wasm32 shared memory
pub mod api_wasm {
    use super::{blit, state};

    /// Number of words in the frame buffer for each line of the lcd (for wasm)
    pub fn lcd_words_per_line() -> usize {
        blit::LCD_WORDS_PER_LINE
    }

    /// Number of pixels in each line of the frame buffer (for wasm)
    pub fn lcd_px_per_line() -> usize {
        blit::LCD_PX_PER_LINE
    }

    /// Number of lines in the frame buffer
    pub fn lcd_lines() -> usize {
        blit::LCD_LINES
    }

    /// Pointer to the frame buffer (needed for wasm)
    pub fn lcd_frame_buf_ptr() -> *const u32 {
        unsafe { state::lcd::FRAME_BUF.as_ptr() }
    }
}

/// Public API for keyboard and screen events
pub mod api {
    use super::{demo, kbd, state, views};

    /// Repaint the active view
    pub fn repaint() {
        unsafe {
            views::home_screen(&mut state::lcd::FRAME_BUF);
        }
    }

    /// Return non-zero if LCD frame buffer needs a repaint
    pub fn lcd_dirty() -> u32 {
        state::lcd::dirty()
    }

    /// Mark LCD frame buffer as needing a repaint
    pub fn lcd_set_dirty() {
        state::lcd::set_dirty();
    }

    /// Mark LCD frame buffer as clean
    pub fn lcd_clear_dirty() {
        state::lcd::clear_dirty();
    }

    /// Handle a key down event
    pub fn keydown(key_index: u32) {
        if key_index >= kbd::MAP_SIZE as u32 {
            return;
        }
        let result = &kbd::cur_map_lut()[key_index as usize];
        match result {
            kbd::R::C(c) => {
                state::home::buffer_keystroke(*c);
                repaint();
            }
            kbd::R::AltL => {
                kbd::modkey_down(result);
                repaint();
            }
            kbd::R::AltR => {
                kbd::modkey_down(result);
                repaint();
            }
            kbd::R::Shift => {
                kbd::modkey_down(result);
                repaint();
            }
            _ => (),
        }
        unsafe {
            views::keyboard_invert_key(&mut state::lcd::FRAME_BUF, key_index as usize);
        }
    }

    /// Handle a key up event
    pub fn keyup(key_index: u32) {
        if key_index >= kbd::MAP_SIZE as u32 {
            return;
        }
        unsafe {
            views::keyboard_invert_key(&mut state::lcd::FRAME_BUF, key_index as usize);
        }
    }

    /// Change keyboard layout to azerty
    pub fn kbd_set_layout_azerty() {
        kbd::set_layout(kbd::Layout::Azerty);
        kbd::set_modkey(kbd::ModKey::Base);
        repaint();
    }

    /// Change keyboard layout to qwerty
    pub fn kbd_set_layout_qwerty() {
        kbd::set_layout(kbd::Layout::Qwerty);
        kbd::set_modkey(kbd::ModKey::Base);
        repaint();
    }

    /// Step the UI demonstration animation by 1 tick
    pub fn demo_tick() {
        demo::tick();
    }
}

/// UI Demonstration functions to substitute for unimplemented UI events
mod demo {
    use super::{api, state};

    /// Frame counter for the demo animation
    static mut FRAME: usize = 0;

    /// Step the UI demonstration animation by 1 tick
    pub fn tick() {
        let fr;
        unsafe {
            fr = FRAME;
            FRAME = (FRAME + 1) % 171;
        }
        match fr {
            0..=4 => {
                state::status::cycle_radio();
                api::repaint();
            }
            5..=9 => {
                state::status::cycle_battery();
                api::repaint();
            }
            10 => api::kbd_set_layout_qwerty(),
            11 => api::keydown(Key::P4 as u32),   // shift (F2)
            12 => api::keyup(Key::P4 as u32),     // shift (F2)
            13 => api::keydown(Key::P24 as u32),  // W
            14 => api::keyup(Key::P24 as u32),    // W
            15 => api::keydown(Key::P4 as u32),   // shift (F2)
            16 => api::keyup(Key::P4 as u32),     // shift (F2)
            17 => api::keydown(Key::P38 as u32),  // h
            18 => api::keyup(Key::P38 as u32),    // h
            19 => api::keydown(Key::P33 as u32),  // a
            20 => api::keyup(Key::P33 as u32),    // a
            21 => api::keydown(Key::P27 as u32),  // t
            22 => api::keyup(Key::P27 as u32),    // t
            23 => api::keydown(Key::P55 as u32),  //
            24 => api::keyup(Key::P55 as u32),    //
            25 => api::keydown(Key::P30 as u32),  // i
            26 => api::keyup(Key::P30 as u32),    // i
            27 => api::keydown(Key::P34 as u32),  // s
            28 => api::keyup(Key::P34 as u32),    // s
            29 => api::keydown(Key::P55 as u32),  //
            30 => api::keyup(Key::P55 as u32),    //
            31 => api::keydown(Key::P30 as u32),  // i
            32 => api::keyup(Key::P30 as u32),    // i
            33 => api::keydown(Key::P27 as u32),  // t
            34 => api::keyup(Key::P27 as u32),    // t
            35 => api::keydown(Key::P55 as u32),  //
            36 => api::keyup(Key::P55 as u32),    //
            37 => api::keydown(Key::P28 as u32),  // y
            38 => api::keyup(Key::P28 as u32),    // y
            39 => api::keydown(Key::P31 as u32),  // o
            40 => api::keyup(Key::P31 as u32),    // o
            41 => api::keydown(Key::P29 as u32),  // u
            42 => api::keyup(Key::P29 as u32),    // u
            43 => api::keydown(Key::P55 as u32),  //
            44 => api::keyup(Key::P55 as u32),    //
            45 => api::keydown(Key::P24 as u32),  // w
            46 => api::keyup(Key::P24 as u32),    // w
            47 => api::keydown(Key::P31 as u32),  // o
            48 => api::keyup(Key::P31 as u32),    // o
            49 => api::keydown(Key::P29 as u32),  // u
            50 => api::keyup(Key::P29 as u32),    // u
            51 => api::keydown(Key::P41 as u32),  // l
            52 => api::keyup(Key::P41 as u32),    // l
            53 => api::keydown(Key::P35 as u32),  // d
            54 => api::keyup(Key::P35 as u32),    // d
            55 => api::keydown(Key::P55 as u32),  //
            56 => api::keyup(Key::P55 as u32),    //
            57 => api::keydown(Key::P34 as u32),  // s
            58 => api::keyup(Key::P34 as u32),    // s
            59 => api::keydown(Key::P25 as u32),  // e
            60 => api::keyup(Key::P25 as u32),    // e
            61 => api::keydown(Key::P25 as u32),  // e
            62 => api::keyup(Key::P25 as u32),    // e
            63 => api::keydown(Key::P51 as u32),  // ?
            64 => api::keyup(Key::P51 as u32),    // ?
            65 => api::keydown(Key::P55 as u32),  //
            66 => api::keyup(Key::P55 as u32),    //
            67 => api::keydown(Key::P4 as u32),   // shift (F2)
            68 => api::keyup(Key::P4 as u32),     // shift (F2)
            69 => api::keydown(Key::P30 as u32),  // I
            70 => api::keyup(Key::P30 as u32),    // I
            71 => api::keydown(Key::P4 as u32),   // shift (F2)
            72 => api::keyup(Key::P4 as u32),     // shift (F2)
            73 => api::keydown(Key::P36 as u32),  // f
            74 => api::keyup(Key::P36 as u32),    // f
            75 => api::keydown(Key::P55 as u32),  //
            76 => api::keyup(Key::P55 as u32),    //
            77 => api::keydown(Key::P33 as u32),  // a
            78 => api::keyup(Key::P33 as u32),    // a
            79 => api::keydown(Key::P29 as u32),  // u
            80 => api::keyup(Key::P29 as u32),    // u
            81 => api::keydown(Key::P37 as u32),  // g
            82 => api::keyup(Key::P37 as u32),    // g
            83 => api::keydown(Key::P38 as u32),  // h
            84 => api::keyup(Key::P38 as u32),    // h
            85 => api::keydown(Key::P27 as u32),  // t
            86 => api::keyup(Key::P27 as u32),    // t
            87 => api::keydown(Key::P55 as u32),  //
            88 => api::keyup(Key::P55 as u32),    //
            89 => api::keydown(Key::P31 as u32),  // o
            90 => api::keyup(Key::P31 as u32),    // o
            91 => api::keydown(Key::P36 as u32),  // f
            92 => api::keyup(Key::P36 as u32),    // f
            93 => api::keydown(Key::P55 as u32),  //
            94 => api::keyup(Key::P55 as u32),    //
            95 => api::keydown(Key::P24 as u32),  // w
            96 => api::keyup(Key::P24 as u32),    // w
            97 => api::keydown(Key::P31 as u32),  // o
            98 => api::keyup(Key::P31 as u32),    // o
            99 => api::keydown(Key::P25 as u32),  // e
            100 => api::keyup(Key::P25 as u32),   // e
            101 => api::keydown(Key::P55 as u32), //
            102 => api::keyup(Key::P55 as u32),   //
            103 => api::keydown(Key::P31 as u32), // o
            104 => api::keyup(Key::P31 as u32),   // o
            105 => api::keydown(Key::P26 as u32), // r
            106 => api::keyup(Key::P26 as u32),   // r
            107 => api::keydown(Key::P55 as u32), //
            108 => api::keyup(Key::P55 as u32),   //
            109 => api::keydown(Key::P24 as u32), // w
            110 => api::keyup(Key::P24 as u32),   // w
            111 => api::keydown(Key::P31 as u32), // o
            112 => api::keyup(Key::P31 as u32),   // o
            113 => api::keydown(Key::P49 as u32), // n
            114 => api::keyup(Key::P49 as u32),   // n
            115 => api::keydown(Key::P35 as u32), // d
            116 => api::keyup(Key::P35 as u32),   // d
            117 => api::keydown(Key::P25 as u32), // e
            118 => api::keyup(Key::P25 as u32),   // e
            119 => api::keydown(Key::P26 as u32), // r
            120 => api::keyup(Key::P26 as u32),   // r
            121 => api::keydown(Key::P54 as u32), // ,
            122 => api::keyup(Key::P54 as u32),   // ,
            123 => api::keydown(Key::P55 as u32), //
            124 => api::keyup(Key::P55 as u32),   //
            125 => api::keydown(Key::P46 as u32), // c
            126 => api::keyup(Key::P46 as u32),   // c
            127 => api::keydown(Key::P25 as u32), // e
            128 => api::keyup(Key::P25 as u32),   // e
            129 => api::keydown(Key::P33 as u32), // a
            130 => api::keyup(Key::P33 as u32),   // a
            131 => api::keydown(Key::P34 as u32), // s
            132 => api::keyup(Key::P34 as u32),   // s
            133 => api::keydown(Key::P25 as u32), // e
            134 => api::keyup(Key::P25 as u32),   // e
            135 => api::keydown(Key::P55 as u32), //
            136 => api::keyup(Key::P55 as u32),   //
            137 => api::keydown(Key::P28 as u32), // y
            138 => api::keyup(Key::P28 as u32),   // y
            139 => api::keydown(Key::P31 as u32), // o
            140 => api::keyup(Key::P31 as u32),   // o
            141 => api::keydown(Key::P29 as u32), // u
            142 => api::keyup(Key::P29 as u32),   // u
            143 => api::keydown(Key::P26 as u32), // r
            144 => api::keyup(Key::P26 as u32),   // r
            145 => api::keydown(Key::P55 as u32), //
            146 => api::keyup(Key::P55 as u32),   //
            147 => api::keydown(Key::P34 as u32), // s
            148 => api::keyup(Key::P34 as u32),   // s
            149 => api::keydown(Key::P25 as u32), // e
            150 => api::keyup(Key::P25 as u32),   // e
            151 => api::keydown(Key::P33 as u32), // a
            152 => api::keyup(Key::P33 as u32),   // a
            153 => api::keydown(Key::P26 as u32), // r
            154 => api::keyup(Key::P26 as u32),   // r
            155 => api::keydown(Key::P46 as u32), // c
            156 => api::keyup(Key::P46 as u32),   // c
            157 => api::keydown(Key::P38 as u32), // h
            158 => api::keyup(Key::P38 as u32),   // h
            159 => api::keydown(Key::P56 as u32), // .
            160 => api::keyup(Key::P56 as u32),   // .
            161 => api::keydown(Key::P55 as u32), //
            162 => api::keyup(Key::P55 as u32),   //
            163 => api::keydown(Key::P55 as u32), //
            164 => api::keyup(Key::P55 as u32),   //
            165 => api::keydown(Key::P55 as u32), //
            166 => api::keyup(Key::P55 as u32),   //
            167 => api::keydown(Key::P55 as u32), //
            168 => api::keyup(Key::P55 as u32),   //
            169 => api::keydown(Key::P55 as u32), //
            170 => api::keyup(Key::P55 as u32),   //
            _ => (),
        }
    }

    /// Lookup table to translate from keycode to key index
    /// Comments refer to key caps of base qwerty layout
    #[allow(dead_code)]
    enum Key {
        P2 = 0,   // Up
        P5 = 1,   // Left
        PC = 2,   // Click
        P6 = 3,   // Right
        P3 = 4,   // F1
        P4 = 5,   // F2 (shift)
        P9 = 6,   // Down
        P7 = 7,   // F3
        P8 = 8,   // F4
        P13 = 9,  // 1
        P14 = 10, // 2
        P15 = 11, // 3
        P16 = 12, // 4
        P17 = 13, // 5
        P18 = 14, // 6
        P19 = 15, // 7
        P20 = 16, // 8
        P21 = 17, // 9
        P22 = 18, // 0
        P23 = 19, // q
        P24 = 20, // w
        P25 = 21, // e
        P26 = 22, // r
        P27 = 23, // t
        P28 = 24, // y
        P29 = 25, // u
        P30 = 26, // i
        P31 = 27, // o
        P32 = 28, // p
        P33 = 29, // a
        P34 = 30, // s
        P35 = 31, // d
        P36 = 32, // f
        P37 = 33, // g
        P38 = 34, // h
        P39 = 35, // j
        P40 = 36, // k
        P41 = 37, // l
        P42 = 38, // backspace
        P43 = 39, // !
        P44 = 40, // z
        P45 = 41, // x
        P46 = 42, // c
        P47 = 43, // v
        P48 = 44, // b
        P49 = 45, // n
        P50 = 46, // m
        P51 = 47, // ?
        P52 = 48, // enter
        P53 = 49, // leftShift
        P54 = 50, // ,
        P55 = 51, // space
        P56 = 52, // .
        P57 = 53, // rightShift
    }
}
