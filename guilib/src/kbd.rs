#![allow(dead_code)]
//! Model keyboard configuration and state

use super::state;

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
    Shift,
    AltL,
    AltR,
    AltRS,
}

/// Possible mappings of keystroke to resulting character or action
pub enum Map {
    Azerty = 0,
    AzertyS = 1,
    AzertyAltL = 2,
    AzertyAltR = 3,
    AzertyAltRS = 4,
    Qwerty = 5,
    QwertyS = 6,
    QwertyAlt = 7,
}

/// Determine current mapping of keystrokes to resulting characters or actions
pub fn cur_map_enum(ctx: &state::Context) -> Map {
    match ctx.kbd_layout {
        Layout::Azerty => match ctx.kbd_modkey {
            ModKey::Base => Map::Azerty,
            ModKey::Shift => Map::AzertyS,
            ModKey::AltL => Map::AzertyAltL,
            ModKey::AltR => Map::AzertyAltR,
            ModKey::AltRS => Map::AzertyAltRS,
        },
        Layout::Qwerty => match ctx.kbd_modkey {
            ModKey::Base => Map::Qwerty,
            ModKey::Shift => Map::QwertyS,
            ModKey::AltL => Map::QwertyAlt,
            ModKey::AltR => Map::QwertyAlt,
            ModKey::AltRS => Map::QwertyAlt,
        },
    }
}

/// Return index of current keyboard map for passing to javascript
pub fn cur_map_index(ctx: &state::Context) -> i32 {
    cur_map_enum(ctx) as i32
}

/// Return lookup table of current keyboard map for handling keystroke results
pub fn cur_map_lut(ctx: &state::Context) -> &'static MapResultLUT {
    match cur_map_enum(ctx) {
        Map::Azerty => &MAP_AZERTY_BASE,
        Map::AzertyS => &MAP_AZERTY_SHIFT,
        Map::AzertyAltL => &MAP_AZERTY_ALTL,
        Map::AzertyAltR => &MAP_AZERTY_ALTR,
        Map::AzertyAltRS => &MAP_AZERTY_ALTRS,
        Map::Qwerty => &MAP_QWERTY_BASE,
        Map::QwertyS => &MAP_QWERTY_SHIFT,
        Map::QwertyAlt => &MAP_QWERTY_ALT,
    }
}

/// Change current keyboard layout
pub fn set_layout(ctx: &mut state::Context, lo: Layout) {
    ctx.kbd_layout = lo;
}

/// Change current modifier key state
pub fn set_modkey(ctx: &mut state::Context, mk: ModKey) {
    ctx.kbd_modkey = mk;
}

/// Handle a modifier key press event
pub fn modkey_down(ctx: &mut state::Context, r: &R) {
    match ctx.kbd_layout {
        Layout::Azerty => match r {
            // Azerty has separate mappings for AltL and AltR
            R::AltL => match ctx.kbd_modkey {
                ModKey::AltL => ctx.kbd_modkey = ModKey::Base,
                _ => ctx.kbd_modkey = ModKey::AltL,
            },
            R::AltR => match ctx.kbd_modkey {
                ModKey::Shift => ctx.kbd_modkey = ModKey::AltRS,
                ModKey::AltR => ctx.kbd_modkey = ModKey::Base,
                ModKey::AltRS => ctx.kbd_modkey = ModKey::Shift,
                _ => ctx.kbd_modkey = ModKey::AltR,
            },
            R::Shift => match ctx.kbd_modkey {
                ModKey::Base => ctx.kbd_modkey = ModKey::Shift,
                ModKey::Shift => ctx.kbd_modkey = ModKey::Base,
                ModKey::AltR => ctx.kbd_modkey = ModKey::AltRS,
                ModKey::AltRS => ctx.kbd_modkey = ModKey::AltR,
                _ => (),
            },
            _ => (),
        },
        Layout::Qwerty => match r {
            // Qwerty only has one Alt mapping, so AltL can turn AltR off and
            // vice versa
            R::AltL => match ctx.kbd_modkey {
                ModKey::Base => ctx.kbd_modkey = ModKey::AltL,
                ModKey::Shift => ctx.kbd_modkey = ModKey::AltL,
                _ => ctx.kbd_modkey = ModKey::Base,
            },
            R::AltR => match ctx.kbd_modkey {
                ModKey::Base => ctx.kbd_modkey = ModKey::AltR,
                ModKey::Shift => ctx.kbd_modkey = ModKey::AltR,
                _ => ctx.kbd_modkey = ModKey::Base,
            },
            R::Shift => match ctx.kbd_modkey {
                ModKey::Base => ctx.kbd_modkey = ModKey::Shift,
                ModKey::Shift => ctx.kbd_modkey = ModKey::Base,
                _ => (),
            },
            _ => (),
        },
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
    Shift,
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
    R::Shift,  // P4
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

/// Keyboard map for Azerty with shift
pub const MAP_AZERTY_SHIFT: MapResultLUT = [
    R::Up,     // P2 Nav and function keys
    R::Left,   // P5
    R::Click,  // PC
    R::Right,  // P6
    R::F1,     // P3
    R::Shift,  // P4
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
    R::C('A'), // P23 Upper letter row
    R::C('Z'), // P24
    R::C('E'), // P25
    R::C('R'), // P26
    R::C('T'), // P27
    R::C('Y'), // P28
    R::C('U'), // P29
    R::C('I'), // P30
    R::C('O'), // P31
    R::C('P'), // P32
    R::C('Q'), // P33 Home letter row
    R::C('S'), // P34
    R::C('D'), // P35
    R::C('F'), // P36
    R::C('G'), // P37
    R::C('H'), // P38
    R::C('J'), // P39
    R::C('K'), // P40
    R::C('L'), // P41
    R::C('M'), // P42
    R::Nop,    // P43 Lower letter row
    R::C('W'), // P44
    R::C('X'), // P45
    R::C('C'), // P46
    R::C('V'), // P47
    R::C('B'), // P48
    R::C('N'), // P49
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
    R::Shift,    // P4
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

/// Keyboard map for Azerty with AltR and Shift
pub const MAP_AZERTY_ALTRS: MapResultLUT = [
    R::Up,       // P2 Nav and function keys
    R::Left,     // P5
    R::Click,    // PC
    R::Right,    // P6
    R::F1,       // P3
    R::Shift,    // P4
    R::Down,     // P9
    R::F3,       // P7
    R::F4,       // P8
    R::C('À'),  // P13 Number row
    R::C('É'),  // P14
    R::C('È'),  // P15
    R::C('Ê'),  // P16
    R::C('('),   // P17
    R::C(')'),   // P18
    R::C('&'),   // P19
    R::C('*'),   // P20
    R::C('«'),  // P21
    R::C('»'),  // P22
    R::C('Æ'),  // P23 Upper letter row
    R::C('£'),  // P24
    R::C('€'), // P25
    R::C('`'),   // P26
    R::C('{'),   // P27
    R::C('}'),   // P28
    R::C('Ù'),  // P29
    R::C('Ï'),  // P30
    R::C('Œ'),  // P31
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
    R::C('Ç'),  // P46
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
    R::Shift,  // P4
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
    R::C('m'), // P50
    R::C('?'), // P51
    R::Enter,  // P52
    R::AltL,   // P53 Bottom row
    R::C(','), // P54
    R::C(' '), // P55
    R::C('.'), // P56
    R::AltR,   // P57
];

/// Keyboard map for Qwerty with Shift
pub const MAP_QWERTY_SHIFT: MapResultLUT = [
    R::Up,     // P2 Nav and function keys
    R::Left,   // P5
    R::Click,  // PC
    R::Right,  // P6
    R::F1,     // P3
    R::Shift,  // P4
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
    R::C('Q'), // P23 Upper letter row
    R::C('W'), // P24
    R::C('E'), // P25
    R::C('R'), // P26
    R::C('T'), // P27
    R::C('Y'), // P28
    R::C('U'), // P29
    R::C('I'), // P30
    R::C('O'), // P31
    R::C('P'), // P32
    R::C('A'), // P33 Home letter row
    R::C('S'), // P34
    R::C('D'), // P35
    R::C('F'), // P36
    R::C('G'), // P37
    R::C('H'), // P38
    R::C('J'), // P39
    R::C('K'), // P40
    R::C('L'), // P41
    R::Bksp,   // P42
    R::C('!'), // P43 Lower letter row
    R::C('Z'), // P44
    R::C('X'), // P45
    R::C('C'), // P46
    R::C('V'), // P47
    R::C('B'), // P48
    R::C('N'), // P49
    R::C('M'), // P50
    R::C('?'), // P51
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
