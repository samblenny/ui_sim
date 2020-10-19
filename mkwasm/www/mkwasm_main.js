"use strict";
import * as wasm from './mkwasm_wasm.js';
import * as kbd from './bkit_kbd.js';

const backlightBtn = document.querySelector('#backlightBtn');
const kbdSelect = document.querySelector('#kbdSelect');
const demoModeBtn = document.querySelector('#demoMode');
const keyboard = document.querySelector('#keyboard');
const screen = document.querySelector('#screen');
const screenCtx = screen.getContext('2d');

// Load wasm module with callback to continue initialization
let loadSuccessCallback = initialize;
wasm.loadModule(loadSuccessCallback);

// Load data and add event listeners
function initialize() {
    // Configure backlight button
    backlightBtn.addEventListener('click', e => {
        let c = "backlit";
        let list = screen.classList;
        if (list.contains(c)) {
            list.remove(c);
        } else {
            list.add(c);
        }
        backlightBtn.blur();
    });

    // Configure demo tick button (cycle demo animation)
    demoModeBtn.addEventListener('click', e => {
        if (demoModeBtn.classList.contains('on')) {
            demoModeBtn.classList.remove('on');
        } else {
            demoModeBtn.classList.add('on');
            kbdSelect.value = 'Qwerty';
            kbd.showQwertyOSK(keyboard);
            wasm.setLayoutQwerty();
            repaintLCD();
            recurringDemoTick();
        }
        demoModeBtn.blur();
    });

    // Configure on-screen keyboard
    let pressFn = sc => {
        wasm.keydown(sc);
        repaintLCD();
    };
    let releaseFn = sc => {
        wasm.keyup(sc);
        repaintLCD();
    };
    kbd.setKeyscanCallbacks(pressFn, releaseFn);
    kbd.addKeyboardListener(document);
    kbd.showAzertyOSK(keyboard);
    kbdSelect.addEventListener('change', e => {
        if (e.target.value === 'Azerty') {
            kbd.showAzertyOSK(keyboard);
            wasm.setLayoutAzerty();
            repaintLCD();
        } else if (e.target.value === 'Qwerty') {
            kbd.showQwertyOSK(keyboard);
            wasm.setLayoutQwerty();
            repaintLCD();
        }
        kbdSelect.blur();
    });

    // Initialize LCD screen
    screen.height = 536;
    screen.width = 336;
    wasm.init();
    repaintLCD();
}

// Send a demo tick event and conditionally schedule the next one
function recurringDemoTick() {
    wasm.demoTick();
    repaintLCD();
    if (demoModeBtn.classList.contains('on')) {
        // Demo mode is on, so keep ticking
        setTimeout(recurringDemoTick, 200);
    }
}

// Paint the frame buffer (wasm shared memory) to the screen (canvas element)
function repaintLCD() {
    if (!wasm.lcdDirty()) {
        return;
    }
    wasm.lcdClearDirty();
    let lcdData = wasm.lcdFrameBuf();
    let imageData = screenCtx.getImageData(0, 0, screen.width, screen.height);
    let pxOffset = 0;
    for (let line=0; line<lcdData.lines; line++) {
        for (let w=0; w<lcdData.wordsPerLine; w++) {
            // Lines are padded to multiples of 4 bytes
            if (w*32 < lcdData.pxPerLine) {
                let index = (line * lcdData.wordsPerLine * 4) + w*4;
                let b0 = lcdData.bytes[index];
                let b1 = lcdData.bytes[index+1];
                let b2 = lcdData.bytes[index+2];
                let b3 = lcdData.bytes[index+3];
                let word = ((b3 >>> 0) << 24) | (b2 << 16) | (b1 << 8) | b0;
                for (let bit=0; bit<32; bit++) {
                    let pxOffset = (line * lcdData.pxPerLine + w*32 + bit) * 4;
                    let fbPixel = 1 & (word >> bit)
                    // Pixel == 1 means clear (takes color of backlit background)
                    // Pixel == 0 means black
                    // To let the white (clear) pixels take the color of the canvas element's
                    // background, modulate the alpha channel.
                    imageData.data[pxOffset] = 0x33;
                    imageData.data[pxOffset+1] = 0x33;
                    imageData.data[pxOffset+2] = 0x33;
                    imageData.data[pxOffset+3] = (fbPixel==1) ? 0 : 0xff;
                }
            }
        }
    }
    screenCtx.putImageData(imageData, 0, 0);
}

// Keyboard overlay index to rom function lookup table
const KbdOverlay = [
    'kAzerty',
    'kAzertyS',
    'kAzertyAltL',
    'kAzertyAltR',
    'kAzertyAltRS',
    'kQwerty',
    'kQwertyS',
    'kQwertyAlt',
];
