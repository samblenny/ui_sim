"use strict";

// Listen for keyboard events (from OS keyboard)
export function addKeyboardListener(element) {
    element.addEventListener('keydown', doKeydown);
    element.addEventListener('keyup', doKeyup);
}

// Stop listening for keyboard events
export function removeKeyboardListener(element) {
    element.removeEventListener('keydown', doKeydown);
    element.removeEventListener('keyup', doKeyup);
}

// Handle a keydown event (from OS keyboard)
function doKeydown(e) {
    // Ignore repeat events
    if (e.repeat) {
        return;
    }
    // Try to interpret the keypress
    let scancode = translate(e.code);
    const filter = ['Space', 'Enter', 'ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'];
    if (scancode) {
        if (filter.includes(e.code)) {
            // Prevent actions normally triggered by spacebar and arrow keys
            e.preventDefault();
        }
        console.log(scancode + 'p');
        let onscreenKey = document.getElementById(scancode);
        if (onscreenKey) {
            onscreenKey.classList.add('glow');
        }
    } else {
        console.log('unrecognized keydown:', e.code);
    }
}

// Handle a keyup event (from OS keyboard)
function doKeyup(e) {
    let scancode = translate(e.code);
    const filter = ['Space', 'Enter', 'ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'];
    if (scancode) {
        if (filter.includes(e.code)) {
            // Prevent actions normally triggered by spacebar and arrow keys
            e.preventDefault();
        }
        console.log(scancode + 'r');
        let onscreenKey = document.getElementById(scancode);
        if (onscreenKey) {
            onscreenKey.classList.remove('glow');
        }
    } else {
        console.log('unrecognized keyup:', e.code);
    }
}

// Translate KeyboardEvent.code to scancode.
// See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
// to learn about KeyboardEvent.code relationship to physical key positions.
function translate(keyCode) {
    return KeyCodeLookupTable[keyCode];
}

//                        P2
//                    P5  PC  P6
//    P3       P4         P9         P7       P8
//
// P13  P14  P15  P16  P17  P18  P19  P20  P21  P22
//
// P23  P24  P25  P26  P27  P28  P29  P30  P31  P32
//
// P33  P34  P35  P36  P37  P38  P39  P40  P41  P42
//
// P43  P44  P45  P46  P47  P48  P49  P50  P51  P52
//
//     P53  P54  P55_____P55____P55  P56  P57
//
const KeyCodeLookupTable = {
    ArrowUp: 'P2',
    ArrowLeft: 'P5',
    Escape: 'PC',
    ArrowRight: 'P6',
    F1: 'P3',
    F2: 'P4',
    ArrowDown: 'P9',
    F3: 'P7',
    F4: 'P8',
    Digit1: 'P13',
    Digit2: 'P14',
    Digit3: 'P15',
    Digit4: 'P16',
    Digit5: 'P17',
    Digit6: 'P18',
    Digit7: 'P19',
    Digit8: 'P20',
    Digit9: 'P21',
    Digit0: 'P22',
    KeyQ: 'P23',
    KeyW: 'P24',
    KeyE: 'P25',
    KeyR: 'P26',
    KeyT: 'P27',
    KeyY: 'P28',
    KeyU: 'P29',
    KeyI: 'P30',
    KeyO: 'P31',
    KeyP: 'P32',
    KeyA: 'P33',
    KeyS: 'P34',
    KeyD: 'P35',
    KeyF: 'P36',
    KeyG: 'P37',
    KeyH: 'P38',
    KeyJ: 'P39',
    KeyK: 'P40',
    KeyL: 'P41',
    Enter: 'P42',
    ShiftLeft: 'P43',
    KeyZ: 'P44',
    KeyX: 'P45',
    KeyC: 'P46',
    KeyV: 'P47',
    KeyB: 'P48',
    KeyN: 'P49',
    KeyM: 'P50',
    Slash: 'P51',
    ShiftRight: 'P52',
    AltLeft: 'P53',
    MetaLeft: 'P54',
    ControlLeft: 'P54',
    Comma: 'P54',
    Space: 'P55',
    MetaRight: 'P56',
    ControlRight: 'P56',
    Period: 'P56',
    AltRight: 'P57',
};

// Stuff below this line is for clickable SVG on-screen keyboard

// Replace contents of svgElement with Azerty on-screen keyboard
export function showAzertyOSK(svgElement) {
    pruneSVG(svgElement);
    let osk = buildAzertyOSK();
    svgElement.appendChild(osk);
}

// Replace contents of svgElement with a Qwerty on-screen keyboard
export function showQwertyOSK(svgElement) {
    pruneSVG(svgElement);
    let osk = buildQwertyOSK();
    svgElement.appendChild(osk);
}

// Remove previous contents of SVG element
function pruneSVG(svgElement) {
    while(svgElement.firstChild) {
        svgElement.removeChild(svgElement.firstChild);
    }
}

// SVG namespace is required to make document.createElementNS work for SVG
const SVG_NS = 'http://www.w3.org/2000/svg';

// Return an svg group with Azerty key rectangles and labels
function buildAzertyOSK() {
    let g = document.createElementNS(SVG_NS, 'g');
    for (const [keyId, col, row, width] of keyBoxBounds) {
        let r = keyRect(keyId, col, row, width);
        g.appendChild(r);
        let label = AzertyKeys[keyId][0];
        if (label !== '') {
            let tc = textC(label, col, row, width);
            g.appendChild(tc);
        }
    }
    return g;
}

// Return an svg group with Qwerty key rectangles and labels
function buildQwertyOSK() {
    let g = document.createElementNS(SVG_NS, 'g');
    for (const [keyId, col, row, width] of keyBoxBounds) {
        let r = keyRect(keyId, col, row, width);
        g.appendChild(r);
        let label = QwertyKeys[keyId][0];
        if (label !== '') {
            let tc = textC(label, col, row, width);
            g.appendChild(tc);
        }
    }
    return g;
}

// Return rectangle with key grid bounds specified by (row, col, width)
function keyRect(keyId, col, row, width) {
    let r = document.createElementNS(SVG_NS, 'rect');
    let [x, y, w, h] = xywhForColRowWidth(col, row, width);
    let radius = 2;
    r.setAttribute('id', keyId);
    r.setAttribute('x', x);
    r.setAttribute('y', y);
    r.setAttribute('width', w);
    r.setAttribute('height', h);
    r.addEventListener('mousedown', doSvgKeyPress);
    r.addEventListener('touchstart', doSvgKeyPress, {passive: true});
    r.addEventListener('mouseup', doSvgKeyMaybeRelease);
    r.addEventListener('touchend', doSvgKeyMaybeRelease);
    r.addEventListener('mouseout', doSvgKeyMaybeRelease);
    r.addEventListener('mouseleave', doSvgKeyMaybeRelease);
    r.addEventListener('touchcancel', doSvgKeyMaybeRelease);
    r.addEventListener('click', doSvgClick);
    return r;
}

// Handle key press and show visual feedback
function doSvgKeyPress(e) {
    e.target.classList.add('glow');
    console.log("press:", e.target.id);
}

// Handle possible key release event and remove visual feedback if needed.
// These need a check for whether the target has glow because it is possible
// to drag out of a key while the mouse button is down.
function doSvgKeyMaybeRelease(e) {
    if (e.target.classList.contains('glow')) {
        e.target.classList.remove('glow');
        console.log("release:", e.target.id);
    }
}

// Respond to a key that has been tapped or clicked
function doSvgClick(e) {
    e.target.classList.remove('glow');
    console.log("click: ", e.target.id);
}

// Return text centered in key grid rectangle specified by (row, col, width)
function textC(text, col, row, width) {
    let tc = document.createElementNS(SVG_NS, 'text');
    let [x, y, w, h] = xywhForColRowWidth(col, row, width);
    tc.setAttribute('x', x + w * 0.5);
    tc.setAttribute('y', y + h * 0.8);
    tc.setAttribute('class', 'tc');
    tc.textContent = text;
    return tc;
}

// Compute rectangle bounds on key grid given key's column, row, and width
function xywhForColRowWidth(col, row, width) {
    const keyGridSize = 48;
    const gutter = 5;
    const x = gutter + keyGridSize * col;
    const y = gutter + keyGridSize * row;
    const w = (keyGridSize * width) - gutter;
    const h = keyGridSize - gutter;
    return [x, y, w, h];
}

// Key label list format: Id: [Base, ShiftL, ShiftR]
const AzertyKeys = {
    'P2': ['', '', ''],
    'P5': ['', '', ''],
    'PC': ['', '', ''],
    'P6': ['', '', ''],
    'P9': ['', '', ''],

    'P3': ['F1', '', 'Tab'],
    'P4': ['F2', '', ''],
    'P7': ['F3', '', ''],
    'P8': ['F4', 'Ctrl', ''],

    'P13': ['1', '§', 'à'],
    'P14': ['2', '', 'é'],
    'P15': ['3', '', 'è'],
    'P16': ['4', '', 'ê'],
    'P17': ['5', '[', '('],
    'P18': ['6', ']', ')'],
    'P19': ['7', '', '&'],
    'P20': ['8', '_', '*'],
    'P21': ['9', '\'', '«'],
    'P22': ['0', '"', '»'],

    'P23': ['A', '', 'æ'],
    'P24': ['Z', '', '£'],
    'P25': ['E', '', '€'],
    'P26': ['R', '', '`'],
    'P27': ['T', '', '{'],
    'P28': ['Y', '', '}'],
    'P29': ['U', '', 'ù'],
    'P30': ['I', '', 'ï'],
    'P31': ['O', '', 'œ'],
    'P32': ['P', '', '%'],

    'P33': ['Q', '', '@'],
    'P34': ['S', '', 'ß'],
    'P35': ['D', '', '$'],
    'P36': ['F', '', '¤'],
    'P37': ['G', '', 'µ'],
    'P38': ['H', '', '-'],
    'P39': ['J', '', '+'],
    'P40': ['K', '\\', '/'],
    'P41': ['L', '', '|'],
    'P42': ['M', '', '#'],

    'P43': ['', '', '⌫'],
    'P44': ['W', '', '<'],
    'P45': ['X', '', '>'],
    'P46': ['C', '', 'ç'],
    'P47': ['V', '¨', '^'],
    'P48': ['B', '', '='],
    'P49': ['N', '', '~'],
    'P50': [':', '¿', '?'],
    'P51': [';', '¡', '!'],
    'P52': ['', '⏎', ''],

    'P53': ['↑', '↑', '↑'],
    'P54': [',', '', 'Sym'],
    'P55': ['', '', ''],
    'P56': ['.', '', '😃'],
    'P57': ['↑', '↑', '↑'],
};

// Key label list format: Id: [Base, Shift]
const QwertyKeys = {
    'P2': ['', ''],
    'P5': ['', ''],
    'PC': ['', ''],
    'P6': ['', ''],
    'P9': ['', ''],

    'P3': ['F1', 'Tab'],
    'P4': ['F2', ''],
    'P7': ['F3', ''],
    'P8': ['F4', 'Ctrl'],

    'P13': ['1', ''],
    'P14': ['2', ''],
    'P15': ['3', ''],
    'P16': ['4', ''],
    'P17': ['5', ''],
    'P18': ['6', ''],
    'P19': ['7', ''],
    'P20': ['8', ''],
    'P21': ['9', ''],
    'P22': ['0', ''],

    'P23': ['Q', '%'],
    'P24': ['W', '^'],
    'P25': ['E', '~'],
    'P26': ['R', '|'],
    'P27': ['T', '['],
    'P28': ['Y', ']'],
    'P29': ['U', '<'],
    'P30': ['I', '>'],
    'P31': ['O', '{'],
    'P32': ['P', '}'],

    'P33': ['A', '@'],
    'P34': ['S', '#'],
    'P35': ['D', '&'],
    'P36': ['F', '*'],
    'P37': ['G', '-'],
    'P38': ['H', '+'],
    'P39': ['J', '='],
    'P40': ['K', '('],
    'P41': ['L', ')'],
    'P42': ['⌫', ''],

    'P43': ['!', '`'],
    'P44': ['Z', '_'],
    'P45': ['X', '$'],
    'P46': ['C', '"'],
    'P47': ['V', '\''],
    'P48': ['B', ':'],
    'P49': ['N', ';'],
    'P50': ['M', '/'],
    'P51': ['?', '\\'],
    'P52': ['⏎', ''],

    'P53': ['↑', '↑'],
    'P54': [',', 'Sym'],
    'P55': ['', ''],
    'P56': ['.', '😃'],
    'P57': ['↑', '↑'],
};

// Key location list format: [Id, Column, Row, Width]
const keyBoxBounds = [
    ['P2', 4.5, 0, 1], // Up
    ['P5', 3.5, 1, 1], // Left
    ['PC', 4.5, 1, 1], // Click
    ['P6', 5.5, 1, 1], // Right
    ['P9', 4.5, 2, 1], // Down

    ['P3',  0, 2, 2],  // F-keys
    ['P4',  2, 2, 2],
    ['P7',  6, 2, 2],
    ['P8',  8, 2, 2],

    ['P13', 0, 3, 1],  // Number row
    ['P14', 1, 3, 1],
    ['P15', 2, 3, 1],
    ['P16', 3, 3, 1],
    ['P17', 4, 3, 1],
    ['P18', 5, 3, 1],
    ['P19', 6, 3, 1],
    ['P20', 7, 3, 1],
    ['P21', 8, 3, 1],
    ['P22', 9, 3, 1],

    ['P23', 0, 4, 1],  // Upper letter row
    ['P24', 1, 4, 1],
    ['P25', 2, 4, 1],
    ['P26', 3, 4, 1],
    ['P27', 4, 4, 1],
    ['P28', 5, 4, 1],
    ['P29', 6, 4, 1],
    ['P30', 7, 4, 1],
    ['P31', 8, 4, 1],
    ['P32', 9, 4, 1],

    ['P33', 0, 5, 1],  // Home row
    ['P34', 1, 5, 1],
    ['P35', 2, 5, 1],
    ['P36', 3, 5, 1],
    ['P37', 4, 5, 1],
    ['P38', 5, 5, 1],
    ['P39', 6, 5, 1],
    ['P40', 7, 5, 1],
    ['P41', 8, 5, 1],
    ['P42', 9, 5, 1],

    ['P43', 0, 6, 1],  // Lower letter row
    ['P44', 1, 6, 1],
    ['P45', 2, 6, 1],
    ['P46', 3, 6, 1],
    ['P47', 4, 6, 1],
    ['P48', 5, 6, 1],
    ['P49', 6, 6, 1],
    ['P50', 7, 6, 1],
    ['P51', 8, 6, 1],
    ['P52', 9, 6, 1],

    ['P53', 1, 7, 1],  // Spacebar row
    ['P54', 2, 7, 1],
    ['P55', 3, 7, 4],
    ['P56', 7, 7, 1],
    ['P57', 8, 7, 1],
];
