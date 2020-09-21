#![no_std]
extern crate kbddrv;

/// For building wasm32 no_std, add panic handler and functions to let
/// javascript check shared buffer pointers. This panic handler conflicts with
/// test panic handler and therefore cannot be included during `cargo test`.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

/// For debug logging
mod trace;

/// For keyboard configuration and state
mod kbd;

/// Buffers for IPC with javascript using pointer into wasm shared memory
const MAX_CHARS: usize = 20;
const CHAR_BUF_SIZE: usize = MAX_CHARS;
const UTF8_BUF_SIZE: usize = MAX_CHARS * 4;
static mut CHAR_BUF: [char; CHAR_BUF_SIZE] = ['\0'; CHAR_BUF_SIZE];
static mut CHAR_BUF_END: usize = 0;
pub static mut UTF8_BUF: [u8; UTF8_BUF_SIZE] = [0; UTF8_BUF_SIZE];
pub static mut UTF8_BUF_END: usize = 0;

/// Respond to key press event
#[no_mangle]
pub extern "C" fn keydown(key_index: i32) {
    if key_index < 0 || key_index >= kbd::MAP_SIZE as i32 {
        trace::log_code(trace::Code::BadKeyIndex);
        return;
    }
    let result = &kbd::cur_map_lut()[key_index as usize];
    match result {
        kbd::R::C(c) => buffer_keystroke(*c),
        kbd::R::AltL => kbd::modkey_down(result),
        kbd::R::AltR => kbd::modkey_down(result),
        _ => (),
    }
}

/// Accumulate buffer of recently typed characters
fn buffer_keystroke(c: char) {
    unsafe {
        // Update the character buffer
        if CHAR_BUF_END < CHAR_BUF_SIZE {
            // Append character when character buffer is not yet full
            CHAR_BUF[CHAR_BUF_END] = c;
            CHAR_BUF_END += 1;
        } else {
            // When buffer is full, discard oldest, then append
            for i in 0..CHAR_BUF_SIZE - 1 {
                CHAR_BUF[i] = CHAR_BUF[i + 1];
            }
            CHAR_BUF[CHAR_BUF_SIZE - 1] = c;
        }
        // Encode the character buffer as utf-8 into the utf8 buffer
        let mut end = 0;
        for c in CHAR_BUF[0..CHAR_BUF_END].iter() {
            let dest = &mut UTF8_BUF[end..end + 4];
            let result = c.encode_utf8(dest);
            end += result.len();
        }
        UTF8_BUF_END = end;
    }
}

/// Respond to key release event
#[no_mangle]
pub extern "C" fn keyup(key_index: i32) {
    let _ = key_index;
}

/// Return current keyboard map based on layout config and modkey state machine
#[no_mangle]
pub extern "C" fn key_map_index() -> i32 {
    kbd::cur_map_index()
}

/// Substitute for using (non-existant) menu to select azerty keyboard layout
#[no_mangle]
pub extern "C" fn set_layout_azerty() {
    kbd::set_layout(kbd::Layout::Azerty);
}

/// Substitute for using (non-existant) menu to select qwerty keyboard layout
#[no_mangle]
pub extern "C" fn set_layout_qwerty() {
    kbd::set_layout(kbd::Layout::Qwerty);
}

#[cfg(test)]
mod tests {

    #[test]
    fn one() {
        assert_eq!(1, 1);
    }
}
