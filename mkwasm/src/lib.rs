#![no_std]
extern crate kbddrv;

// Wasm shared memory exports
pub static mut KBD_OVERLAY: u32 = 65;
pub static mut UTF8_BUF: [u8; 30] = [0; 30];
pub static mut UTF8_BUF_LAST: u32 = 0;

// For building wasm32 no_std, add panic handler. This panic handler would
// conflict with the text panic handler if included during `cargo test`.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

// For wasm32 build, use debug trace WebAssembly IPC function binding
#[cfg(target_arch = "wasm32")]
use no_std_bindings::js_log_trace;

// For other builds (test), replace debug trace binding with stub
#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_log_trace(_: i32) {}

/// Respond to key press event
#[no_mangle]
pub extern "C" fn keydown(key_index: i32) {
    if key_index < 0 || key_index >= char_map::SIZE as i32 {
        unsafe {js_log_trace(-1);}
        return;
    }
    unsafe {
        let c = char_map::AZERTY_BASE[key_index as usize];
        if c != '\0' {
            if UTF8_BUF_LAST as usize + 1 < UTF8_BUF.len() {
                // Append until buffer is full
                UTF8_BUF_LAST += 1;
                UTF8_BUF[UTF8_BUF_LAST as usize] = c as u8;
            }  else {
                // When buffer is full, discard oldest, then append
                for i in 0..UTF8_BUF.len()-1 {
                    UTF8_BUF[i] = UTF8_BUF[i+1];
                }
                UTF8_BUF[UTF8_BUF.len()-1] = c as u8;
            }
        }
    }
}

/// Respond to key release event
#[no_mangle]
pub extern "C" fn keyup(key_index: i32) {
    let _ = key_index;
}

mod char_map {
    pub const SIZE: usize = 54;
    pub const AZERTY_BASE: [char; SIZE] = [
        '\0', // P2  Up
        '\0', // P5  Left
        '\0', // PC  Click
        '\0', // P6  Right
        '\0', // P3  F1
        '\0', // P4  F2
        '\0', // P9  Down
        '\0', // P7  F3
        '\0', // P8  F4
        '1',  // P13 1
        '2',  // P14 2
        '3',  // P15 3
        '4',  // P16 4
        '5',  // P17 5
        '6',  // P18 6
        '7',  // P19 7
        '8',  // P20 8
        '9',  // P21 9
        '0',  // P22 0
        'a',  // P23 Upper letter row
        'z',  // P24
        'e',  // P25
        'r',  // P26
        't',  // P27
        'y',  // P28
        'u',  // P29
        'i',  // P30
        'o',  // P31
        'p',  // P32
        'q',  // P33 Home letter row
        's',  // P34
        'd',  // P35
        'f',  // P36
        'g',  // P37
        'h',  // P38
        'j',  // P39
        'k',  // P40
        'l',  // P41
        'm',  // P42
        '\0', // P43 Lower letter row
        'w',  // P44
        'x',  // P45
        'c',  // P46
        'v',  // P47
        'b',  // P48
        'n',  // P49
        ':',  // P50
        ';',  // P51
        '\0', // P52
        '\0', // P53 ShiftL
        ',',  // P54 Comma
        ' ',  // P55 Space
        '.',  // P56 Period
        '\0', // P57 ShiftR
    ];
}

#[cfg(test)]
mod tests {

    #[test]
    fn one() {
        assert_eq!(1, 1);
    }
}
