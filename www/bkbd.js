"use strict";
var postURL = "";

// Convert keyboard events to scancodes and POST them
export function addKeyboardListener(element, url) {
    // Blindly POST and ignore the response. If something seems wrong, look for
    // fetch errors (400, 404, etc.) in the Javascript console log.
    postURL = url;
    element.addEventListener('keydown', handleKeydown);
    element.addEventListener('keyup', handleKeyup);
}

export function removeKeyboardListener(element) {
    element.removeEventListener('keydown', handleKeydown);
    element.removeEventListener('keyup', handleKeyup);
}

// Handle a keydown event
function handleKeydown(e) {
    // Ignore repeat events
    if (e.repeat) {
        return;
    }
    // Try to interpret the keypress
    let scancode = translate(e.code);
    if (scancode) {
        // Attempt to prevent accidents like clicking "Hang Up" button when typing space
        if (e.code == 'Space') { e.preventDefault(); }
        // POST the keydown event
        fetch(postURL + scancode + 'p', {method: "POST"});
    } else {
        console.log('unrecognized keydown:', e.code);
    }
}

// Handle a keyup event
function handleKeyup(e) {
    let scancode = translate(e.code);
    if (scancode) {
        // Attempt to prevent accidents like clicking "Hang Up" button when typing space
        if (e.code == 'Space') { e.preventDefault(); }
        // POST the keyup event
        fetch(postURL + scancode + 'r', {method: "POST"});
    } else {
        console.log('unrecognized keyup:', e.code);
    }
}

// Translate KeyboardEvent.code to scancode
function translate(keyCode) {
    return LookupTable[keyCode];
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
const LookupTable = {
    ArrowUp: 'P2_',
    ArrowLeft: 'P5_',
    Escape: 'PC_',
    ArrowRight: 'P6_',
    F1: 'P3_',
    F2: 'P4_',
    ArrowDown: 'P9_',
    F3: 'P7_',
    F4: 'P8_',
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
    Space: 'P55',
    MetaRight: 'P56',
    AltRight: 'P57',
};
