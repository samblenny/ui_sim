/// WASM Notes:
/// 1. The panic() boilerplate below allows use of no_std without wasm-bindgen
///    and wasm-pack.
/// 2. Using #[no_mangle] on public functions is necessary for linking.

/// Panic Handler for no_std.
///
/// Rust docs suggest `loop {}`, but that can max CPU and make browser UI
/// unresponsive. WebAssembly unreachable trap instruction is better (available
/// in stable since late 2019).
use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(_panic_info: &PanicInfo) -> ! {
    unsafe {
        core::arch::wasm32::unreachable();
    }
}

extern crate blit;

/// Export pointer and size of shared frame buffer for javascript
#[no_mangle]
pub unsafe extern "C" fn lcd_words_per_line() -> i32 {
    blit::LCD_WORDS_PER_LINE as i32
}
#[no_mangle]
pub unsafe extern "C" fn lcd_px_per_line() -> i32 {
    blit::LCD_PX_PER_LINE as i32
}
#[no_mangle]
pub unsafe extern "C" fn lcd_lines() -> i32 {
    blit::LCD_LINES as i32
}
#[no_mangle]
pub unsafe extern "C" fn lcd_frame_buf_ptr() -> *const u32 {
    super::LCD_FRAME_BUF.as_ptr()
}
