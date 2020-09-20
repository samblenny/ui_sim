/// WASM Notes:
/// 1. The panic() boilerplate below allows use of no_std without wasm-bindgen
///    and wasm-pack.
/// 2. Using #[no_mangle] on public functions is necessary for linking.

#[link(wasm_import_module = "js")]
extern "C" {
    pub fn js_log_trace(code: i32);
}

/// Panic Handler for no_std.
///
/// Rust semantics require panic handler to never return, and docs for embedded
/// no_std rust suggest to accomplish that with loop {}. But, loop {} can peg
/// the CPU at 100% and make browser UI unresponsive. Better alternative is to
/// use WebAssembly unreachable trap instruction (available in stable since
/// late 2019).
use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(_panic_info: &PanicInfo) -> ! {
    unsafe {
        core::arch::wasm32::unreachable();
    }
}

/// Export pointers so js can find rust static vars in wasm VM memory buffer
#[no_mangle]
pub unsafe extern "C" fn kbd_overlay_ptr() -> *const u32 {
    &super::KBD_OVERLAY
}
#[no_mangle]
pub unsafe extern "C" fn utf8_buf_ptr() -> *const u8 {
    super::UTF8_BUF.as_ptr()
}
#[no_mangle]
pub unsafe extern "C" fn utf8_buf_size() -> usize {
    super::UTF8_BUF.len()
}
