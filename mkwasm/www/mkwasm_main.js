"use strict";
import { loadIMEEngineWasm, syncMessages } from './mkwasm_wasm.js';
import * as bkit_gui from './bkit_gui.js';
import * as rom from './bkit_gui_rom.js';
import * as kbd from './bkit_kbd.js';

const wasmModule = "mkwasm.wasm"
const backlightBtn = document.querySelector('#backlightBtn');
const kbdSelect = document.querySelector('#kbdSelect');
const keyboard = document.querySelector('#keyboard');
const screen = document.querySelector('#screen');
var cachedRomPages = {};

loadIMEEngineWasm(wasmModule);

window.requestAnimationFrame(initialize);

// Load data and add event listeners
function initialize() {
    loadRomToCache();

    // Configure backlight button
    backlightBtn.addEventListener('click', e => {
        let c = "backlit";
        let list = screen.classList;
        if (list.contains(c)) {
            list.remove(c);
        } else {
            list.add(c);
        }
    });

    // Configure on-screen keyboard
    kbd.addKeyboardListener(document);
    kbd.showAzertyOSK(keyboard);
    kbdSelect.addEventListener('change', e => {
        if (e.target.value === 'Azerty') {
            kbd.showAzertyOSK(keyboard);
        } else if (e.target.value === 'Qwerty') {
            kbd.showQwertyOSK(keyboard);
        }
    });

    // Hard Reset:  doHardReset(),
    // Soft Reboot: doRepaintWithEventCode(""),
    // Wifi 0:      doRepaintWithEventCode(": wWifi sprWifi0 ;"),
    // Bat 99:      doRepaintWithEventCode(": wBat sprBat99 ;"),
    // Qwerty:      doRepaintWithEventCode(": kbd kQwerty ;"),
    // QwertyAlt:   doRepaintWithEventCode(": kbd kQwertyAlt ;"),
    // Azerty:      doRepaintWithEventCode(": kbd kAzerty ;"),
    // AzertyAltL:  doRepaintWithEventCode(": kbd kAzertyAltL ;"),
    // AzertyAltR:  doRepaintWithEventCode(": kbd kAzertyAltR ;"),
    // Note:        doRepaintWithEventCode(`: note (${text}) ;`);
    doHardReset();
}

// Load ROM pages and render with default slot values
function doHardReset() {
    loadRomToCache();
    let allRomPages = ['Sprites', 'Widgets', 'KbdCommon', 'KbdQwerty',
                    'KbdAzerty', 'Views', 'PaintFrame'];
    let code = allRomPages.map(p => cachedRomPages[p]).join("\n");
    // This might take a while, so schedule it for a repaint
    window.requestAnimationFrame(() => bkit_gui.run(code, screen));
}

// Render frame with event code spliced between toolkit library pages (with
// slot defaults) and paint frame page with code to render active view. Goal is
// to let event code override default slot values before view renders.
function doRepaintWithEventCode(eventCode) {
    let libraryPages = ['Sprites', 'Widgets', 'KbdCommon', 'KbdQwerty',
                        'KbdAzerty', 'Views'];
    let libraryCode = libraryPages.map(p => cachedRomPages[p]).join("\n");
    let slotOverrides = eventCode;
    let paintFrameCode = cachedRomPages['PaintFrame'];
    let code = [libraryCode, slotOverrides, paintFrameCode].join("\n");
    // This might take a while, so schedule it for a repaint
    window.requestAnimationFrame(() => bkit_gui.run(code, screen));
}

// Load a fresh copy of ROM into the cache
function loadRomToCache() {
    cachedRomPages = {
        PaintFrame: rom.PaintFrame,
        Views: rom.Views,
        KbdAzerty: rom.KbdAzerty,
        KbdQwerty: rom.KbdQwerty,
        KbdCommon: rom.KbdCommon,
        Widgets: rom.Widgets,
        Sprites: rom.Sprites,
    };
}
