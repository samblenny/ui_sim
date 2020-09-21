//! Wasm32 browser sandboxes currently do not have stderr, so debug logging
//! must go through a javascript function binding.

/// For wasm32 build, bind debug trace to a javascript function call
#[cfg(target_arch = "wasm32")]
use crate::no_std_bindings::js_log_trace;

/// For other builds (test), replace debug trace binding with stub
#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_log_trace(_: i32) {}

/// For logging control flow
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
