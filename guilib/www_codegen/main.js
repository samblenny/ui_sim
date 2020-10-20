"use strict";

const canvas = document.querySelector('#canvas');
const preOut = document.querySelector('#preOut');
const preOut2 = document.querySelector('#preOut2');
const bitmap = document.querySelector('#bitmap');
const imgSelect = document.querySelector('#imgSelect');
const ctx = canvas.getContext('2d');

window.addEventListener('load', init);

function init(e) {
    paintPngToCanvas();
    // Configure bitmap selector
    imgSelect.addEventListener('change', e => {
        bitmap.onload = paintPngToCanvas;
        bitmap.src = e.target.value;
    });
    // Configure canvas click detector
    canvas.addEventListener('click', doCanvasClick);
}

function paintPngToCanvas() {
    let h = bitmap.height;
    let w = bitmap.width;
    canvas.height = h;
    canvas.width = w;
    ctx.drawImage(bitmap, 0, 0, w, h);
    preOut.textContent = "";
    // Render charmap table
    renderCharMap();
}

function doCanvasClick(e) {
    let bbox = e.target.getBoundingClientRect();
    let x = e.clientX - Math.floor(bbox.left);
    let y = e.clientY - Math.floor(bbox.top);
    const border = 2;
    const columns = 16;
    // Grid is 16x16 boxes of 24x24px or 30x30px with 2px gutters and borders
    let gridSize = Math.floor((canvas.width - border) / columns);
    // Determine which grid box contains the click coordinate
    let row = Math.floor(y/gridSize);
    let col = Math.floor(x/gridSize);
    let maxTrim = getMaxTrim(row, col, gridSize);
    let [pxMatrix, yOffset] = convertGlyphBoxToMatrix(row, col, maxTrim);
    preOut.textContent = convertMatrixToText(pxMatrix, yOffset, row, col);
}

function renderCharMap() {
    let data_buf = [];
    let str_buf = [];
    const border = 2;
    const columns = 16;
    let gridSize = Math.floor((canvas.width - border) / columns);
    for (let k of Object.keys(charMap).sort((a,b) => a-b)) {
        let v = charMap[k];
        let isUISprite = (0xE000 <= k) && (k <= 0xF8FF);
        if (isUISprite && !['img/bold.png', 'img/regular.png'].includes(imgSelect.value)) {
            // Skip sprites for small.png
            v.start = null;
            continue;
        }
        v.start = data_buf.length;
        let maxTrim = getMaxTrim(v.row, v.col, gridSize);
        let [matrix, yOffset] = convertGlyphBoxToMatrix(v.row, v.col, maxTrim);
        let pattern = convertMatrixToPattern(matrix, yOffset);
        Array.prototype.push.apply(data_buf, pattern);
        let description = '';
        if (isUISprite) {
            // Unicode Private Use Area block (system fonts won't have a glyph)
            description = v.name;
        } else {
            // Regular Unicode blocks (ok to assume system font has glyph)
            if (v.chr === "\\") {
                description = "'\\\\'";
            } else if (v.chr === "'") {
                description = "'\\''";
            } else {
                description = `'${v.chr}'`;
            }
        }
        let comment = `[${v.start}]: ${v.hex} ${description}`;
        let rust = convertPatternToRust(pattern, comment);
        str_buf.push(rust);
    }
    let fontName = {
        'img/bold.png': 'Bold',
        'img/regular.png': 'Regular',
        'img/small.png': 'Small'
    }[imgSelect.value];
    let rustCode = `
#![allow(dead_code)]
//! ${fontName} Font

${buildCharMapIndex()}

/// Maximum height of glyph patterns in this bitmap typeface.
/// This will be true: h + yOffset <= MAX_HEIGHT
pub const MAX_HEIGHT: u8 = ${gridSize-border};

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
pub const DATA: [u32; ${data_buf.length}] = [
${str_buf.join("\n")}
];
`;
    preOut2.textContent = rustCode.trim();
}

// Return rust source for lookup table from char code to glyph pattern offset
// in DATA array
function buildCharMapIndex() {
    // Unicode Blocks
    let basicLatin = [];         // Block:     00..7E; Subset:     20..7E
    let latin1 = [];             // Block:     80..FF; Subset:     A0..FF
    let latinExtendedA = [];     // Block:   100..17F; Subset:   152..153
    let generalPunctuation = []; // Block: 2000..206F; Subset: 2018..2022
    let currencySymbols = [];    // Block: 20A0..20CF; Subset: 20AC..20AC
    let privateUseArea = [];     // Block: E000..F8FF; Subset: E700..E70C
    let specials = [];           // Block: FFF0..FFFF; Subset: FFFD..FFFD
    for (let k of Object.keys(charMap).sort((a,b) => a-b)) {
        let v = charMap[k];
        if (v.start === null) {
            continue;
        }
        if (0x20 <= k && k <= 0x7E) {
            basicLatin[k-0x20] = v;
        } else if (0xA0 <= k && k <= 0xFF) {
            latin1[k-0xA0] = v;
        } else if (0x152 <= k && k <= 0x153) {
            latinExtendedA[k-0x152] = v;
        } else if (0x2018 <= k && k <= 0x2022) {
            generalPunctuation[k-0x2018] = v;
        } else if (0x20AC <= k && k <= 0x20AC) {
            currencySymbols[k-0x20AC] = v;
        } else if (0xE700 <= k && k <= 0xE70C) {
            privateUseArea[k-0xE700] = v;
        } else if (0xFFFD <= k && k <= 0xFFFD) {
            specials[k-0xFFFD] = v;
        }
    }
    let puaIndexStr = privateUseArea.length<1 ? '' : `

// Index to Unicode Private Use Area block glyph patterns (UI sprites)
const PRIVATE_USE_AREA: [u16; ${privateUseArea.length}] = [
    ${privateUseArea.map(v => v.start + ", // " + v.name).join("\n    ")}
];`;
    let puaMatchStr = privateUseArea.length<1 ? '' : `
        0xE700..=0xE70C => PRIVATE_USE_AREA[(c as usize) - 0xE700] as usize,`;
    let indexStr = `
/// Return offset into DATA[] for start of pattern depicting glyph for character c
pub fn get_glyph_pattern_offset(c: char) -> usize {
    match c as u32 {
        0x20..=0x7E => BASIC_LATIN[(c as usize) - 0x20] as usize,
        0xA0..=0xFF => LATIN_1[(c as usize) - 0xA0] as usize,
        0x152..=0x153 => LATIN_EXTENDED_A[(c as usize) - 0x152] as usize,
        0x2018..=0x2022 => GENERAL_PUNCTUATION[(c as usize) - 0x2018] as usize,
        0x20AC..=0x20AC => CURRENCY_SYMBOLS[(c as usize) - 0x20AC] as usize,${puaMatchStr}
        _ => SPECIALS[(0xFFFD as usize) - 0xFFFD] as usize,
    }
}

// Index to Unicode Basic Latin block glyph patterns
const BASIC_LATIN: [u16; ${basicLatin.length}] = [
    ${basicLatin.map(v => v.start + ", // '" + v.chr + "'").join("\n    ")}
];

// Index to Unicode Latin 1 block glyph patterns
const LATIN_1: [u16; ${latin1.length}] = [
    ${latin1.map(v => v.start + ", // '" + v.chr + "'").join("\n    ")}
];

// Index to Unicode Latin Extended A block glyph patterns
const LATIN_EXTENDED_A: [u16; ${latinExtendedA.length}] = [
    ${latinExtendedA.map(v => v.start + ", // '" + v.chr + "'").join("\n    ")}
];

// Index to General Punctuation block glyph patterns
const GENERAL_PUNCTUATION: [u16; ${generalPunctuation.length}] = [
    ${generalPunctuation.map(v => v.start + ", // '" + v.chr + "'").join("\n    ")}
];

// Index to Unicode Currency Symbols block glyph patterns
const CURRENCY_SYMBOLS: [u16; ${currencySymbols.length}] = [
    ${currencySymbols.map(v => v.start + ", // '" + v.chr +"'").join("\n    ")}
];${puaIndexStr}

// Index to Unicode Specials block glyph patterns
const SPECIALS: [u16; ${specials.length}] = [
    ${specials.map(v => v.start + ", // '" + v.chr + "'").join("\n    ")}
];
`;
    return indexStr.trim();
}

// Look up trim limits based on row & column in glyph grid
function getMaxTrim(row, col, gridSize) {
    // Radio strength bars get trimmed to match bounds of three bars
    if (col === 0 && [5, 6, 7, 8, 9].includes(row)) {
        return [7, 5, 6, 4];
    }
    // Space gets 4px width and 2px height
    if (col === 2 && row === 0) {
        return [Math.floor((gridSize-3)/2), Math.floor((gridSize-5)/2)];
    }
    // Everything else gets default trim
    return null;
}

// Extract matrix of pixels from an image containing grid of glyphs
// - row: source row in glyph grid
// - col: source column in glyph grid
// - maxTrim: [top right bottom left] upper limits in pixels of whitespace to
//   trim from border around glyph in grid cell
// Trim limits allow creation of patterns with whitespace borders, useful for
// purposes like making the radio strength sprites have the same size pattern.
function convertGlyphBoxToMatrix(row, col, maxTrim) {
    const border = 2;
    const columns = 16;
    const rows = columns;
    if (row < 0 || row >= rows || col < 0 || col >= columns) {
        // Ignore clicks on grid border, etc.
        return;
    }
    let w = canvas.width;
    let h = canvas.height;
    var idat = ctx.getImageData(0, 0, w, h);
    // Get pixels for grid cell, converting from RGBA to 1-bit
    let grid = Math.floor((w - border) / columns);
    let pxMatrix = [];
    for (let y=(row*grid)+border; y<(row+1)*grid; y++) {
        let row = [];
        for (let x=(col*grid)+border; x<(col+1)*grid; x++) {
            let offset = ((w * y) + x) * 4;
            let r = idat.data[offset];
            row.push(r>0 ? 0 : 1);
        }
        pxMatrix.push(row);
    }
    // Use default trim limits if none were given and expand partial
    // top/right/bottom/left trim bounds in the manner of css margins
    if (maxTrim === null || maxTrim === undefined || maxTrim.length < 1) {
        maxTrim = [h, w, h, w];
    } else if (maxTrim.length === 1) {
        let [t] = maxTrim;
        maxTrim = [t, t, t, t];
    } else if (maxTrim.length === 2) {
        let [t, r] = maxTrim;
        maxTrim = [t, r, t, r];
    } else if (maxTrim.length === 3) {
        let [t, r, b] = maxTrim;
        maxTrim = [t, r, b, r];
    }
    // Trim left whitespace
    pxMatrix = matrixTranspose(pxMatrix);
    let limit = maxTrim[3];
    trimLeadingEmptyRows(pxMatrix, limit);
    // Trim right whitespace
    pxMatrix.reverse();
    limit = maxTrim[1];
    trimLeadingEmptyRows(pxMatrix, limit);
    pxMatrix.reverse();
    pxMatrix = matrixTranspose(pxMatrix);
    // Trim top whitespace and calculate y-offset
    let preTrimH = pxMatrix.length;
    limit = maxTrim[0];
    trimLeadingEmptyRows(pxMatrix, limit);
    let yOffset = preTrimH - pxMatrix.length;
    // Trim bottom whitespace
    pxMatrix.reverse();
    limit = maxTrim[2];
    trimLeadingEmptyRows(pxMatrix, limit);
    pxMatrix.reverse();
    // Return matrix and yOffset
    return [pxMatrix, yOffset];
}

// Trim whitespace rows from top of matrix, subject to limit
// Side-effect: may change pxMatrix
function trimLeadingEmptyRows(pxMatrix, limit) {
    for (let i=0; i<limit; i++) {
        if (pxMatrix.length>0 && pxMatrix[0].reduce((a, b) => a+b) == 0) {
            pxMatrix.shift();
        } else {
            break;
        }
    }
}

// Return glyph as text with one ASCII char per pixel
function convertMatrixToText(pxMatrix, yOffset, row, col) {
    let ascii = pxMatrix.map(
        row => row.map(col => col==0 ? "." : "#").join("")
    ).join("\n");
    let rowCol = `row=${row}, col=${col}`;
    let pxW = pxMatrix.length>0 ? pxMatrix[0].length : 0;
    let size = `w=${pxW}, h=${pxMatrix.length}, yOffset=${yOffset}`;
    return `${ascii}\n\n${rowCol}, ${size}`;
}

// Return pixel matrix as pattern packed into a byte array
// pat[0]: pattern width in px
// pat[1]: pattern height in px
// pat[2]: y-offset from top of line (position properly relative to text baseline)
// pat[3..2+w*h/8]: pixels packed into bytes
function convertMatrixToPattern(pxMatrix, yOffset) {
    // Pack trimmed pattern into a byte array
    let patW = 0;
    let patH = 0;
    if (pxMatrix.length>0 && pxMatrix[0].length>0) {
        patW = pxMatrix[0].length;
        patH = pxMatrix.length;
    }
    let pattern = [(patW << 16) | (patH << 8) | yOffset];
    let bufWord = 0;
    let flushed = false;
    for (let y=0; y<patH; y++) {
        for (let x=0; x<patW; x++) {
            bufWord = (bufWord << 1) | (pxMatrix[y][patW-1-x]>0 ? 1 : 0);
            flushed = false;
            if ((y*patW + x) % 32 == 31) {
                pattern.push(bufWord);
                bufWord = 0;
                flushed = true;
            }
        }
    }
    if (!flushed) {
        let finalShift = 32 - ((patW * patH) % 32);
        pattern.push(bufWord << finalShift);
    }
    return pattern
}

// Convert pattern to rust source code for part of an array of bytes
function convertPatternToRust(pattern, comment) {
    let patternStr = `    // ${comment}\n    `;
    let wordsPerRow = 8;
    for (let i=0; i<Math.ceil(pattern.length/wordsPerRow); i++) {
        let start = i*wordsPerRow;
        let end = Math.min(pattern.length, (i+1)*wordsPerRow);
        let line = pattern.slice(start, end);
        patternStr += line.map(n => `0x${toHexWord(n)}`).join(", ") + ",";
        if (end < pattern.length) {
            patternStr += "\n    ";
        }
    }
    return  patternStr;
}

// Convert number to 8-digit hexidecimal string with left zero padding if needed
function toHexWord(n) {
    // Unsigned right shift (>>> 0) casts its answer to an unsigned 32-bit
    // integer which ensures toString(16) gives an unsigned result
    let hex = (n >>> 0).toString(16);
    let pad = Math.max(8-hex.length, 0);
    return "0".repeat(pad) + hex;
}

function matrixTranspose(matrix) {
    if (matrix.length < 1) {
        return matrix;
    }
    let w = matrix[0].length;
    let h = matrix.length;
    let transposed = [];
    for (let col=0; col<w; col++) {
        let trRow = [];
        for (let row=0; row<h; row++) {
            trRow.push(matrix[row][col]);
        }
        transposed.push(trRow);
    }
    return transposed;
}

var charMap = {
    32: {row: 0, col: 2, hex: '20', chr: ' '},
    33: {row: 1, col: 2, hex: '21', chr: '!'},
    34: {row: 2, col: 2, hex: '22', chr: '\"'},
    35: {row: 3, col: 2, hex: '23', chr: '#'},
    36: {row: 4, col: 2, hex: '24', chr: '$'},
    37: {row: 5, col: 2, hex: '25', chr: '%'},
    38: {row: 6, col: 2, hex: '26', chr: '&'},
    39: {row: 7, col: 2, hex: '27', chr: '\''},
    40: {row: 8, col: 2, hex: '28', chr: '('},
    41: {row: 9, col: 2, hex: '29', chr: ')'},
    42: {row:10, col: 2, hex: '2A', chr: '*'},
    43: {row:11, col: 2, hex: '2B', chr: '+'},
    44: {row:12, col: 2, hex: '2C', chr: ','},
    45: {row:13, col: 2, hex: '2D', chr: '-'},
    46: {row:14, col: 2, hex: '2E', chr: '.'},
    47: {row:15, col: 2, hex: '2F', chr: '/'},
    48: {row: 0, col: 3, hex: '30', chr: '0'},
    49: {row: 1, col: 3, hex: '31', chr: '1'},
    50: {row: 2, col: 3, hex: '32', chr: '2'},
    51: {row: 3, col: 3, hex: '33', chr: '3'},
    52: {row: 4, col: 3, hex: '34', chr: '4'},
    53: {row: 5, col: 3, hex: '35', chr: '5'},
    54: {row: 6, col: 3, hex: '36', chr: '6'},
    55: {row: 7, col: 3, hex: '37', chr: '7'},
    56: {row: 8, col: 3, hex: '38', chr: '8'},
    57: {row: 9, col: 3, hex: '39', chr: '9'},
    58: {row:10, col: 3, hex: '3A', chr: ':'},
    59: {row:11, col: 3, hex: '3B', chr: ';'},
    60: {row:12, col: 3, hex: '3C', chr: '<'},
    61: {row:13, col: 3, hex: '3D', chr: '='},
    62: {row:14, col: 3, hex: '3E', chr: '>'},
    63: {row:15, col: 3, hex: '3F', chr: '?'},
    64: {row: 0, col: 4, hex: '40', chr: '@'},
    65: {row: 1, col: 4, hex: '41', chr: 'A'},
    66: {row: 2, col: 4, hex: '42', chr: 'B'},
    67: {row: 3, col: 4, hex: '43', chr: 'C'},
    68: {row: 4, col: 4, hex: '44', chr: 'D'},
    69: {row: 5, col: 4, hex: '45', chr: 'E'},
    70: {row: 6, col: 4, hex: '46', chr: 'F'},
    71: {row: 7, col: 4, hex: '47', chr: 'G'},
    72: {row: 8, col: 4, hex: '48', chr: 'H'},
    73: {row: 9, col: 4, hex: '49', chr: 'I'},
    74: {row:10, col: 4, hex: '4A', chr: 'J'},
    75: {row:11, col: 4, hex: '4B', chr: 'K'},
    76: {row:12, col: 4, hex: '4C', chr: 'L'},
    77: {row:13, col: 4, hex: '4D', chr: 'M'},
    78: {row:14, col: 4, hex: '4E', chr: 'N'},
    79: {row:15, col: 4, hex: '4F', chr: 'O'},
    80: {row: 0, col: 5, hex: '50', chr: 'P'},
    81: {row: 1, col: 5, hex: '51', chr: 'Q'},
    82: {row: 2, col: 5, hex: '52', chr: 'R'},
    83: {row: 3, col: 5, hex: '53', chr: 'S'},
    84: {row: 4, col: 5, hex: '54', chr: 'T'},
    85: {row: 5, col: 5, hex: '55', chr: 'U'},
    86: {row: 6, col: 5, hex: '56', chr: 'V'},
    87: {row: 7, col: 5, hex: '57', chr: 'W'},
    88: {row: 8, col: 5, hex: '58', chr: 'X'},
    89: {row: 9, col: 5, hex: '59', chr: 'Y'},
    90: {row:10, col: 5, hex: '5A', chr: 'Z'},
    91: {row:11, col: 5, hex: '5B', chr: '['},
    92: {row:12, col: 5, hex: '5C', chr: '\\'},
    93: {row:13, col: 5, hex: '5D', chr: ']'},
    94: {row:14, col: 5, hex: '5E', chr: '^'},
    95: {row:15, col: 5, hex: '5F', chr: '_'},
    96: {row: 0, col: 6, hex: '60', chr: '`'},
    97: {row: 1, col: 6, hex: '61', chr: 'a'},
    98: {row: 2, col: 6, hex: '62', chr: 'b'},
    99: {row: 3, col: 6, hex: '63', chr: 'c'},
    100: {row: 4, col: 6, hex: '64', chr: 'd'},
    101: {row: 5, col: 6, hex: '65', chr: 'e'},
    102: {row: 6, col: 6, hex: '66', chr: 'f'},
    103: {row: 7, col: 6, hex: '67', chr: 'g'},
    104: {row: 8, col: 6, hex: '68', chr: 'h'},
    105: {row: 9, col: 6, hex: '69', chr: 'i'},
    106: {row:10, col: 6, hex: '6A', chr: 'j'},
    107: {row:11, col: 6, hex: '6B', chr: 'k'},
    108: {row:12, col: 6, hex: '6C', chr: 'l'},
    109: {row:13, col: 6, hex: '6D', chr: 'm'},
    110: {row:14, col: 6, hex: '6E', chr: 'n'},
    111: {row:15, col: 6, hex: '6F', chr: 'o'},
    112: {row: 0, col: 7, hex: '70', chr: 'p'},
    113: {row: 1, col: 7, hex: '71', chr: 'q'},
    114: {row: 2, col: 7, hex: '72', chr: 'r'},
    115: {row: 3, col: 7, hex: '73', chr: 's'},
    116: {row: 4, col: 7, hex: '74', chr: 't'},
    117: {row: 5, col: 7, hex: '75', chr: 'u'},
    118: {row: 6, col: 7, hex: '76', chr: 'v'},
    119: {row: 7, col: 7, hex: '77', chr: 'w'},
    120: {row: 8, col: 7, hex: '78', chr: 'x'},
    121: {row: 9, col: 7, hex: '79', chr: 'y'},
    122: {row:10, col: 7, hex: '7A', chr: 'z'},
    123: {row:11, col: 7, hex: '7B', chr: '{'},
    124: {row:12, col: 7, hex: '7C', chr: '|'},
    125: {row:13, col: 7, hex: '7D', chr: '}'},
    126: {row:14, col: 7, hex: '7E', chr: '~'},

    160: {row: 0, col: 2, hex: 'A0', chr: '\xA0'},  // No-Break Space
    161: {row: 1, col:12, hex: 'A1', chr: '¡'},
    162: {row: 2, col:10, hex: 'A2', chr: '¢'},
    163: {row: 3, col:10, hex: 'A3', chr: '£'},
    164: {row:15, col: 1, hex: 'A4', chr: '¤'},
    165: {row: 4, col:11, hex: 'A5', chr: '¥'},
    166: {row:15, col: 7, hex: 'A6', chr: '¦'},
    167: {row: 4, col:10, hex: 'A7', chr: '§'},
    168: {row:12, col:10, hex: 'A8', chr: '¨'},
    169: {row: 9, col:10, hex: 'A9', chr: '©'},
    170: {row:11, col:11, hex: 'AA', chr: 'ª'},
    171: {row: 7, col:12, hex: 'AB', chr: '«'},
    172: {row: 2, col:12, hex: 'AC', chr: '¬'},
    173: {row:13, col: 2, hex: 'AD', chr: '\xAD'},  // Soft Hyphen
    174: {row: 8, col:10, hex: 'AE', chr: '®'},
    175: {row: 8, col:15, hex: 'AF', chr: '¯'},  // Macron
    176: {row: 1, col:10, hex: 'B0', chr: '°'},  // Degree Sign
    177: {row: 1, col:11, hex: 'B1', chr: '±'},
    178: {row: 3, col: 1, hex: 'B2', chr: '²'},
    179: {row: 4, col: 1, hex: 'B3', chr: '³'},
    180: {row:11, col:10, hex: 'B4', chr: '´'},
    181: {row: 5, col:11, hex: 'B5', chr: 'µ'},
    182: {row: 6, col:10, hex: 'B6', chr: '¶'},
    183: {row: 1, col:14, hex: 'B7', chr: '·'},
    184: {row:12, col:15, hex: 'B8', chr: '¸'},  // Cedillia
    185: {row: 2, col: 1, hex: 'B9', chr: '¹'},
    186: {row:12, col:11, hex: 'BA', chr: 'º'},
    187: {row: 8, col:12, hex: 'BB', chr: '»'},
    188: {row: 5, col: 1, hex: 'BC', chr: '¼'},
    189: {row: 6, col: 1, hex: 'BD', chr: '½'},
    190: {row: 7, col: 1, hex: 'BE', chr: '¾'},
    191: {row: 0, col:12, hex: 'BF', chr: '¿'},
    192: {row:11, col:12, hex: 'C0', chr: 'À'},
    193: {row: 7, col:14, hex: 'C1', chr: 'Á'},
    194: {row: 5, col:14, hex: 'C2', chr: 'Â'},
    195: {row:12, col:12, hex: 'C3', chr: 'Ã'},
    196: {row: 0, col: 8, hex: 'C4', chr: 'Ä'},
    197: {row: 1, col: 8, hex: 'C5', chr: 'Å'},
    198: {row:14, col:10, hex: 'C6', chr: 'Æ'},
    199: {row: 2, col: 8, hex: 'C7', chr: 'Ç'},
    200: {row: 9, col:14, hex: 'C8', chr: 'È'},
    201: {row: 3, col: 8, hex: 'C9', chr: 'É'},
    202: {row: 6, col:14, hex: 'CA', chr: 'Ê'},
    203: {row: 8, col:14, hex: 'CB', chr: 'Ë'},
    204: {row:13, col:14, hex: 'CC', chr: 'Ì'},
    205: {row:10, col:14, hex: 'CD', chr: 'Í'},
    206: {row:11, col:14, hex: 'CE', chr: 'Î'},
    207: {row:12, col:14, hex: 'CF', chr: 'Ï'},
    208: {row: 8, col: 1, hex: 'D0', chr: 'Ð'},
    209: {row: 4, col: 8, hex: 'D1', chr: 'Ñ'},
    210: {row: 1, col:15, hex: 'D2', chr: 'Ò'},
    211: {row:14, col:14, hex: 'D3', chr: 'Ó'},
    212: {row:15, col:14, hex: 'D4', chr: 'Ô'},
    213: {row:13, col:12, hex: 'D5', chr: 'Õ'},
    214: {row: 5, col: 8, hex: 'D6', chr: 'Ö'},
    215: {row: 9, col: 1, hex: 'D7', chr: '×'},  // Multiplication Sign
    216: {row:15, col:10, hex: 'D8', chr: 'Ø'},
    217: {row: 4, col:15, hex: 'D9', chr: 'Ù'},
    218: {row: 2, col:15, hex: 'DA', chr: 'Ú'},
    219: {row: 3, col:15, hex: 'DB', chr: 'Û'},
    220: {row: 6, col: 8, hex: 'DC', chr: 'Ü'},
    221: {row:10, col: 1, hex: 'DD', chr: 'Ý'},
    222: {row:11, col: 1, hex: 'DE', chr: 'Þ'},
    223: {row: 7, col:10, hex: 'DF', chr: 'ß'},
    224: {row: 8, col: 8, hex: 'E0', chr: 'à'},
    225: {row: 7, col: 8, hex: 'E1', chr: 'á'},
    226: {row: 9, col: 8, hex: 'E2', chr: 'â'},
    227: {row:11, col: 8, hex: 'E3', chr: 'ã'},
    228: {row:10, col: 8, hex: 'E4', chr: 'ä'},
    229: {row:12, col: 8, hex: 'E5', chr: 'å'},
    230: {row:14, col:11, hex: 'E6', chr: 'æ'},
    231: {row:13, col: 8, hex: 'E7', chr: 'ç'},
    232: {row:15, col: 8, hex: 'E8', chr: 'è'},
    233: {row:14, col: 8, hex: 'E9', chr: 'é'},
    234: {row: 0, col: 9, hex: 'EA', chr: 'ê'},
    235: {row: 1, col: 9, hex: 'EB', chr: 'ë'},
    236: {row: 3, col: 9, hex: 'EC', chr: 'ì'},
    237: {row: 2, col: 9, hex: 'ED', chr: 'í'},
    238: {row: 4, col: 9, hex: 'EE', chr: 'î'},
    239: {row: 5, col: 9, hex: 'EF', chr: 'ï'},
    240: {row:12, col: 1, hex: 'F0', chr: 'ð'},
    241: {row: 6, col: 9, hex: 'F1', chr: 'ñ'},
    242: {row: 8, col: 9, hex: 'F2', chr: 'ò'},
    243: {row: 7, col: 9, hex: 'F3', chr: 'ó'},
    244: {row: 9, col: 9, hex: 'F4', chr: 'ô'},
    245: {row:11, col: 9, hex: 'F5', chr: 'õ'},
    246: {row:10, col: 9, hex: 'F6', chr: 'ö'},
    247: {row: 6, col:13, hex: 'F7', chr: '÷'},
    248: {row:15, col:11, hex: 'F8', chr: 'ø'},
    249: {row:13, col: 9, hex: 'F9', chr: 'ù'},
    250: {row:12, col: 9, hex: 'FA', chr: 'ú'},
    251: {row:14, col: 9, hex: 'FB', chr: 'û'},
    252: {row:15, col: 9, hex: 'FC', chr: 'ü'},
    253: {row:13, col: 1, hex: 'FD', chr: 'ý'},
    254: {row:14, col: 1, hex: 'FE', chr: 'þ'},
    255: {row: 8, col:13, hex: 'FF', chr: 'ÿ'},

    // Unicode Latin Extended A block
    338: {row:14, col:12, hex: '152', chr: 'Œ'},
    339: {row:15, col:12, hex: '153', chr: 'œ'},

    // Unicode General Punctuation block
    8216: {row:4, col:13, hex: '2018', chr: '‘'}, // Left Single Quotation Mark
    8217: {row:5, col:13, hex: '2019', chr: '’'}, // Right Single Quotation Mark
    8218: {row:2, col:14, hex: '201A', chr: '‚'}, // Single Low-9 Quotation Mark
    8219: {row:7, col:11, hex: '201B', chr: '‛'}, // Single High-Reversed-9 Quotation Mark
    8220: {row:2, col:13, hex: '201C', chr: '“'}, // Left Double Quotation Mark
    8221: {row:3, col:13, hex: '201D', chr: '”'}, // Right Double Quotation Mark
    8222: {row:3, col:14, hex: '201E', chr: '„'}, // Double Low-9 Quotation Mark
    8223: {row:8, col:11, hex: '201F', chr: '‟'}, // Double High-Reversed-9 Quotation Mark
    8224: {row:0, col:10, hex: '2020', chr: '†'}, // Dagger
    8225: {row:0, col:14, hex: '2021', chr: '‡'}, // Double Dagger
    8226: {row:5, col:10, hex: '2022', chr: '•'}, // Bullet

    // Unicode Currency Symbols block
    8364: {row:11, col:13, hex: '20AC', chr: '€'},

    // Unicode Private Use Area assignments for UI sprites
    59136: {row:0, col:0, hex: 'E700', name: 'Battery_05'},
    59137: {row:1, col:0, hex: 'E701', name: 'Battery_25'},
    59138: {row:2, col:0, hex: 'E702', name: 'Battery_50'},
    59139: {row:3, col:0, hex: 'E703', name: 'Battery_75'},
    59140: {row:4, col:0, hex: 'E704', name: 'Battery_99'},
    59141: {row:5, col:0, hex: 'E705', name: 'Radio_3'},
    59142: {row:6, col:0, hex: 'E706', name: 'Radio_2'},
    59143: {row:7, col:0, hex: 'E707', name: 'Radio_1'},
    59144: {row:8, col:0, hex: 'E708', name: 'Radio_0'},
    59145: {row:9, col:0, hex: 'E709', name: 'Radio_Off'},
    59146: {row:13, col:0, hex: 'E70A', name: 'Shift_Arrow'},
    59147: {row:14, col:0, hex: 'E70B', name: 'Backspace_Symbol'},
    59148: {row:15, col:0, hex: 'E70C', name: 'Enter_Symbol'},

    // Unicode Specials Block
    65533: {row:0, col:15, hex: 'FFFD', chr: '�'},
};
