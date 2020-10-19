"use strict";
const wasmModule = "mkwasm.wasm";

// Load WASM module, bind shared memory, then invoke callback
export function loadModule(callback) {
    var importObject = {
        js: {js_log_trace: (traceCode) => {
                  console.log("wasm trace code:", traceCode);
              },
            },
    };
    if ("instantiateStreaming" in WebAssembly) {
        // The new, more efficient way
        WebAssembly.instantiateStreaming(fetch(wasmModule), importObject)
            .then(initSharedMemBindings)
            .then(callback);
    } else {
        // Fallback for Safari
        fetch(wasmModule)
            .then(response => response.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, importObject))
            .then(initSharedMemBindings)
            .then(callback);
    }
}

// Bindings for shared memory and functions
var wasmShared;
var wasmExports;
var wasmInstanceReady = false;

// UTF8 decoder
let decoder = new TextDecoder();

// Callback to initialize shared memory IPC bindings once WASM module is instantiated
function initSharedMemBindings(result) {
    wasmExports = result.instance.exports;
    wasmShared = new Uint8Array(wasmExports.memory.buffer);
    wasmInstanceReady = true;
}

export function init() {
    wasmExports.init();
}

export function lcdDirty() {
    return wasmExports.lcd_dirty();
}

export function lcdClearDirty() {
    wasmExports.lcd_clear_dirty();
}

export function demoTick() {
    wasmExports.demo_tick();
}

export function keydown(keyCode) {
    if (!wasmInstanceReady) {throw "wasm instance is not ready";}
    let kci = KeyCodeIndex[keyCode];
    if (kci && kci >= 0) {
        wasmExports.keydown(KeyCodeIndex[keyCode]);
    }
}

export function keyup(keyCode) {
    if (!wasmInstanceReady) {throw "wasm instance is not ready";}
    let kci = KeyCodeIndex[keyCode];
    if (kci && kci >= 0) {
        wasmExports.keyup(KeyCodeIndex[keyCode]);
    }
}

export function lcdFrameBuf() {
    if (!wasmInstanceReady) {throw "wasm instance is not ready";}
    const lcdLines = 536;
    const lcdWordsPerLine = 11;
    const lcdPxPerLine = 336;
    let size = lcdLines * lcdWordsPerLine * 4;
    let start = wasmExports.lcd_frame_buf_ptr();
    let bytes = wasmShared.subarray(start, start + size);
    return {
        bytes: bytes,
        lines: lcdLines,
        wordsPerLine: lcdWordsPerLine,
        pxPerLine: lcdPxPerLine,
    };
}

export function setLayoutAzerty() {
    if (!wasmInstanceReady) {throw "wasm instance is not ready";}
    wasmExports.set_layout_azerty();
}

export function setLayoutQwerty() {
    if (!wasmInstanceReady) {throw "wasm instance is not ready";}
    wasmExports.set_layout_qwerty();
}

// Lookup table to translate from keycode to u8
const KeyCodeIndex = {
    P2: 0,
    P5: 1,
    PC: 2,
    P6: 3,
    P3: 4,
    P4: 5,
    P9: 6,
    P7: 7,
    P8: 8,
    P13: 9,
    P14: 10,
    P15: 11,
    P16: 12,
    P17: 13,
    P18: 14,
    P19: 15,
    P20: 16,
    P21: 17,
    P22: 18,
    P23: 19,
    P24: 20,
    P25: 21,
    P26: 22,
    P27: 23,
    P28: 24,
    P29: 25,
    P30: 26,
    P31: 27,
    P32: 28,
    P33: 29,
    P34: 30,
    P35: 31,
    P36: 32,
    P37: 33,
    P38: 34,
    P39: 35,
    P40: 36,
    P41: 37,
    P42: 38,
    P43: 39,
    P44: 40,
    P45: 41,
    P46: 42,
    P47: 43,
    P48: 44,
    P49: 45,
    P50: 46,
    P51: 47,
    P52: 48,
    P53: 49,
    P54: 50,
    P55: 51,
    P56: 52,
    P57: 53,
};
