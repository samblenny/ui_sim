//! Model keyboard configuration and state
#![allow(dead_code)]

use crate::trace;

pub static mut CUR_LAYOUT: Layout = Layout::Azerty;
pub static mut CUR_MODKEY: ModKey = ModKey::Base;

/// Possible keyboard layouts (relates to labels on physical keys)
#[derive(Copy, Clone)]
pub enum Layout {
    Azerty,
    Qwerty,
}

/// Possible modifier key states (relates to sequence of keystrokes)
#[derive(Copy, Clone)]
pub enum ModKey {
    Base,
    AltL,
    AltR,
}

/// Possible mappings of keystroke to resulting character or action
pub enum Map {
    Azerty = 0,
    AzertyAltL = 1,
    AzertyAltR = 2,
    Qwerty = 3,
}

/// Determine current mapping of keystrokes to resulting characters or actions
pub fn cur_map_enum() -> Map {
    let layout;
    let modkey;
    unsafe {
        layout = CUR_LAYOUT;
        modkey = CUR_MODKEY;
    }
    match layout {
        Layout::Azerty => match modkey {
            ModKey::Base => Map::Azerty,
            ModKey::AltL => Map::AzertyAltL,
            ModKey::AltR => Map::AzertyAltR,
        },
        Layout::Qwerty => match modkey {
            ModKey::Base => Map::Qwerty,
            ModKey::AltL => Map::Qwerty,
            ModKey::AltR => Map::Qwerty,
        },
    }
}

/// Return index of current keyboard map for passing to javascript
pub fn cur_map_index() -> i32 {
    cur_map_enum() as i32
}

/// Return lookup table of current keyboard map for handling keystroke results
pub fn cur_map_lut() -> &'static MapResultLUT {
    match cur_map_enum() {
        Map::Azerty => &MAP_AZERTY_BASE,
        _ => &MAP_AZERTY_BASE,
        // Map::AzertyAltL => &MAP_AZERTY_ALTL,
        // Map::AzertyAltR => &MAP_AZERTY_ALTR,
        // Map::Qwerty => &MAP_QWERTY_BASE,
    }
}

/// Handle a modifier key press event
pub fn modkey_down(r: &R) {
    let mut modkey;
    unsafe {
        modkey = CUR_MODKEY;
    }
    match r {
        R::AltL => match modkey {
            ModKey::Base => modkey = ModKey::AltL,
            ModKey::AltL => modkey = ModKey::Base,
            ModKey::AltR => modkey = ModKey::AltL,
        }
        R::AltR => match modkey {
            ModKey::Base => modkey = ModKey::AltR,
            ModKey::AltL => modkey = ModKey::AltR,
            ModKey::AltR => modkey = ModKey::Base,
        }
        _ => trace::log_code(trace::Code::BadModkeyDownR),
    }
    unsafe {
        CUR_MODKEY = modkey;
    }
}

/// Possible result of keystroke (event or character)
pub enum R {
    Nop,
    Up,
    Left,
    Click,
    Right,
    Down,
    F1,
    F2,
    F3,
    F4,
    AltL,
    AltR,
    Bksp,
    Enter,
    C(char),
}

/// Keyboard maps lookup tables all have 54 keys
pub const MAP_SIZE: usize = 54;

/// Type for all the keyboard map lookup tables
pub type MapResultLUT = [R; MAP_SIZE];

/// Keyboard map for Azerty with no active modifier keys
pub const MAP_AZERTY_BASE: MapResultLUT = [
    R::Up,     // P2 Nav and function keys
    R::Left,   // P5
    R::Click,  // PC
    R::Right,  // P6
    R::F1,     // P3
    R::F2,     // P4
    R::Down,   // P9
    R::F3,     // P7
    R::F4,     // P8
    R::C('1'), // P13 Number row
    R::C('2'), // P14
    R::C('3'), // P15
    R::C('4'), // P16
    R::C('5'), // P17
    R::C('6'), // P18
    R::C('7'), // P19
    R::C('8'), // P20
    R::C('9'), // P21
    R::C('0'), // P22
    R::C('a'), // P23 Upper letter row
    R::C('z'), // P24
    R::C('e'), // P25
    R::C('r'), // P26
    R::C('t'), // P27
    R::C('y'), // P28
    R::C('u'), // P29
    R::C('i'), // P30
    R::C('o'), // P31
    R::C('p'), // P32
    R::C('q'), // P33 Home letter row
    R::C('s'), // P34
    R::C('d'), // P35
    R::C('f'), // P36
    R::C('g'), // P37
    R::C('h'), // P38
    R::C('j'), // P39
    R::C('k'), // P40
    R::C('l'), // P41
    R::C('m'), // P42
    R::Nop,    // P43 Lower letter row
    R::C('w'), // P44
    R::C('x'), // P45
    R::C('c'), // P46
    R::C('v'), // P47
    R::C('b'), // P48
    R::C('n'), // P49
    R::C(':'), // P50
    R::C(';'), // P51
    R::Nop,    // P52
    R::AltL,   // P53 Bottom row
    R::C(','), // P54
    R::C(' '), // P55
    R::C('.'), // P56
    R::AltR,   // P57
];
