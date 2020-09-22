#![cfg_attr(target_arch = "wasm32", no_std)]
//! Provide minimalist debug tracing that works even in wasm32. Since wasm
//! browser sandboxes currently do not have stderr, debug logging must go
//! through a javascript function binding. Passing strings from wasm to
//! javascript is possible, but it adds lots of complexity. Simplest option is
//! function call with i32 argument.

/// For wasm32 build, bind debug trace to a javascript function call
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "js")]
extern "C" {
    pub fn js_log_trace(code: i32);
}

/// For non-wasm32 builds, replace javascript call with eprintln!
#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_log_trace(code: i32) {
    eprintln!("{}", code);
}

/// For logging control flow (printf style debugging)
#[allow(dead_code)]
pub fn log(n: i32) {
    unsafe {
        js_log_trace(n as i32);
    }
}

/// For logging error and status codes
pub fn log_code(code: Code) {
    unsafe {
        js_log_trace(code as i32);
    }
}

/// Error and status codes
pub enum Code {
    BadKeyIndex = -1,
    BadModkeyDownR = -2,
}
