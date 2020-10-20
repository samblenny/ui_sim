#![no_std]

mod blit;
mod fonts;
mod kbd;
pub mod state;
mod views;

/// Public API for keyboard and screen events
pub mod api {
    use super::{demo, kbd, state, views};

    /// Repaint the active view
    pub fn repaint(fb: &mut state::FrameBuf, ctx: &mut state::Context) {
        views::home_screen(fb, ctx);
    }

    /// Handle a key down event
    pub fn keydown(fb: &mut state::FrameBuf, ctx: &mut state::Context, key_index: u32) {
        if key_index >= kbd::MAP_SIZE as u32 {
            return;
        }
        let result = &kbd::cur_map_lut(ctx)[key_index as usize];
        match result {
            kbd::R::C(c) => {
                ctx.buffer_keystroke(*c);
                repaint(fb, ctx);
            }
            kbd::R::AltL => {
                kbd::modkey_down(ctx, result);
                repaint(fb, ctx);
            }
            kbd::R::AltR => {
                kbd::modkey_down(ctx, result);
                repaint(fb, ctx);
            }
            kbd::R::Shift => {
                kbd::modkey_down(ctx, result);
                repaint(fb, ctx);
            }
            _ => (),
        }
        views::keyboard_invert_key(fb, key_index as usize);
    }

    /// Handle a key up event
    pub fn keyup(fb: &mut state::FrameBuf, _ctx: &mut state::Context, key_index: u32) {
        if key_index >= kbd::MAP_SIZE as u32 {
            return;
        }
        views::keyboard_invert_key(fb, key_index as usize);
    }

    /// Change keyboard layout to azerty
    pub fn kbd_set_layout_azerty(fb: &mut state::FrameBuf, ctx: &mut state::Context) {
        kbd::set_layout(ctx, kbd::Layout::Azerty);
        kbd::set_modkey(ctx, kbd::ModKey::Base);
        repaint(fb, ctx);
    }

    /// Change keyboard layout to qwerty
    pub fn kbd_set_layout_qwerty(fb: &mut state::FrameBuf, ctx: &mut state::Context) {
        kbd::set_layout(ctx, kbd::Layout::Qwerty);
        kbd::set_modkey(ctx, kbd::ModKey::Base);
        repaint(fb, ctx);
    }

    /// Step the UI demonstration animation by 1 tick
    pub fn demo_tick(fb: &mut state::FrameBuf, ctx: &mut state::Context) {
        demo::tick(fb, ctx);
    }
}

/// UI Demonstration functions to substitute for unimplemented UI events
mod demo {
    use super::{api, state};

    /// Step the UI demonstration animation by 1 tick
    pub fn tick(fb: &mut state::FrameBuf, ctx: &mut state::Context) {
        let fr = ctx.demo_frame;
        ctx.demo_frame = (fr + 1) % 171;
        match fr {
            0..=4 => {
                ctx.cycle_radio();
                api::repaint(fb, ctx);
            }
            5..=9 => {
                ctx.cycle_battery();
                api::repaint(fb, ctx);
            }
            10 => api::kbd_set_layout_qwerty(fb, ctx),
            11 => api::keydown(fb, ctx, Key::P4 as u32), // shift (F2)
            12 => api::keyup(fb, ctx, Key::P4 as u32),   // shift (F2)
            13 => api::keydown(fb, ctx, Key::P24 as u32), // W
            14 => api::keyup(fb, ctx, Key::P24 as u32),  // W
            15 => api::keydown(fb, ctx, Key::P4 as u32), // shift (F2)
            16 => api::keyup(fb, ctx, Key::P4 as u32),   // shift (F2)
            17 => api::keydown(fb, ctx, Key::P38 as u32), // h
            18 => api::keyup(fb, ctx, Key::P38 as u32),  // h
            19 => api::keydown(fb, ctx, Key::P33 as u32), // a
            20 => api::keyup(fb, ctx, Key::P33 as u32),  // a
            21 => api::keydown(fb, ctx, Key::P27 as u32), // t
            22 => api::keyup(fb, ctx, Key::P27 as u32),  // t
            23 => api::keydown(fb, ctx, Key::P55 as u32), //
            24 => api::keyup(fb, ctx, Key::P55 as u32),  //
            25 => api::keydown(fb, ctx, Key::P30 as u32), // i
            26 => api::keyup(fb, ctx, Key::P30 as u32),  // i
            27 => api::keydown(fb, ctx, Key::P34 as u32), // s
            28 => api::keyup(fb, ctx, Key::P34 as u32),  // s
            29 => api::keydown(fb, ctx, Key::P55 as u32), //
            30 => api::keyup(fb, ctx, Key::P55 as u32),  //
            31 => api::keydown(fb, ctx, Key::P30 as u32), // i
            32 => api::keyup(fb, ctx, Key::P30 as u32),  // i
            33 => api::keydown(fb, ctx, Key::P27 as u32), // t
            34 => api::keyup(fb, ctx, Key::P27 as u32),  // t
            35 => api::keydown(fb, ctx, Key::P55 as u32), //
            36 => api::keyup(fb, ctx, Key::P55 as u32),  //
            37 => api::keydown(fb, ctx, Key::P28 as u32), // y
            38 => api::keyup(fb, ctx, Key::P28 as u32),  // y
            39 => api::keydown(fb, ctx, Key::P31 as u32), // o
            40 => api::keyup(fb, ctx, Key::P31 as u32),  // o
            41 => api::keydown(fb, ctx, Key::P29 as u32), // u
            42 => api::keyup(fb, ctx, Key::P29 as u32),  // u
            43 => api::keydown(fb, ctx, Key::P55 as u32), //
            44 => api::keyup(fb, ctx, Key::P55 as u32),  //
            45 => api::keydown(fb, ctx, Key::P24 as u32), // w
            46 => api::keyup(fb, ctx, Key::P24 as u32),  // w
            47 => api::keydown(fb, ctx, Key::P31 as u32), // o
            48 => api::keyup(fb, ctx, Key::P31 as u32),  // o
            49 => api::keydown(fb, ctx, Key::P29 as u32), // u
            50 => api::keyup(fb, ctx, Key::P29 as u32),  // u
            51 => api::keydown(fb, ctx, Key::P41 as u32), // l
            52 => api::keyup(fb, ctx, Key::P41 as u32),  // l
            53 => api::keydown(fb, ctx, Key::P35 as u32), // d
            54 => api::keyup(fb, ctx, Key::P35 as u32),  // d
            55 => api::keydown(fb, ctx, Key::P55 as u32), //
            56 => api::keyup(fb, ctx, Key::P55 as u32),  //
            57 => api::keydown(fb, ctx, Key::P34 as u32), // s
            58 => api::keyup(fb, ctx, Key::P34 as u32),  // s
            59 => api::keydown(fb, ctx, Key::P25 as u32), // e
            60 => api::keyup(fb, ctx, Key::P25 as u32),  // e
            61 => api::keydown(fb, ctx, Key::P25 as u32), // e
            62 => api::keyup(fb, ctx, Key::P25 as u32),  // e
            63 => api::keydown(fb, ctx, Key::P51 as u32), // ?
            64 => api::keyup(fb, ctx, Key::P51 as u32),  // ?
            65 => api::keydown(fb, ctx, Key::P55 as u32), //
            66 => api::keyup(fb, ctx, Key::P55 as u32),  //
            67 => api::keydown(fb, ctx, Key::P4 as u32), // shift (F2)
            68 => api::keyup(fb, ctx, Key::P4 as u32),   // shift (F2)
            69 => api::keydown(fb, ctx, Key::P30 as u32), // I
            70 => api::keyup(fb, ctx, Key::P30 as u32),  // I
            71 => api::keydown(fb, ctx, Key::P4 as u32), // shift (F2)
            72 => api::keyup(fb, ctx, Key::P4 as u32),   // shift (F2)
            73 => api::keydown(fb, ctx, Key::P36 as u32), // f
            74 => api::keyup(fb, ctx, Key::P36 as u32),  // f
            75 => api::keydown(fb, ctx, Key::P55 as u32), //
            76 => api::keyup(fb, ctx, Key::P55 as u32),  //
            77 => api::keydown(fb, ctx, Key::P33 as u32), // a
            78 => api::keyup(fb, ctx, Key::P33 as u32),  // a
            79 => api::keydown(fb, ctx, Key::P29 as u32), // u
            80 => api::keyup(fb, ctx, Key::P29 as u32),  // u
            81 => api::keydown(fb, ctx, Key::P37 as u32), // g
            82 => api::keyup(fb, ctx, Key::P37 as u32),  // g
            83 => api::keydown(fb, ctx, Key::P38 as u32), // h
            84 => api::keyup(fb, ctx, Key::P38 as u32),  // h
            85 => api::keydown(fb, ctx, Key::P27 as u32), // t
            86 => api::keyup(fb, ctx, Key::P27 as u32),  // t
            87 => api::keydown(fb, ctx, Key::P55 as u32), //
            88 => api::keyup(fb, ctx, Key::P55 as u32),  //
            89 => api::keydown(fb, ctx, Key::P31 as u32), // o
            90 => api::keyup(fb, ctx, Key::P31 as u32),  // o
            91 => api::keydown(fb, ctx, Key::P36 as u32), // f
            92 => api::keyup(fb, ctx, Key::P36 as u32),  // f
            93 => api::keydown(fb, ctx, Key::P55 as u32), //
            94 => api::keyup(fb, ctx, Key::P55 as u32),  //
            95 => api::keydown(fb, ctx, Key::P24 as u32), // w
            96 => api::keyup(fb, ctx, Key::P24 as u32),  // w
            97 => api::keydown(fb, ctx, Key::P31 as u32), // o
            98 => api::keyup(fb, ctx, Key::P31 as u32),  // o
            99 => api::keydown(fb, ctx, Key::P25 as u32), // e
            100 => api::keyup(fb, ctx, Key::P25 as u32), // e
            101 => api::keydown(fb, ctx, Key::P55 as u32), //
            102 => api::keyup(fb, ctx, Key::P55 as u32), //
            103 => api::keydown(fb, ctx, Key::P31 as u32), // o
            104 => api::keyup(fb, ctx, Key::P31 as u32), // o
            105 => api::keydown(fb, ctx, Key::P26 as u32), // r
            106 => api::keyup(fb, ctx, Key::P26 as u32), // r
            107 => api::keydown(fb, ctx, Key::P55 as u32), //
            108 => api::keyup(fb, ctx, Key::P55 as u32), //
            109 => api::keydown(fb, ctx, Key::P24 as u32), // w
            110 => api::keyup(fb, ctx, Key::P24 as u32), // w
            111 => api::keydown(fb, ctx, Key::P31 as u32), // o
            112 => api::keyup(fb, ctx, Key::P31 as u32), // o
            113 => api::keydown(fb, ctx, Key::P49 as u32), // n
            114 => api::keyup(fb, ctx, Key::P49 as u32), // n
            115 => api::keydown(fb, ctx, Key::P35 as u32), // d
            116 => api::keyup(fb, ctx, Key::P35 as u32), // d
            117 => api::keydown(fb, ctx, Key::P25 as u32), // e
            118 => api::keyup(fb, ctx, Key::P25 as u32), // e
            119 => api::keydown(fb, ctx, Key::P26 as u32), // r
            120 => api::keyup(fb, ctx, Key::P26 as u32), // r
            121 => api::keydown(fb, ctx, Key::P54 as u32), // ,
            122 => api::keyup(fb, ctx, Key::P54 as u32), // ,
            123 => api::keydown(fb, ctx, Key::P55 as u32), //
            124 => api::keyup(fb, ctx, Key::P55 as u32), //
            125 => api::keydown(fb, ctx, Key::P46 as u32), // c
            126 => api::keyup(fb, ctx, Key::P46 as u32), // c
            127 => api::keydown(fb, ctx, Key::P25 as u32), // e
            128 => api::keyup(fb, ctx, Key::P25 as u32), // e
            129 => api::keydown(fb, ctx, Key::P33 as u32), // a
            130 => api::keyup(fb, ctx, Key::P33 as u32), // a
            131 => api::keydown(fb, ctx, Key::P34 as u32), // s
            132 => api::keyup(fb, ctx, Key::P34 as u32), // s
            133 => api::keydown(fb, ctx, Key::P25 as u32), // e
            134 => api::keyup(fb, ctx, Key::P25 as u32), // e
            135 => api::keydown(fb, ctx, Key::P55 as u32), //
            136 => api::keyup(fb, ctx, Key::P55 as u32), //
            137 => api::keydown(fb, ctx, Key::P28 as u32), // y
            138 => api::keyup(fb, ctx, Key::P28 as u32), // y
            139 => api::keydown(fb, ctx, Key::P31 as u32), // o
            140 => api::keyup(fb, ctx, Key::P31 as u32), // o
            141 => api::keydown(fb, ctx, Key::P29 as u32), // u
            142 => api::keyup(fb, ctx, Key::P29 as u32), // u
            143 => api::keydown(fb, ctx, Key::P26 as u32), // r
            144 => api::keyup(fb, ctx, Key::P26 as u32), // r
            145 => api::keydown(fb, ctx, Key::P55 as u32), //
            146 => api::keyup(fb, ctx, Key::P55 as u32), //
            147 => api::keydown(fb, ctx, Key::P34 as u32), // s
            148 => api::keyup(fb, ctx, Key::P34 as u32), // s
            149 => api::keydown(fb, ctx, Key::P25 as u32), // e
            150 => api::keyup(fb, ctx, Key::P25 as u32), // e
            151 => api::keydown(fb, ctx, Key::P33 as u32), // a
            152 => api::keyup(fb, ctx, Key::P33 as u32), // a
            153 => api::keydown(fb, ctx, Key::P26 as u32), // r
            154 => api::keyup(fb, ctx, Key::P26 as u32), // r
            155 => api::keydown(fb, ctx, Key::P46 as u32), // c
            156 => api::keyup(fb, ctx, Key::P46 as u32), // c
            157 => api::keydown(fb, ctx, Key::P38 as u32), // h
            158 => api::keyup(fb, ctx, Key::P38 as u32), // h
            159 => api::keydown(fb, ctx, Key::P56 as u32), // .
            160 => api::keyup(fb, ctx, Key::P56 as u32), // .
            161 => api::keydown(fb, ctx, Key::P55 as u32), //
            162 => api::keyup(fb, ctx, Key::P55 as u32), //
            163 => api::keydown(fb, ctx, Key::P55 as u32), //
            164 => api::keyup(fb, ctx, Key::P55 as u32), //
            165 => api::keydown(fb, ctx, Key::P55 as u32), //
            166 => api::keyup(fb, ctx, Key::P55 as u32), //
            167 => api::keydown(fb, ctx, Key::P55 as u32), //
            168 => api::keyup(fb, ctx, Key::P55 as u32), //
            169 => api::keydown(fb, ctx, Key::P55 as u32), //
            170 => api::keyup(fb, ctx, Key::P55 as u32), //
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
