#![no_std]
extern crate guilib;
extern crate trace;

/// For building wasm32 no_std, add panic handler and functions to let
/// javascript check shared buffer pointers. This panic handler conflicts with
/// test panic handler and therefore cannot be included during `cargo test`.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

/// Initialize the hardware (splash screen, etc.)
#[no_mangle]
pub extern "C" fn init() {
    guilib::api::repaint();
}

/// Check if lcd frame buffer is dirty: 0=>clean, _=>dirty
#[no_mangle]
pub extern "C" fn lcd_dirty() -> i32 {
    guilib::api::lcd_dirty() as i32
}

/// Mark lcd frame buffer as clean
#[no_mangle]
pub extern "C" fn lcd_clear_dirty() {
    guilib::api::lcd_clear_dirty();
}

/// Step the UI demo animation
#[no_mangle]
pub extern "C" fn demo_tick() {
    guilib::api::demo_tick();
}

/// Respond to key press event
#[no_mangle]
pub extern "C" fn keydown(key_index: i32) {
    guilib::api::keydown(key_index as u32);
}

/// Respond to key release event
#[no_mangle]
pub extern "C" fn keyup(key_index: i32) {
    guilib::api::keyup(key_index as u32);
}

/// Substitute for using (non-existant) menu to select azerty keyboard layout
#[no_mangle]
pub extern "C" fn set_layout_azerty() {
    guilib::api::kbd_set_layout_azerty();
}

/// Substitute for using (non-existant) menu to select qwerty keyboard layout
#[no_mangle]
pub extern "C" fn set_layout_qwerty() {
    guilib::api::kbd_set_layout_qwerty();
}

#[cfg(test)]
mod tests {

    #[test]
    fn one() {
        assert_eq!(1, 1);
    }
}
