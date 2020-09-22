#![no_std]
#![allow(dead_code)]
//! Model keyboard configuration and state

extern crate trace;

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
    QwertyAlt = 4,
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
            ModKey::AltL => Map::QwertyAlt,
            ModKey::AltR => Map::QwertyAlt,
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
        Map::AzertyAltL => &MAP_AZERTY_ALTL,
        Map::AzertyAltR => &MAP_AZERTY_ALTR,
        Map::Qwerty => &MAP_QWERTY_BASE,
        Map::QwertyAlt => &MAP_QWERTY_ALT,
    }
}

/// Change current keyboard layout
pub fn set_layout(lo: Layout) {
    unsafe {
        CUR_LAYOUT = lo;
    }
}

/// Handle a modifier key press event
pub fn modkey_down(r: &R) {
    let layout;
    let mut modkey;
    unsafe {
        layout = CUR_LAYOUT;
        modkey = CUR_MODKEY;
    }
    match layout {
        Layout::Azerty => match r {
            // Azerty has separate mappings for AltL and AltR
            R::AltL => match modkey {
                ModKey::Base => modkey = ModKey::AltL,
                ModKey::AltL => modkey = ModKey::Base,
                ModKey::AltR => modkey = ModKey::AltL,
            },
            R::AltR => match modkey {
                ModKey::Base => modkey = ModKey::AltR,
                ModKey::AltL => modkey = ModKey::AltR,
                ModKey::AltR => modkey = ModKey::Base,
            },
            _ => trace::log_code(trace::Code::BadModkeyDownR),
        },
        Layout::Qwerty => match r {
            // Qwerty only has one Alt mapping, so AltL can turn AltR off and
            // vice versa
            R::AltL => match modkey {
                ModKey::Base => modkey = ModKey::AltL,
                ModKey::AltL => modkey = ModKey::Base,
                ModKey::AltR => modkey = ModKey::Base,
            },
            R::AltR => match modkey {
                ModKey::Base => modkey = ModKey::AltR,
                ModKey::AltL => modkey = ModKey::Base,
                ModKey::AltR => modkey = ModKey::Base,
            },
            _ => trace::log_code(trace::Code::BadModkeyDownR),
        },
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
    C(char),
    Bksp,
    Enter,
    AltL,
    Symbol,
    Emoji,
    AltR,
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

/// Keyboard map for Azerty with AltL
pub const MAP_AZERTY_ALTL: MapResultLUT = [
    R::Up,      // P2 Nav and function keys
    R::Left,    // P5
    R::Click,   // PC
    R::Right,   // P6
    R::F1,      // P3
    R::F2,      // P4
    R::Down,    // P9
    R::F3,      // P7
    R::F4,      // P8
    R::C('§'), // P13 Number row
    R::Nop,     // P14
    R::Nop,     // P15
    R::Nop,     // P16
    R::C('['),  // P17
    R::C(']'),  // P18
    R::Nop,     // P19
    R::C('_'),  // P20
    R::C('\''), // P21
    R::C('"'),  // P22
    R::Nop,     // P23 Upper letter row
    R::Nop,     // P24
    R::Nop,     // P25
    R::Nop,     // P26
    R::Nop,     // P27
    R::Nop,     // P28
    R::Nop,     // P29
    R::Nop,     // P30
    R::Nop,     // P31
    R::Nop,     // P32
    R::Nop,     // P33 Home letter row
    R::Nop,     // P34
    R::Nop,     // P35
    R::Nop,     // P36
    R::Nop,     // P37
    R::Nop,     // P38
    R::Nop,     // P39
    R::C('/'),  // P40
    R::Nop,     // P41
    R::Nop,     // P42
    R::Nop,     // P43 Lower letter row
    R::Nop,     // P44
    R::Nop,     // P45
    R::Nop,     // P46
    R::Nop,     // P47
    R::Nop,     // P48
    R::Nop,     // P49
    R::C('¿'), // P50
    R::C('¡'), // P51
    R::Nop,     // P52
    R::AltL,    // P53 Bottom row
    R::Nop,     // P54
    R::C(' '),  // P55
    R::Nop,     // P56
    R::AltR,    // P57
];

/// Keyboard map for Azerty with AltR
pub const MAP_AZERTY_ALTR: MapResultLUT = [
    R::Up,       // P2 Nav and function keys
    R::Left,     // P5
    R::Click,    // PC
    R::Right,    // P6
    R::F1,       // P3
    R::F2,       // P4
    R::Down,     // P9
    R::F3,       // P7
    R::F4,       // P8
    R::C('à'),  // P13 Number row
    R::C('é'),  // P14
    R::C('è'),  // P15
    R::C('ê'),  // P16
    R::C('('),   // P17
    R::C(')'),   // P18
    R::C('&'),   // P19
    R::C('*'),   // P20
    R::C('«'),  // P21
    R::C('»'),  // P22
    R::C('æ'),  // P23 Upper letter row
    R::C('£'),  // P24
    R::C('€'), // P25
    R::C('`'),   // P26
    R::C('{'),   // P27
    R::C('}'),   // P28
    R::C('ù'),  // P29
    R::C('ï'),  // P30
    R::C('œ'),  // P31
    R::C('%'),   // P32
    R::C('@'),   // P33 Home letter row
    R::C('ß'),  // P34
    R::C('$'),   // P35
    R::C('¤'),  // P36
    R::C('µ'),  // P37
    R::C('-'),   // P38
    R::C('+'),   // P39
    R::C('\\'),  // P40
    R::C('|'),   // P41
    R::C('#'),   // P42
    R::Bksp,     // P43 Lower letter row
    R::C('<'),   // P44
    R::C('>'),   // P45
    R::C('ç'),  // P46
    R::C('^'),   // P47
    R::C('='),   // P48
    R::C('~'),   // P49
    R::C('?'),   // P50
    R::C('!'),   // P51
    R::Enter,    // P52
    R::AltL,     // P53 Bottom row
    R::Symbol,   // P54
    R::C(' '),   // P55
    R::Emoji,    // P56
    R::AltR,     // P57
];

/// Keyboard map for Qwerty with no active modifier keys
pub const MAP_QWERTY_BASE: MapResultLUT = [
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
    R::C('q'), // P23 Upper letter row
    R::C('w'), // P24
    R::C('e'), // P25
    R::C('r'), // P26
    R::C('t'), // P27
    R::C('y'), // P28
    R::C('u'), // P29
    R::C('i'), // P30
    R::C('o'), // P31
    R::C('p'), // P32
    R::C('a'), // P33 Home letter row
    R::C('s'), // P34
    R::C('d'), // P35
    R::C('f'), // P36
    R::C('g'), // P37
    R::C('h'), // P38
    R::C('j'), // P39
    R::C('k'), // P40
    R::C('l'), // P41
    R::Bksp,   // P42
    R::C('!'), // P43 Lower letter row
    R::C('z'), // P44
    R::C('x'), // P45
    R::C('c'), // P46
    R::C('v'), // P47
    R::C('b'), // P48
    R::C('n'), // P49
    R::C(':'), // P50
    R::C(';'), // P51
    R::Enter,  // P52
    R::AltL,   // P53 Bottom row
    R::C(','), // P54
    R::C(' '), // P55
    R::C('.'), // P56
    R::AltR,   // P57
];

/// Keyboard map for Qwerty with AltL or AltR
pub const MAP_QWERTY_ALT: MapResultLUT = [
    R::Up,      // P2 Nav and function keys
    R::Left,    // P5
    R::Click,   // PC
    R::Right,   // P6
    R::F1,      // P3
    R::F2,      // P4
    R::Down,    // P9
    R::F3,      // P7
    R::F4,      // P8
    R::Nop,     // P13 Number row
    R::Nop,     // P14
    R::Nop,     // P15
    R::Nop,     // P16
    R::Nop,     // P17
    R::Nop,     // P18
    R::Nop,     // P19
    R::Nop,     // P20
    R::Nop,     // P21
    R::Nop,     // P22
    R::C('%'),  // P23 Upper letter row
    R::C('^'),  // P24
    R::C('~'),  // P25
    R::C('|'),  // P26
    R::C('['),  // P27
    R::C(']'),  // P28
    R::C('<'),  // P29
    R::C('>'),  // P30
    R::C('{'),  // P31
    R::C('}'),  // P32
    R::C('@'),  // P33 Home letter row
    R::C('#'),  // P34
    R::C('&'),  // P35
    R::C('*'),  // P36
    R::C('-'),  // P37
    R::C('+'),  // P38
    R::C('='),  // P39
    R::C('('),  // P40
    R::C(')'),  // P41
    R::Bksp,    // P42
    R::C('`'),  // P43 Lower letter row
    R::C('_'),  // P44
    R::C('$'),  // P45
    R::C('"'),  // P46
    R::C('\''), // P47
    R::C(':'),  // P48
    R::C(';'),  // P49
    R::C('/'),  // P50
    R::C('\\'), // P51
    R::Enter,   // P52
    R::AltL,    // P53 Bottom row
    R::Symbol,  // P54
    R::C(' '),  // P55
    R::Emoji,   // P56
    R::AltR,    // P57
];
