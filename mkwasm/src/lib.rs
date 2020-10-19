#![no_std]
extern crate guilib;
extern crate trace;

use guilib::state::{Context, FrameBuf};

static mut FB: FrameBuf = FrameBuf::new();
static mut CTX: Context = Context::new();

/// For building wasm32 no_std, add panic handler and functions to let
/// javascript check shared buffer pointers. This panic handler conflicts with
/// test panic handler and therefore cannot be included during `cargo test`.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

/// Initialize the hardware (splash screen, etc.)
#[no_mangle]
pub extern "C" fn init() {
    guilib::api::repaint(unsafe { &mut FB }, unsafe { &mut CTX });
}

/// Check if lcd frame buffer is dirty: 0=>clean, _=>dirty
#[no_mangle]
pub extern "C" fn lcd_dirty() -> i32 {
    unsafe { FB.dirty() as i32 }
}

/// Mark lcd frame buffer as clean
#[no_mangle]
pub extern "C" fn lcd_clear_dirty() {
    unsafe { FB.clear_dirty() }
}

/// Step the UI demo animation
#[no_mangle]
pub extern "C" fn demo_tick() {
    guilib::api::demo_tick(unsafe { &mut FB }, unsafe { &mut CTX });
}

/// Respond to key press event
#[no_mangle]
pub extern "C" fn keydown(key_index: i32) {
    guilib::api::keydown(unsafe { &mut FB }, unsafe { &mut CTX }, key_index as u32);
}

/// Respond to key release event
#[no_mangle]
pub extern "C" fn keyup(key_index: i32) {
    guilib::api::keyup(unsafe { &mut FB }, unsafe { &mut CTX }, key_index as u32);
}

/// Substitute for using (non-existant) menu to select azerty keyboard layout
#[no_mangle]
pub extern "C" fn set_layout_azerty() {
    guilib::api::kbd_set_layout_azerty(unsafe { &mut FB }, unsafe { &mut CTX });
}

/// Substitute for using (non-existant) menu to select qwerty keyboard layout
#[no_mangle]
pub extern "C" fn set_layout_qwerty() {
    guilib::api::kbd_set_layout_qwerty(unsafe { &mut FB }, unsafe { &mut CTX });
}

/// Export pointer to frame buffer shared memory for javascript + wasm32
#[no_mangle]
pub extern "C" fn lcd_frame_buf_ptr() -> *const u32 {
    unsafe { FB.buf.as_ptr() }
}

#[cfg(test)]
mod tests {

    #[test]
    fn one() {
        assert_eq!(1, 1);
    }
}
