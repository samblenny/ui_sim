#![no_std]
extern crate kbddrv;

// Always include IPC shared memory buffer stuff
pub mod ipc_mem;

pub mod constants {
    pub const BUF_SIZE: usize = 150;
}

// For building wasm32 no_std, add panic handler and imports/exports for
// functions used in IPC between WebAssembly and Javascript. This panic handler
// cannot be included for `cargo test` because it would conflict with the test
// panic handler.
#[cfg(target_arch = "wasm32")]
pub mod no_std_bindings;

// For wasm32 build, use debug trace WebAssembly IPC function binding
#[cfg(target_arch = "wasm32")]
use no_std_bindings::js_log_trace;

// For other builds (test), replace debug trace binding with stub
#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_log_trace(_: i32) {}

// Writer decouples query response formatting from stream IO implementation details.
pub trait Writer {
    fn write(&mut self, message: &str);
    fn trace(&mut self, trace_code: i32);
    fn to_s(&self) -> &str;
}

// BufWriter is a Writer for string slices backed by stack allocated [u8].
pub struct BufWriter {
    buf: [u8; constants::BUF_SIZE],
    buf_pos: usize,
}
impl BufWriter {
    // Return empty buffer ready for use.
    pub fn new() -> BufWriter {
        BufWriter {
            buf: [0; constants::BUF_SIZE],
            buf_pos: 0,
        }
    }
    // Truncate buffer position back to 0 bytes.
    pub fn rewind(&mut self) {
        self.buf_pos = 0;
    }
}
impl Writer for BufWriter {
    // Append message to buffer
    fn write(&mut self, message: &str) {
        for b in message.bytes() {
            // TODO: better strategy for overflow (vs. silently drop extra)
            if self.buf_pos < self.buf.len() {
                self.buf[self.buf_pos] = b;
                self.buf_pos += 1;
            }
        }
    }

    // Ignore traces
    fn trace(&mut self, _: i32) {}

    // Return string slice of buffer contents.
    fn to_s(&self) -> &str {
        match core::str::from_utf8(&self.buf[0..self.buf_pos]) {
            Ok(s) => &s,
            Err(_) => &"", // TODO: handle mal-formed utf8 strings better
        }
    }
}

// IPCWriter is a Writer for UTF-8 bytes backed by static IPC shared memory.
struct IPCWriter {}
impl Writer for IPCWriter {
    fn write(&mut self, message: &str) {
        ipc_mem::write(message);
    }

    // Log trace codes to the javascript console to help debug control flow.
    fn trace(&mut self, trace_code: i32) {
        unsafe {
            js_log_trace(trace_code);
        }
    }

    fn to_s(&self) -> &str {
        ipc_mem::out_to_s()
    }
}

// Receive query message, search, write results to IPC out buffer.
// This is for calling from Javascript with WebAssembly.
// Returns: number of bytes written to IPC out buffer.
#[no_mangle]
pub extern "C" fn query_shared_mem_ipc(n: usize) -> usize {
    let mut ipc_writer = IPCWriter {};
    let qry = ipc_mem::get_query(n);
    ipc_mem::rewind();
    crate::look_up(&qry, &mut ipc_writer);
    ipc_mem::position()
}

pub fn look_up(query_bytes: &str, sink: &mut impl Writer) {
    sink.write(&query_bytes);
}

#[cfg(test)]
mod tests {
    use super::constants;
    use super::ipc_mem;

    // Send query string to ime-engine; THIS IS NOT THREAD SAFE.
    // Returns: reply string.
    fn query(qry: &str) -> &str {
        // Encode UTF-8 bytes to inbox buffer
        let mut i: usize = 0;
        unsafe {
            for b in qry.bytes() {
                if i < constants::BUF_SIZE {
                    ipc_mem::IN[i] = b;
                    i += 1;
                }
            }
        }
        // Run query
        let ipc_query_len = i;
        let _ = crate::query_shared_mem_ipc(ipc_query_len);
        // Decode reply string as UTF-8 bytes from IPC shared mem OUT buffer
        let ipc_reply = ipc_mem::out_to_s();
        ipc_reply
    }

    #[test]
    fn min_query() {
        assert_eq!("", query(&""));
    }

    #[test]
    fn max_query() {
        let buf_max = ['A' as u8; constants::BUF_SIZE];
        let qry_max = core::str::from_utf8(&buf_max).unwrap();
        // This should be passed through unchanged as ASCII
        assert_eq!(qry_max, query(qry_max));
    }

    #[test]
    fn max_query_plus_1_truncate() {
        let buf_max = ['A' as u8; constants::BUF_SIZE];
        let qry_max = core::str::from_utf8(&buf_max).unwrap();
        let buf_1_too_big = ['A' as u8; constants::BUF_SIZE + 1];
        let qry_1_too_big = core::str::from_utf8(&buf_1_too_big).unwrap();
        // This should truncate the query
        assert_eq!(qry_max, query(qry_1_too_big));
    }
}
