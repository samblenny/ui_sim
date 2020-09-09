"use strict";
import * as bkit_gui from './bkit_gui.js';
import * as rom from './bkit_gui_rom.js';

const backlightBtn = document.querySelector('#backlightBtn');
const romPage = document.querySelector('#romPage');
const editor = document.querySelector('#editor');
const spriteError = document.querySelector('#sprite h2 span.err');
const spriteGrid = document.querySelector('#sprite .grid');
var spritePixels; // This gets set properly after buttons are injected into grid
const spriteText = document.querySelector('#spriteText');
const screen = document.querySelector('#screen');
var cachedRomPages = {};
var activePage = null;

window.requestAnimationFrame(initialize);

// Load data and add event listeners
function initialize() {
    loadRomToCache();
    openRomPage("PaintFrame");

    // Configure ROM page select element
    romPage.addEventListener('change', e => {
        saveRomPage();
        openRomPage(romPage.value);
    });

    // Immediately save edits to active page to make them available for use
    // with event triggers
    editor.addEventListener('input', e => {
        saveRomPage();
    });

    // Configure backlight button
    backlightBtn.addEventListener('click', e => {
        let c = "backlit";
        let list = screen.classList;
        let grid = spriteGrid.classList;
        if (list.contains(c)) {
            list.remove(c);
            grid.remove(c);
        } else {
            list.add(c);
            grid.add(c);
        }
    });

    // Inject pixel buttons for sprite editor
    for (let i=0; i<24*16; i++) {
        let pxBtn = document.createElement('button');
        pxBtn.addEventListener('click', doToggleSpritePixel);
        spriteGrid.appendChild(pxBtn);
    }
    spritePixels = document.querySelectorAll('#sprite .grid button');
    // Configure sprite editor
    spriteText.addEventListener('input', doSpriteTextChange);
    spriteText.addEventListener('dragenter', doAddDragoverStyle);
    spriteText.addEventListener('dragexit', doRemoveDragoverStyle);
    spriteText.addEventListener('dragend', doRemoveDragoverStyle);
    spriteText.addEventListener('dragleave', doRemoveDragoverStyle);
    spriteText.addEventListener('drop', doSpriteTextDrop);
    encodePixelsToSpriteText();

    // Inject Event Trigger Buttons with callbacks
    let triggerDiv = document.querySelector('#triggers');
    let triggers = {
        "Hard Reset": e => doHardReset(),
        "Soft Reboot": e => doRepaintWithEventCode(""),
        "Wifi 0": e => doRepaintWithEventCode(": wWifi sprWifi0 ;"),
        "Wifi 1": e => doRepaintWithEventCode(": wWifi sprWifi1 ;"),
        "Wifi 2": e => doRepaintWithEventCode(": wWifi sprWifi2 ;"),
        "Wifi 3": e => doRepaintWithEventCode(": wWifi sprWifi3 ;"),
        "Bat 99": e => doRepaintWithEventCode(": wBat sprBat99 ;"),
        "Bat 75": e => doRepaintWithEventCode(": wBat sprBat75 ;"),
        "Bat 50": e => doRepaintWithEventCode(": wBat sprBat50 ;"),
        "Bat 25": e => doRepaintWithEventCode(": wBat sprBat25 ;"),
        "Bat 05": e => doRepaintWithEventCode(": wBat sprBat05 ;"),
        "Qwerty": e => doRepaintWithEventCode(": kbd kQwerty ;"),
        "QwertyAlt": e => doRepaintWithEventCode(": kbd kQwertyAlt ;"),
        "Azerty": e => doRepaintWithEventCode(": kbd kAzerty ;"),
        "AzertyAltL": e => doRepaintWithEventCode(": kbd kAzertyAltL ;"),
        "AzertyAltR": e => doRepaintWithEventCode(": kbd kAzertyAltR ;"),
    };
    for (const [tLabel, tCallback] of Object.entries(triggers)) {
        let b = document.createElement('button');
        b.textContent = tLabel;
        b.addEventListener('click', tCallback);
        triggerDiv.appendChild(b);
    }
    // Add note slot editor
    let note = document.createElement('input');
    note.setAttribute('type', 'text');
    note.setAttribute('size', 20);
    note.setAttribute('id', 'tEventNote');
    note.setAttribute('value', 'Hello, World!');
    note.addEventListener('input', e => {
        // Escape the end of string character to prevent code injection
        let text = e.srcElement.value.replace(/\)/g, '\\)');
        doRepaintWithEventCode(`: note (${text}) ;`);
    });
    triggerDiv.appendChild(note);
    doHardReset();
}

// Load ROM pages and render with default slot values
function doHardReset() {
    document.querySelector('#tEventNote').value = "Hello, World!";
    loadRomToCache();
    openRomPage(activePage);
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

// Temporarily save edits to the current ROM page.
// Edits will be discarded when tab is closed
function saveRomPage() {
    if(activePage) {cachedRomPages[activePage] = editor.value;}
}

// Load a different ROM page into the editor
function openRomPage(newPage) {
    editor.value = cachedRomPages[newPage];
    activePage = newPage;
}

// Handle click on sprite editor pixel
function doToggleSpritePixel(e) {
    let pxCList = e.target.classList;
    if (pxCList.contains("on")) {pxCList.remove("on");}
    else {pxCList.add("on");}
    encodePixelsToSpriteText();
}

// Handle change to sprite editor textarea (maybe typing, maybe drop)
// - Attempt to parse block of 24x16=384 pixels from string with < ... >
// - Warn about unrecoverable syntax errors
function doSpriteTextChange(e) {
    let bits = match384SpriteBits(spriteText.value);
    if(bits) {
        let revBits = Array.from(bits).reverse();
        console.assert(revBits.length === spritePixels.length);
        spritePixels.forEach(p => {
            if(revBits.pop() === '1') {p.classList.add("on");} 
            else {p.classList.remove("on");}
        });
        encodePixelsToSpriteText();
        spriteError.textContent = "";
    } else {
        spriteError.textContent = "[Bad Data]";
    }
}

// Handle dragover events by changing styles to give visual feedback
function doAddDragoverStyle(e) {spriteText.classList.add("dragover");}
function doRemoveDragoverStyle(e) {spriteText.classList.remove("dragover");}

// Handle drop event on sprite editor textarea
// - Clear dragover visual feedback
// - Validate data format
// - Update pixels and normalize text representation if data was parseable
function doSpriteTextDrop(e) {
    e.preventDefault();
    doRemoveDragoverStyle();
    let newText = e.dataTransfer.getData("text/plain");
    if (match384SpriteBits(newText)) {
        spriteText.value = newText;
        doSpriteTextChange();
    } else {
        showBadSpriteDataError();
    }
}

// Attempt to match and return bits for a 24x16=384 pixel sprite
// - Check for a 24x16=384 pixel bit string in < ... > format
// - Ignore whitespace
// - Ignore text outside the angle brackets
function match384SpriteBits(text) {
    let match = text.replace(/[ \n]/gm, '').match(/<([01]{384})>/);
    if (match && match[1]) {
        return match[1];
    } else {
        return false;
    }
}

// Regenerate normalized sprite text representation after change in pixel states
// - Precondition: sprite editor is supposed to have a 24x16 grid of buttons
// - Decode pixel data by checking buttons for presence of "on" class
// - Format decoded pixel data into string like "< 1010... >\n24 16"
function encodePixelsToSpriteText() {
    let cols = 24;
    let rows = 16;
    let correctSize = spritePixels.length === cols * rows
    console.assert(correctSize, "sprite editor grid is not 24x16 pixels!");
    let pixels = Array.from(spritePixels).map(p => p.classList.contains("on") ? 1 : 0);
    let lines = [];
    for (let i=0; i<rows*cols; i+=cols) {
        let bits = pixels.slice(i, i+cols);
        lines.push(bits.join(""));
    }
    spriteText.value = "< " + lines.join("\n  ") + " >\n24 16";
    hideBadSpriteDataError();
}

// Set/Clear bad data visual indicator next to sprite editor heading
function showBadSpriteDataError() {spriteError.textContent = "[Bad Data]";}
function hideBadSpriteDataError() {spriteError.textContent = "";}
