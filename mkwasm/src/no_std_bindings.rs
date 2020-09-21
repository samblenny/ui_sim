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

/// Export pointer and size of shared buffer for javascript
#[no_mangle]
pub unsafe extern "C" fn utf8_buf_ptr() -> *const u8 {
    super::UTF8_BUF.as_ptr()
}
#[no_mangle]
pub unsafe extern "C" fn utf8_buf_size() -> usize {
    super::UTF8_BUF.len()
}
