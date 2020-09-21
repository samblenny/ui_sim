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

/// Buffer for IPC with javascript using pointer into wasm shared memory
pub static mut UTF8_BUF: [u8; 30] = [0; 30];
pub static mut UTF8_BUF_LAST: u32 = 0;

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
        if (UTF8_BUF_LAST as usize) + 1 < UTF8_BUF.len() {
            // Append until buffer is full
            UTF8_BUF_LAST += 1;
            UTF8_BUF[UTF8_BUF_LAST as usize] = c as u8;
        } else {
            // When buffer is full, discard oldest, then append
            for i in 0..UTF8_BUF.len() - 1 {
                UTF8_BUF[i] = UTF8_BUF[i + 1];
            }
            UTF8_BUF[UTF8_BUF.len() - 1] = c as u8;
        }
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
