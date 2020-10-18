#![no_std]
extern crate gui;
extern crate kbd;
extern crate trace;

use gui::{state, views};

/// For building wasm32 no_std, add panic handler and functions to let
/// javascript check shared buffer pointers. This panic handler conflicts with
/// test panic handler and therefore cannot be included during `cargo test`.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

/// Initialize the hardware (splash screen, etc.)
#[no_mangle]
pub extern "C" fn init() {
    paint_home_screen();
}

/// Draw the home screen, incorporating current global state (slots)
fn paint_home_screen() {
    unsafe {
        views::home_screen(&mut state::LCD_FRAME_BUF);
    }
    lcd_set_dirty();
}

/// Check if lcd frame buffer is dirty: 0=clean, 1=dirty
#[no_mangle]
pub extern "C" fn lcd_dirty() -> i32 {
    unsafe {
        if state::LCD_DIRTY > 0 {
            1
        } else {
            0
        }
    }
}

/// Mark lcd frame buffer as clean
#[no_mangle]
pub extern "C" fn lcd_clear_dirty() {
    unsafe {
        state::LCD_DIRTY = 0;
    }
}

/// Mark lcd frame buffer as dirty
fn lcd_set_dirty() {
    unsafe {
        state::LCD_DIRTY = 1;
    }
}

/// Respond to a cyle radio event (intended for UI demonstration)
#[no_mangle]
pub extern "C" fn cycle_radio() {
    state::status::cycle_radio();
    paint_home_screen();
}

/// Respond to a cyle radio event (intended for UI demonstration)
#[no_mangle]
pub extern "C" fn cycle_battery() {
    state::status::cycle_battery();
    paint_home_screen();
}

/// Respond to key press event
#[no_mangle]
pub extern "C" fn keydown(key_index: i32) {
    if key_index < 0 || key_index >= kbd::MAP_SIZE as i32 {
        trace::log_code(trace::Code::BadKeyIndex);
        return;
    }
    let result = &kbd::cur_map_lut()[key_index as usize];
    let mut dirty = false;
    match result {
        kbd::R::C(c) => {
            state::home::buffer_keystroke(*c);
            dirty = true;
        }
        kbd::R::AltL => {
            kbd::modkey_down(result);
            dirty = true;
        }
        kbd::R::AltR => {
            kbd::modkey_down(result);
            dirty = true;
        }
        kbd::R::Shift => {
            kbd::modkey_down(result);
            dirty = true;
        }
        _ => (),
    }
    if dirty {
        paint_home_screen();
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
    kbd::set_modkey(kbd::ModKey::Base);
    paint_home_screen();
}

/// Substitute for using (non-existant) menu to select qwerty keyboard layout
#[no_mangle]
pub extern "C" fn set_layout_qwerty() {
    kbd::set_layout(kbd::Layout::Qwerty);
    kbd::set_modkey(kbd::ModKey::Base);
    paint_home_screen();
}

#[cfg(test)]
mod tests {

    #[test]
    fn one() {
        assert_eq!(1, 1);
    }
}
