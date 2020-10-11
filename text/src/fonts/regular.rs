#![allow(dead_code)]
//! Regular Font

/// Return offset into DATA[] for start of pattern depicting glyph for character c
pub fn get_glyph_pattern_offset(c: char) -> usize {
    match c as u32 {
        0x20..=0x7E => BASIC_LATIN[(c as usize) - 0x20] as usize,
        0xA0..=0xFF => LATIN_1[(c as usize) - 0xA0] as usize,
        0x152..=0x153 => LATIN_EXTENDED_A[(c as usize) - 0x152] as usize,
        0x2018..=0x2022 => GENERAL_PUNCTUATION[(c as usize) - 0x2018] as usize,
        0x20AC..=0x20AC => CURRENCY_SYMBOLS[(c as usize) - 0x20AC] as usize,
        _ => SPECIALS[(0xFFFD as usize) - 0xFFFD] as usize,
    }
}

// Index to Unicode Basic Latin block glyph patterns
const BASIC_LATIN: [u16; 95] = [
    0, // ' '
    2, // '!'
    5, // '"'
    7, // '#'
    13, // '$'
    21, // '%'
    30, // '&'
    40, // '''
    42, // '('
    48, // ')'
    54, // '*'
    59, // '+'
    64, // ','
    66, // '-'
    68, // '.'
    70, // '/'
    78, // '0'
    86, // '1'
    90, // '2'
    98, // '3'
    106, // '4'
    115, // '5'
    123, // '6'
    131, // '7'
    139, // '8'
    147, // '9'
    155, // ':'
    157, // ';'
    160, // '<'
    165, // '='
    169, // '>'
    174, // '?'
    182, // '@'
    191, // 'A'
    200, // 'B'
    208, // 'C'
    216, // 'D'
    224, // 'E'
    231, // 'F'
    238, // 'G'
    246, // 'H'
    254, // 'I'
    257, // 'J'
    265, // 'K'
    273, // 'L'
    280, // 'M'
    289, // 'N'
    297, // 'O'
    305, // 'P'
    313, // 'Q'
    323, // 'R'
    331, // 'S'
    339, // 'T'
    348, // 'U'
    356, // 'V'
    365, // 'W'
    377, // 'X'
    384, // 'Y'
    391, // 'Z'
    398, // '['
    402, // '\'
    410, // ']'
    414, // '^'
    417, // '_'
    419, // '`'
    421, // 'a'
    427, // 'b'
    434, // 'c'
    440, // 'd'
    447, // 'e'
    453, // 'f'
    459, // 'g'
    467, // 'h'
    474, // 'i'
    477, // 'j'
    483, // 'k'
    490, // 'l'
    493, // 'm'
    502, // 'n'
    508, // 'o'
    514, // 'p'
    521, // 'q'
    528, // 'r'
    534, // 's'
    540, // 't'
    546, // 'u'
    552, // 'v'
    558, // 'w'
    567, // 'x'
    573, // 'y'
    582, // 'z'
    588, // '{'
    594, // '|'
    597, // '}'
    603, // '~'
];

// Index to Unicode Latin 1 block glyph patterns
const LATIN_1: [u16; 96] = [
    606, // ' '
    608, // '¡'
    611, // '¢'
    618, // '£'
    626, // '¤'
    633, // '¥'
    640, // '¦'
    643, // '§'
    653, // '¨'
    655, // '©'
    664, // 'ª'
    669, // '«'
    674, // '¬'
    677, // '­'
    679, // '®'
    688, // '¯'
    690, // '°'
    693, // '±'
    699, // '²'
    702, // '³'
    705, // '´'
    707, // 'µ'
    717, // '¶'
    728, // '·'
    730, // '¸'
    732, // '¹'
    735, // 'º'
    740, // '»'
    745, // '¼'
    758, // '½'
    771, // '¾'
    784, // '¿'
    791, // 'À'
    803, // 'Á'
    815, // 'Â'
    827, // 'Ã'
    839, // 'Ä'
    850, // 'Å'
    861, // 'Æ'
    874, // 'Ç'
    884, // 'È'
    893, // 'É'
    902, // 'Ê'
    911, // 'Ë'
    919, // 'Ì'
    923, // 'Í'
    927, // 'Î'
    933, // 'Ï'
    939, // 'Ð'
    948, // 'Ñ'
    958, // 'Ò'
    968, // 'Ó'
    978, // 'Ô'
    988, // 'Õ'
    998, // 'Ö'
    1008, // '×'
    1013, // 'Ø'
    1023, // 'Ù'
    1033, // 'Ú'
    1043, // 'Û'
    1053, // 'Ü'
    1063, // 'Ý'
    1072, // 'Þ'
    1079, // 'ß'
    1087, // 'à'
    1095, // 'á'
    1103, // 'â'
    1111, // 'ã'
    1119, // 'ä'
    1126, // 'å'
    1135, // 'æ'
    1144, // 'ç'
    1151, // 'è'
    1159, // 'é'
    1167, // 'ê'
    1175, // 'ë'
    1182, // 'ì'
    1186, // 'í'
    1190, // 'î'
    1195, // 'ï'
    1200, // 'ð'
    1207, // 'ñ'
    1215, // 'ò'
    1223, // 'ó'
    1231, // 'ô'
    1239, // 'õ'
    1247, // 'ö'
    1254, // '÷'
    1259, // 'ø'
    1267, // 'ù'
    1275, // 'ú'
    1283, // 'û'
    1291, // 'ü'
    1298, // 'ý'
    1309, // 'þ'
    1317, // 'ÿ'
];

// Index to Unicode Latin Extended A block glyph patterns
const LATIN_EXTENDED_A: [u16; 2] = [
    1327, // 'Œ'
    1340, // 'œ'
];

// Index to General Punctuation block glyph patterns
const GENERAL_PUNCTUATION: [u16; 11] = [
    1349, // '‘'
    1351, // '’'
    1353, // '‚'
    1355, // '‛'
    1357, // '“'
    1360, // '”'
    1363, // '„'
    1366, // '‟'
    1369, // '†'
    1376, // '‡'
    1384, // '•'
];

// Index to Unicode Currency Symbols block glyph patterns
const CURRENCY_SYMBOLS: [u16; 1] = [
    1390, // '€'
];

// Index to Unicode Specials block glyph patterns
const SPECIALS: [u16; 1] = [
    1399, // '�'
];

/// Maximum height of glyph patterns in this bitmap typeface.
/// This will be true: h + yOffset <= MAX_HEIGHT
pub const MAX_HEIGHT: u8 = 30;

/// Packed glyph pattern data.
/// Record format:
///  [offset+0]: ((w as u8) << 16) | ((h as u8) << 8) | (yOffset as u8)
///  [offset+1..=ceil(w*h/32)]: packed 1-bit pixels; 0=clear, 1=set
/// Pixels are packed in top to bottom, left to right order with MSB of first
/// pixel word containing the top left pixel.
///  w: Width of pattern in pixels
///  h: Height of pattern in pixels
///  yOffset: Vertical offset (pixels downward from top of line) to position
///     glyph pattern properly relative to text baseline
pub const DATA: [u32; 1412] = [
    // [0]: 20 ' '
    0x0004020e, 0x00000000,
    // [2]: 21 '!'
    0x00021206, 0xfffffff0, 0xf0000000,
    // [5]: 22 '"'
    0x00060406, 0xcf3cf300,
    // [7]: 23 '#'
    0x000e0a06, 0x03300cc3, 0xffcfff0c, 0xc0330fff, 0x3ffc3300, 0xcc000000,
    // [13]: 24 '$'
    0x000a1604, 0x0c0303f0, 0xfcccf33c, 0xc3303c0f, 0x00f03c0c, 0xc330cc33, 0xccf333f0, 0xfc0c0300,
    // [21]: 25 '%'
    0x000e1206, 0x3ffcfffc, 0x30f0c3c3, 0x330cc3cc, 0x0f300300, 0x0c00cf03, 0x3c330ccc, 0x3c30f0c3,
    0xc0f303c0,
    // [30]: 26 '&'
    0x00101206, 0x0f000f00, 0x30c030c0, 0x33003300, 0x0c000c00, 0x33003300, 0xc0ccc0cc, 0xc030c030,
    0xc0ccc0cc, 0x3f033f03,
    // [40]: 27 '\''
    0x00020406, 0xff000000,
    // [42]: 28 '('
    0x00061604, 0x0c330c30, 0xcc30c30c, 0x30c30c30, 0x30c30c0c, 0x30000000,
    // [48]: 29 ')'
    0x00061604, 0xc3030c30, 0xc0c30c30, 0xc30c30c3, 0x30c30cc3, 0x00000000,
    // [54]: 2A '*'
    0x000a0a08, 0x0c030ccf, 0x333f0fc3, 0x30ccc0f0, 0x30000000,
    // [59]: 2B '+'
    0x000a0a0a, 0x0c0300c0, 0x30fffff0, 0xc0300c03, 0x00000000,
    // [64]: 2C ','
    0x00040616, 0x3333cc00,
    // [66]: 2D '-'
    0x000a020e, 0xfffff000,
    // [68]: 2E '.'
    0x00020216, 0xf0000000,
    // [70]: 2F '/'
    0x000a1404, 0x00c0300c, 0x030300c0, 0x300c0c03, 0x00c03030, 0x0c0300c0, 0xc0300c03, 0x00000000,
    // [78]: 30 '0'
    0x000c1206, 0x0f00f030, 0xc30cc03c, 0x03c03c03, 0xc03c03c0, 0x3c03c03c, 0x0330c30c, 0x0f00f000,
    // [86]: 31 '1'
    0x00041206, 0x33ff3333, 0x33333333, 0x33000000,
    // [90]: 32 '2'
    0x000c1206, 0x0fc0fc30, 0x3303c03c, 0x03003003, 0x00c00c03, 0x00300c00, 0xc0300300, 0xffffff00,
    // [98]: 33 '3'
    0x000c1206, 0xffffff00, 0xc00c0300, 0x300f00f0, 0x00c00c00, 0x30030030, 0x03c0cc0c, 0x3f03f000,
    // [106]: 34 '4'
    0x000e1206, 0x003000c0, 0x0f003c03, 0x300cc0c3, 0x030c3030, 0xc0cc0330, 0x0cffffff, 0xf003000c,
    0x003000c0,
    // [115]: 35 '5'
    0x000c1206, 0xffffffc0, 0x0c00c00c, 0x00ffcffc, 0x00300300, 0x30030030, 0x03c03c03, 0x3fc3fc00,
    // [123]: 36 '6'
    0x000c1206, 0x0fc0fc30, 0x0300c00c, 0x00ffcffc, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0x3fc3fc00,
    // [131]: 37 '7'
    0x000c1206, 0xffffff00, 0x300300c0, 0x0c00c00c, 0x03003003, 0x00300c00, 0xc00c00c0, 0x0c00c000,
    // [139]: 38 '8'
    0x000c1206, 0x3fc3fcc0, 0x3c03c03c, 0x033fc3fc, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0x3fc3fc00,
    // [147]: 39 '9'
    0x000c1206, 0x3fc3fcc0, 0x3c03c03c, 0x03c03c03, 0xc03c033f, 0xf3ff0030, 0x0300c00c, 0x3f03f000,
    // [155]: 3A ':'
    0x00020c0c, 0xf0000f00,
    // [157]: 3B ';'
    0x0004100c, 0x33000000, 0x003333cc,
    // [160]: 3C '<'
    0x00080e0a, 0x03030c0c, 0x3030c0c0, 0x30300c0c, 0x03030000,
    // [165]: 3D '='
    0x000a080e, 0xfffff000, 0x0000000f, 0xffff0000,
    // [169]: 3E '>'
    0x00080e0a, 0xc0c03030, 0x0c0c0303, 0x0c0c3030, 0xc0c00000,
    // [174]: 3F '?'
    0x000c1206, 0x3fc3fcc0, 0x3c03c03c, 0x0300c00c, 0x0300300c, 0x00c00c00, 0xc0000000, 0x0c00c000,
    // [182]: 40 '@'
    0x00101008, 0x0ff00ff0, 0x300c300c, 0xc3f3c3f3, 0xcc33cc33, 0xcc33cc33, 0xc3fcc3fc, 0x30003000,
    0x0fc00fc0,
    // [191]: 41 'A'
    0x000e1206, 0x03000c00, 0x3000c00c, 0xc03300cc, 0x03303030, 0xc0c3030c, 0x0cffffff, 0xfc00f003,
    0xc00f0030,
    // [200]: 42 'B'
    0x000c1206, 0xffcffcc0, 0x3c03c03c, 0x03ffcffc, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xffcffc00,
    // [208]: 43 'C'
    0x000c1206, 0x3fc3fcc0, 0x3c03c00c, 0x00c00c00, 0xc00c00c0, 0x0c00c00c, 0x00c03c03, 0x3fc3fc00,
    // [216]: 44 'D'
    0x000c1206, 0xff0ff0c0, 0xcc0cc03c, 0x03c03c03, 0xc03c03c0, 0x3c03c03c, 0x03c0cc0c, 0xff0ff000,
    // [224]: 45 'E'
    0x000a1206, 0xfffffc03, 0x00c0300c, 0x0300ff3f, 0xcc0300c0, 0x300c0300, 0xfffff000,
    // [231]: 46 'F'
    0x000a1206, 0xfffffc03, 0x00c0300c, 0x0300ff3f, 0xcc0300c0, 0x300c0300, 0xc0300000,
    // [238]: 47 'G'
    0x000c1206, 0x3fc3fcc0, 0x3c03c00c, 0x00c00c00, 0xc3fc3fc0, 0x3c03c03c, 0x03c03c03, 0x3fc3fc00,
    // [246]: 48 'H'
    0x000c1206, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xffffffc0, 0x3c03c03c, 0x03c03c03, 0xc03c0300,
    // [254]: 49 'I'
    0x00021206, 0xffffffff, 0xf0000000,
    // [257]: 4A 'J'
    0x000c1206, 0x00300300, 0x30030030, 0x03003003, 0x00300300, 0x3003c03c, 0x03c03c03, 0x3fc3fc00,
    // [265]: 4B 'K'
    0x000c1206, 0xc03c03c0, 0xcc0cc30c, 0x30cc0cc0, 0xf00f00cc, 0x0cc0c30c, 0x30c0cc0c, 0xc03c0300,
    // [273]: 4C 'L'
    0x000a1206, 0xc0300c03, 0x00c0300c, 0x0300c030, 0x0c0300c0, 0x300c0300, 0xfffff000,
    // [280]: 4D 'M'
    0x000e1206, 0xc00f003f, 0x03fc0fcc, 0xcf333c30, 0xf0c3c00f, 0x003c00f0, 0x03c00f00, 0x3c00f003,
    0xc00f0030,
    // [289]: 4E 'N'
    0x000c1206, 0xf03f03f0, 0x3f03cc3c, 0xc3cc3cc3, 0xc33c33c3, 0x3c33c0fc, 0x0fc0fc0f, 0xc03c0300,
    // [297]: 4F 'O'
    0x000c1206, 0x3fc3fcc0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0x3fc3fc00,
    // [305]: 50 'P'
    0x000c1206, 0xffcffcc0, 0x3c03c03c, 0x03c03c03, 0xffcffcc0, 0x0c00c00c, 0x00c00c00, 0xc00c0000,
    // [313]: 51 'Q'
    0x000c1606, 0x3fc3fcc0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0x3fc3fc03,
    0x003000f0, 0x0f000000,
    // [323]: 52 'R'
    0x000c1206, 0xffcffcc0, 0x3c03c03c, 0x03c03c03, 0xffcffcc3, 0x0c30c0cc, 0x0cc03c03, 0xc03c0300,
    // [331]: 53 'S'
    0x000c1206, 0x3fc3fcc0, 0x3c03c00c, 0x00c00c00, 0x3fc3fc00, 0x30030030, 0x03c03c03, 0x3fc3fc00,
    // [339]: 54 'T'
    0x000e1206, 0xfffffff0, 0x3000c003, 0x000c0030, 0x00c00300, 0x0c003000, 0xc003000c, 0x003000c0,
    0x03000c00,
    // [348]: 55 'U'
    0x000c1206, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0x3fc3fc00,
    // [356]: 56 'V'
    0x000e1206, 0xc00f003c, 0x00f003c0, 0x0f003303, 0x0c0c3030, 0xc0c0cc03, 0x300cc033, 0x003000c0,
    0x03000c00,
    // [365]: 57 'W'
    0x00121206, 0xc000f000, 0x3c000f00, 0x03c0c0f0, 0x30330c30, 0xc30c3333, 0x0cccc333, 0x30cccc0c,
    0x0c030300, 0xc0c03030, 0x0c0c0303, 0x00000000,
    // [377]: 58 'X'
    0x000a1206, 0xc0f03c0f, 0x03330cc3, 0x30cc0c03, 0x0330cc33, 0x0ccc0f03, 0xc0f03000,
    // [384]: 59 'Y'
    0x000a1206, 0xc0f03c0f, 0x03c0f033, 0x30cc330c, 0xc0c0300c, 0x0300c030, 0x0c030000,
    // [391]: 5A 'Z'
    0x000a1206, 0xfffff00c, 0x030300c0, 0x300c0c03, 0x0300c030, 0x0c0c0300, 0xfffff000,
    // [398]: 5B '['
    0x00041604, 0xffcccccc, 0xcccccccc, 0xccccff00,
    // [402]: 5C '\\'
    0x000a1404, 0xc0300c03, 0x00300c03, 0x00c00c03, 0x00c03003, 0x00c0300c, 0x00c0300c, 0x03000000,
    // [410]: 5D ']'
    0x00041604, 0xff333333, 0x33333333, 0x3333ff00,
    // [414]: 5E '^'
    0x00060806, 0x30c30ccf, 0x3cf30000,
    // [417]: 5F '_'
    0x00100216, 0xffffffff,
    // [419]: 60 '`'
    0x00040406, 0xcc330000,
    // [421]: 61 'a'
    0x000a0e0a, 0x3f0fcc0f, 0x0300c033, 0xfcffc0f0, 0x3c0f033f, 0xcff00000,
    // [427]: 62 'b'
    0x000a1206, 0xc0300c03, 0x00ff3fcc, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xff3fc000,
    // [434]: 63 'c'
    0x000a0e0a, 0x3f0fcc0f, 0x03c0300c, 0x0300c030, 0x0c0f033f, 0x0fc00000,
    // [440]: 64 'd'
    0x000a1206, 0x00c0300c, 0x033fcffc, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0x3fcff000,
    // [447]: 65 'e'
    0x000a0e0a, 0x3f0fcc0f, 0x03c0f03f, 0xffffc030, 0x0c0f033f, 0x0fc00000,
    // [453]: 66 'f'
    0x00081206, 0x0f0f3030, 0xfcfc3030, 0x30303030, 0x30303030, 0x30300000,
    // [459]: 67 'g'
    0x000a140a, 0x3fcffc0f, 0x03c0f03c, 0x0f03c0f0, 0x3c0f033f, 0xcff00c03, 0xc0f033f0, 0xfc000000,
    // [467]: 68 'h'
    0x000a1206, 0xc0300c03, 0x00cf33cf, 0x0fc3c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f03000,
    // [474]: 69 'i'
    0x00021206, 0xf0ffffff, 0xf0000000,
    // [477]: 6A 'j'
    0x00061806, 0x0c30000c, 0x30c30c30, 0xc30c30c3, 0x0c30c30c, 0x3f3c0000,
    // [483]: 6B 'k'
    0x000a1206, 0xc0300c03, 0x00c0f03c, 0x330ccc33, 0x0f03c0cc, 0x330c330c, 0xc0f03000,
    // [490]: 6C 'l'
    0x00021206, 0xffffffff, 0xf0000000,
    // [493]: 6D 'm'
    0x00120e0a, 0xcf0f33c3, 0xcf0f0fc3, 0xc3c0c0f0, 0x303c0c0f, 0x0303c0c0, 0xf0303c0c, 0x0f0303c0,
    0xc0f03030,
    // [502]: 6E 'n'
    0x000a0e0a, 0xcf33cf0f, 0xc3c0f03c, 0x0f03c0f0, 0x3c0f03c0, 0xf0300000,
    // [508]: 6F 'o'
    0x000a0e0a, 0x3f0fcc0f, 0x03c0f03c, 0x0f03c0f0, 0x3c0f033f, 0x0fc00000,
    // [514]: 70 'p'
    0x000a120a, 0xff3fcc0f, 0x03c0f03c, 0x0f03c0f0, 0x3c0f03ff, 0x3fcc0300, 0xc0300000,
    // [521]: 71 'q'
    0x000a120a, 0x3fcffc0f, 0x03c0f03c, 0x0f03c0f0, 0x3c0f033f, 0xcff00c03, 0x00c03000,
    // [528]: 72 'r'
    0x000a0e0a, 0xcff3ff03, 0xc0c0300c, 0x0300c030, 0x0c0300c0, 0x30000000,
    // [534]: 73 's'
    0x000a0e0a, 0x3f0fcc0f, 0x03c03003, 0xf0fc00c0, 0x3c0f033f, 0x0fc00000,
    // [540]: 74 't'
    0x00081206, 0x30303030, 0xfcfc3030, 0x30303030, 0x30303030, 0x0f0f0000,
    // [546]: 75 'u'
    0x000a0e0a, 0xc0f03c0f, 0x03c0f03c, 0x0f03c0f0, 0x3c3f0f3c, 0xcf300000,
    // [552]: 76 'v'
    0x000a0e0a, 0xc0f03c0f, 0x03c0f033, 0x30cc330c, 0xc0c0300c, 0x03000000,
    // [558]: 77 'w'
    0x00120e0a, 0xc0c0f030, 0x3c0c0f03, 0x0333330c, 0xccc33330, 0xcccc0c0c, 0x030300c0, 0xc030300c,
    0x0c030300,
    // [567]: 78 'x'
    0x000a0e0a, 0xc0f03c0f, 0x03330cc0, 0xc030330c, 0xcc0f03c0, 0xf0300000,
    // [573]: 79 'y'
    0x000c140a, 0x30330330, 0x33033033, 0x030cc0cc, 0x0cc0cc03, 0x00300300, 0x300c00c0, 0x0c00c0f0,
    0x0f000000,
    // [582]: 7A 'z'
    0x000a0e0a, 0xfffff00c, 0x030300c0, 0xc030300c, 0x0c0300ff, 0xfff00000,
    // [588]: 7B '{'
    0x00061604, 0x0c330c30, 0xc30c30cc, 0x3030c30c, 0x30c30c0c, 0x30000000,
    // [594]: 7C '|'
    0x00021604, 0xffffffff, 0xfff00000,
    // [597]: 7D '}'
    0x00061604, 0xc3030c30, 0xc30c30c0, 0xc330c30c, 0x30c30cc3, 0x00000000,
    // [603]: 7E '~'
    0x000c0406, 0x3c33c3c3, 0xcc3c0000,
    // [606]: A0 ' '
    0x0004020e, 0x00000000,
    // [608]: A1 '¡'
    0x00021206, 0xf0ffffff, 0xf0000000,
    // [611]: A2 '¢'
    0x000a1208, 0x0c0303f0, 0xfcccf33c, 0xc330cc33, 0x0cc330cc, 0xf333f0fc, 0x0c030000,
    // [618]: A3 '£'
    0x000c1206, 0x03c03c0c, 0x00c00c00, 0xc00c00c0, 0xff0ff030, 0x03003003, 0x00300300, 0xffffff00,
    // [626]: A4 '¤'
    0x000c1006, 0x30c30c0f, 0x00f030c3, 0x0cc03c03, 0xc03c0330, 0xc30c0f00, 0xf030c30c,
    // [633]: A5 '¥'
    0x000a1206, 0xc0f03330, 0xccfffff0, 0xc030ffff, 0xf0c0300c, 0x0300c030, 0x0c030000,
    // [640]: A6 '¦'
    0x00021604, 0xfffff0ff, 0xfff00000,
    // [643]: A7 '§'
    0x000c1606, 0x3fc3fcc0, 0x3c03c00c, 0x003c03c0, 0xc3cc3cc0, 0x3c033c33, 0xc303c03c, 0x003003c0,
    0x3c033fc3, 0xfc000000,
    // [653]: A8 '¨'
    0x00060206, 0xcf300000,
    // [655]: A9 '©'
    0x00101008, 0x0ff00ff0, 0x300c300c, 0xc3c3c3c3, 0xcc03cc03, 0xcc03cc03, 0xc3c3c3c3, 0x300c300c,
    0x0ff00ff0,
    // [664]: AA 'ª'
    0x00080e06, 0x3c3c0303, 0x3f3fc3c3, 0x3f3f0000, 0xffff0000,
    // [669]: AB '«'
    0x000c0a0e, 0x0c30c330, 0xc30cc30c, 0x3030c30c, 0x0c30c300,
    // [674]: AC '¬'
    0x000a060e, 0xfffff00c, 0x0300c030,
    // [677]: AD '­'
    0x000a020e, 0xfffff000,
    // [679]: AE '®'
    0x00101008, 0x0ff00ff0, 0x300c300c, 0xcfc3cfc3, 0xcc33cc33, 0xcfc3cfc3, 0xcc33cc33, 0x300c300c,
    0x0ff00ff0,
    // [688]: AF '¯'
    0x00060206, 0xfff00000,
    // [690]: B0 '°'
    0x00080806, 0x3c3cc3c3, 0xc3c33c3c,
    // [693]: B1 '±'
    0x000a0e0a, 0x0c0300c0, 0x30fffff0, 0xc0300c03, 0x000000ff, 0xfff00000,
    // [699]: B2 '²'
    0x00060a02, 0xfff0c3ff, 0xfc30fff0,
    // [702]: B3 '³'
    0x00060a02, 0xfff0c3ff, 0xf0c3fff0,
    // [705]: B4 '´'
    0x00040406, 0x33cc0000,
    // [707]: B5 'µ'
    0x0010120a, 0x03030303, 0x03030303, 0x0c0c0c0c, 0x0c0c0c0c, 0x0c0c0c0c, 0x30303030, 0x3fcc3fcc,
    0xc000c000, 0xc000c000,
    // [717]: B6 '¶'
    0x000e1606, 0x3ffcffff, 0xf33fccff, 0x33fccff3, 0x3fcc3f30, 0xfcc03300, 0xcc03300c, 0xc03300cc,
    0x03300cc0, 0x3300cc03, 0x300cc000,
    // [728]: B7 '·'
    0x00020210, 0xf0000000,
    // [730]: B8 '¸'
    0x00040618, 0xcc33ff00,
    // [732]: B9 '¹'
    0x00060a02, 0x30cf3c30, 0xc30cfff0,
    // [735]: BA 'º'
    0x00080e06, 0x3c3cc3c3, 0xc3c3c3c3, 0x3c3c0000, 0xffff0000,
    // [740]: BB '»'
    0x000c0a0e, 0xc30c3030, 0xc30c0c30, 0xc330c30c, 0xc30c3000,
    // [745]: BC '¼'
    0x00121402, 0x300c0c03, 0x0f00c3c0, 0x3030300c, 0x0c030300, 0xc0c0fcc0, 0x3f30000c, 0xcc033303,
    0x0cc0c330, 0x30fc0c3f, 0x0c00c300, 0x30c00c30, 0x03000000,
    // [758]: BD '½'
    0x00121402, 0x300c0c03, 0x0f00c3c0, 0x3030300c, 0x0c030300, 0xc0c0fcc0, 0x3f30000c, 0xfc033f03,
    0x00c0c030, 0x30fc0c3f, 0x0c0c0303, 0x00c0fc30, 0x3f000000,
    // [771]: BE '¾'
    0x00121402, 0xfc0c3f03, 0x00c0c030, 0x30fc303f, 0x0c00c300, 0x30c0fcc0, 0x3f30000c, 0xcc033303,
    0x0cc0c330, 0x30fc0c3f, 0x0c00c300, 0x30c00c30, 0x03000000,
    // [784]: BF '¿'
    0x000a1206, 0x0c030000, 0x000c0300, 0xc030300c, 0x0c0300c0, 0xf03c0f03, 0x3f0fc000,
    // [791]: C0 'À'
    0x000e1800, 0x0c003000, 0x3000c000, 0x00000030, 0x00c00300, 0x0c00cc03, 0x300cc033, 0x03030c0c,
    0x3030c0cf, 0xffffffc0, 0x0f003c00, 0xf0030000,
    // [803]: C1 'Á'
    0x000e1800, 0x00c00300, 0x3000c000, 0x00000030, 0x00c00300, 0x0c00cc03, 0x300cc033, 0x03030c0c,
    0x3030c0cf, 0xffffffc0, 0x0f003c00, 0xf0030000,
    // [815]: C2 'Â'
    0x000e1800, 0x03000c00, 0xcc033000, 0x00000030, 0x00c00300, 0x0c00cc03, 0x300cc033, 0x03030c0c,
    0x3030c0cf, 0xffffffc0, 0x0f003c00, 0xf0030000,
    // [827]: C3 'Ã'
    0x000e1800, 0x0f0c3c33, 0x0f0c3c00, 0x00000030, 0x00c00300, 0x0c00cc03, 0x300cc033, 0x03030c0c,
    0x3030c0cf, 0xffffffc0, 0x0f003c00, 0xf0030000,
    // [839]: C4 'Ä'
    0x000e1602, 0x0cc03300, 0x00000003, 0x000c0030, 0x00c00cc0, 0x3300cc03, 0x303030c0, 0xc3030c0c,
    0xfffffffc, 0x00f003c0, 0x0f003000,
    // [850]: C5 'Å'
    0x000e1602, 0x03000c00, 0xcc033003, 0x000c0030, 0x00c00cc0, 0x3300cc03, 0x303030c0, 0xc3030c0c,
    0xfffffffc, 0x00f003c0, 0x0f003000,
    // [861]: C6 'Æ'
    0x00141206, 0x00fff00f, 0xff033000, 0x33000330, 0x0033000c, 0x3000c300, 0x0fffc0ff, 0xfc303003,
    0x03003030, 0x030300c0, 0x300c0300, 0xc03ffc03, 0xff000000,
    // [874]: C7 'Ç'
    0x000c1806, 0x3fc3fcc0, 0x3c03c00c, 0x00c00c00, 0xc00c00c0, 0x0c00c00c, 0x00c03c03, 0x3fc3fc03,
    0x003000c0, 0x0c030030,
    // [884]: C8 'È'
    0x000a1800, 0x300c00c0, 0x3000000f, 0xffffc030, 0x0c0300c0, 0x300ff3fc, 0xc0300c03, 0x00c0300f,
    0xffff0000,
    // [893]: C9 'É'
    0x000a1800, 0x0300c0c0, 0x3000000f, 0xffffc030, 0x0c0300c0, 0x300ff3fc, 0xc0300c03, 0x00c0300f,
    0xffff0000,
    // [902]: CA 'Ê'
    0x000a1800, 0x0c030330, 0xcc00000f, 0xffffc030, 0x0c0300c0, 0x300ff3fc, 0xc0300c03, 0x00c0300f,
    0xffff0000,
    // [911]: CB 'Ë'
    0x000a1602, 0x330cc000, 0x00fffffc, 0x0300c030, 0x0c0300ff, 0x3fcc0300, 0xc0300c03, 0x00fffff0,
    // [919]: CC 'Ì'
    0x00041800, 0xcc330033, 0x33333333, 0x33333333,
    // [923]: CD 'Í'
    0x00041800, 0x33cc00cc, 0xcccccccc, 0xcccccccc,
    // [927]: CE 'Î'
    0x00061800, 0x30ccf300, 0x030c30c3, 0x0c30c30c, 0x30c30c30, 0xc30c0000,
    // [933]: CF 'Ï'
    0x00061602, 0xcf300030, 0xc30c30c3, 0x0c30c30c, 0x30c30c30, 0xc0000000,
    // [939]: D0 'Ð'
    0x000e1206, 0x3fc0ff03, 0x030c0c30, 0x0cc03300, 0xcc03fc0f, 0xf03300cc, 0x03300cc0, 0x33030c0c,
    0x3fc0ff00,
    // [948]: D1 'Ñ'
    0x000c1800, 0x3c33c3c3, 0xcc3c0000, 0x00f03f03, 0xf03f03cc, 0x3cc3cc3c, 0xc3c33c33, 0xc33c33c0,
    0xfc0fc0fc, 0x0fc03c03,
    // [958]: D2 'Ò'
    0x000c1800, 0x0c00c003, 0x00300000, 0x003fc3fc, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c03c03c, 0x033fc3fc,
    // [968]: D3 'Ó'
    0x000c1800, 0x0300300c, 0x00c00000, 0x003fc3fc, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c03c03c, 0x033fc3fc,
    // [978]: D4 'Ô'
    0x000c1800, 0x0c00c033, 0x03300000, 0x003fc3fc, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c03c03c, 0x033fc3fc,
    // [988]: D5 'Õ'
    0x000c1800, 0x3c33c3c3, 0xcc3c0000, 0x003fc3fc, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c03c03c, 0x033fc3fc,
    // [998]: D6 'Ö'
    0x000c1602, 0x30c30c00, 0x00003fc3, 0xfcc03c03, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c033fc3, 0xfc000000,
    // [1008]: D7 '×'
    0x000a0a0e, 0xc0f03330, 0xcc0c0303, 0x30ccc0f0, 0x30000000,
    // [1013]: D8 'Ø'
    0x00101206, 0x0ff30ff3, 0x300c300c, 0x303c303c, 0x30cc30cc, 0x330c330c, 0x3c0c3c0c, 0x300c300c,
    0xf00cf00c, 0x0ff00ff0,
    // [1023]: D9 'Ù'
    0x000c1800, 0x0c00c003, 0x00300000, 0x00c03c03, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c03c03c, 0x033fc3fc,
    // [1033]: DA 'Ú'
    0x000c1800, 0x0300300c, 0x00c00000, 0x00c03c03, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c03c03c, 0x033fc3fc,
    // [1043]: DB 'Û'
    0x000c1800, 0x0c00c033, 0x03300000, 0x00c03c03, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c03c03c, 0x033fc3fc,
    // [1053]: DC 'Ü'
    0x000c1602, 0x30c30c00, 0x0000c03c, 0x03c03c03, 0xc03c03c0, 0x3c03c03c, 0x03c03c03, 0xc03c03c0,
    0x3c033fc3, 0xfc000000,
    // [1063]: DD 'Ý'
    0x000a1800, 0x06018180, 0x6000000c, 0x0f03c0f0, 0x3c0f0333, 0x0cc330cc, 0x0c0300c0, 0x300c0300,
    0xc0300000,
    // [1072]: DE 'Þ'
    0x000a1206, 0xc0300c03, 0x00ff3fcc, 0x0f03c0f0, 0x3c0f03ff, 0x3fcc0300, 0xc0300000,
    // [1079]: DF 'ß'
    0x000c1206, 0x3f03f0c0, 0xcc0cc30c, 0x30cc0cc0, 0xcc0cc0c3, 0xcc3cc03c, 0x03c03c03, 0xcfccfc00,
    // [1087]: E0 'à'
    0x000a1404, 0x0c030030, 0x0c000003, 0xf0fcc0f0, 0x33fcffc0, 0xf03c0f03, 0xc0f033fc, 0xff000000,
    // [1095]: E1 'á'
    0x000a1404, 0x0c030300, 0xc0000003, 0xf0fcc0f0, 0x33fcffc0, 0xf03c0f03, 0xc0f033fc, 0xff000000,
    // [1103]: E2 'â'
    0x000a1404, 0x0c030330, 0xcc000003, 0xf0fcc0f0, 0x33fcffc0, 0xf03c0f03, 0xc0f033fc, 0xff000000,
    // [1111]: E3 'ã'
    0x000a1404, 0x3ccf3cf3, 0x3c000003, 0xf0fcc0f0, 0x33fcffc0, 0xf03c0f03, 0xc0f033fc, 0xff000000,
    // [1119]: E4 'ä'
    0x000a1206, 0x330cc000, 0x003f0fcc, 0x0f033fcf, 0xfc0f03c0, 0xf03c0f03, 0x3fcff000,
    // [1126]: E5 'å'
    0x000a1800, 0x3c0f0c33, 0x0cc330c3, 0xc0f00000, 0x03f0fcc0, 0xf033fcff, 0xc0f03c0f, 0x03c0f033,
    0xfcff0000,
    // [1135]: E6 'æ'
    0x00120e0a, 0x3f3f0fcf, 0xcc0c0f03, 0x0300c0c0, 0x3033fffc, 0xffffc0c0, 0x30300c0c, 0x0f03033f,
    0x3f0fcfc0,
    // [1144]: E7 'ç'
    0x000a120a, 0x3f0fcc0f, 0x03c0300c, 0x0300c030, 0x0c0f033f, 0x0fc0c030, 0x300c0000,
    // [1151]: E8 'è'
    0x000a1404, 0x300c00c0, 0x30000003, 0xf0fcc0f0, 0x3c0f03ff, 0xfffc0300, 0xc0f033f0, 0xfc000000,
    // [1159]: E9 'é'
    0x000a1404, 0x0c030300, 0xc0000003, 0xf0fcc0f0, 0x3c0f03ff, 0xfffc0300, 0xc0f033f0, 0xfc000000,
    // [1167]: EA 'ê'
    0x000a1404, 0x0c030330, 0xcc000003, 0xf0fcc0f0, 0x3c0f03ff, 0xfffc0300, 0xc0f033f0, 0xfc000000,
    // [1175]: EB 'ë'
    0x000a1206, 0x330cc000, 0x003f0fcc, 0x0f03c0f0, 0x3fffffc0, 0x300c0f03, 0x3f0fc000,
    // [1182]: EC 'ì'
    0x00041404, 0xcc330033, 0x33333333, 0x33330000,
    // [1186]: ED 'í'
    0x00041404, 0x33cc00cc, 0xcccccccc, 0xcccc0000,
    // [1190]: EE 'î'
    0x00061404, 0x30ccf300, 0x030c30c3, 0x0c30c30c, 0x30c30c00,
    // [1195]: EF 'ï'
    0x00061206, 0xcf300030, 0xc30c30c3, 0x0c30c30c, 0x30c00000,
    // [1200]: F0 'ð'
    0x000a1305, 0xc03c0330, 0xfcfc33c0, 0xf0c330f0, 0x3c0f03c0, 0xf03c0f0c, 0xc30f03c0,
    // [1207]: F1 'ñ'
    0x000a1404, 0x3ccf3cf3, 0x3c00000c, 0xf33cf0fc, 0x3c0f03c0, 0xf03c0f03, 0xc0f03c0f, 0x03000000,
    // [1215]: F2 'ò'
    0x000a1404, 0x300c00c0, 0x30000003, 0xf0fcc0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [1223]: F3 'ó'
    0x000a1404, 0x0300c0c0, 0x30000003, 0xf0fcc0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [1231]: F4 'ô'
    0x000a1404, 0x0c030330, 0xcc000003, 0xf0fcc0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [1239]: F5 'õ'
    0x000a1404, 0x3ccf3cf3, 0x3c000003, 0xf0fcc0f0, 0x3c0f03c0, 0xf03c0f03, 0xc0f033f0, 0xfc000000,
    // [1247]: F6 'ö'
    0x000a1206, 0x330cc000, 0x003f0fcc, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0x3f0fc000,
    // [1254]: F7 '÷'
    0x000a0a0a, 0x0c030000, 0x00fffff0, 0x00000c03, 0x00000000,
    // [1259]: F8 'ø'
    0x000e0e0a, 0x0fcc3f33, 0x030c0c30, 0xf0c3c333, 0x0ccc3c30, 0xf0c3030c, 0x0ccfc33f, 0x00000000,
    // [1267]: F9 'ù'
    0x000a1404, 0x300c00c0, 0x3000000c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc3f0f3cc, 0xf3000000,
    // [1275]: FA 'ú'
    0x000a1404, 0x0300c0c0, 0x3000000c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc3f0f3cc, 0xf3000000,
    // [1283]: FB 'û'
    0x000a1404, 0x0c030330, 0xcc00000c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xc3f0f3cc, 0xf3000000,
    // [1291]: FC 'ü'
    0x000a1206, 0x330cc000, 0x00c0f03c, 0x0f03c0f0, 0x3c0f03c0, 0xf03c3f0f, 0x3ccf3000,
    // [1298]: FD 'ý'
    0x000c1a04, 0x0300300c, 0x00c00000, 0x00303303, 0x30330330, 0x33030cc0, 0xcc0cc0cc, 0x03003003,
    0x00300c00, 0xc00c00c0, 0xf00f0000,
    // [1309]: FE 'þ'
    0x000a1606, 0xc0300c03, 0x00ff3fcc, 0x0f03c0f0, 0x3c0f03c0, 0xf03c0f03, 0xff3fcc03, 0x00c03000,
    // [1317]: FF 'ÿ'
    0x000c1806, 0x0cc0cc00, 0x00003033, 0x03303303, 0x3033030c, 0xc0cc0cc0, 0xcc030030, 0x0300300c,
    0x00c00c00, 0xc0f00f00,
    // [1327]: 152 'Œ'
    0x00141206, 0x3fcff3fc, 0xffc0300c, 0x0300c030, 0x0c0300c0, 0x300c0300, 0xc03fcc03, 0xfcc0300c,
    0x0300c030, 0x0c0300c0, 0x300c0300, 0x3fcff3fc, 0xff000000,
    // [1340]: 153 'œ'
    0x00120e0a, 0x3f3f0fcf, 0xcc0c0f03, 0x03c0c0f0, 0x303c0fff, 0x03ffc0c0, 0x30300c0c, 0x0f03033f,
    0x3f0fcfc0,
    // [1349]: 2018 '‘'
    0x00040604, 0x33cccc00,
    // [1351]: 2019 '’'
    0x00040604, 0x3333cc00,
    // [1353]: 201A '‚'
    0x00040616, 0x3333cc00,
    // [1355]: 201B '‛'
    0x00040604, 0xcccc3300,
    // [1357]: 201C '“'
    0x000a0604, 0x30cc3c33, 0x0cc330c0,
    // [1360]: 201D '”'
    0x000a0604, 0x30cc330c, 0xc3c330c0,
    // [1363]: 201E '„'
    0x000a0616, 0x30cc330c, 0xc3c330c0,
    // [1366]: 201F '‟'
    0x000a0604, 0xc330cc33, 0x0c30cc30,
    // [1369]: 2020 '†'
    0x000a1206, 0x0c0300c0, 0x30fffff0, 0xc0300c03, 0x00c0300c, 0x0300c030, 0x0c030000,
    // [1376]: 2021 '‡'
    0x000a1606, 0x0c0300c0, 0x30fffff0, 0xc0300c03, 0x00c0300c, 0x0300c030, 0xfffff0c0, 0x300c0300,
    // [1384]: 2022 '•'
    0x000c0c0a, 0x3fc3fcff, 0xffffffff, 0xffffffff, 0xffffff3f, 0xc3fc0000,
    // [1390]: 20AC '€'
    0x00101008, 0x03fc03fc, 0x0c030c03, 0x30003000, 0xfff0fff0, 0x30003000, 0xfff0fff0, 0x0c030c03,
    0x03fc03fc,
    // [1399]: FFFD '�'
    0x00121404, 0x00c00030, 0x003f000f, 0xc00f3c03, 0xcf03ccf0, 0xf33cffcf, 0xfff3fff3, 0xfffcff3f,
    0xff0fffc0, 0xf3c03cf0, 0x03f000fc, 0x000c0003, 0x00000000,
];
