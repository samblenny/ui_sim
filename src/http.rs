//! Multi-threaded HTTP server Request struct and send_* response functions
use crate::mq;
use crate::mq::{Message, SseRx};
use std::io::Write;
use std::net::TcpStream;

/// Content type for plain text
pub const TEXT_PLAIN: &str = "text/plain;charset=utf-8";

/// Request: For passing context around while handling HTTP requests
pub struct Request<'a> {
    pub mq: &'a mq::Mq,
    pub stream: &'a TcpStream,
    pub method: &'a str,
    pub path: &'a str,
    pub query: &'a str,
}
impl Request<'_> {
    /// Initialize the context for handling an HTTP request.
    /// Subtle point: full_path parameter gets split into .path and .query
    pub fn new<'a>(
        mq: &'a mq::Mq,
        stream: &'a TcpStream,
        method: &'a str,
        full_path: &'a str,
    ) -> Request<'a> {
        let mut itpath = full_path.split('?');
        let path = itpath.next().unwrap_or(full_path);
        let query = itpath.next().unwrap_or(&"");
        Request {
            mq,
            stream,
            method,
            path,
            query,
        }
    }
}

/// Write HTTP 200 or 404 response for file to TcpStream
pub fn send_file(mut r: &mut Request, name: &str) {
    match std::fs::read_to_string(&name) {
        Ok(s) => {
            let ct = match () {
                _ if name.ends_with(".html") => &"text/html;charset=utf-8",
                _ if name.ends_with(".js") => &"text/javascript;charset=utf-8",
                _ if name.ends_with(".css") => &"text/css;charset=utf-8",
                _ if name.ends_with(".txt") => &"text/plain;charset=utf-8",
                _ => &"application/octet-stream",
            };
            send_200(&mut r, &ct, &s);
        }
        Err(e) => {
            r.mq.error(&format!("send_file() {} [{}]", name, e));
            send_404(&mut r);
        }
    }
}

/// Pipe message queue to TcpStream with HTTP Server-Sent Events protocol
pub fn pipe_mq_rx_to_sse(mut r: &mut Request, sse_rx: &mut SseRx) {
    let header = &"HTTP/1.1 200\r\nContent-Type: text/event-stream;charset=utf-8\r\n\
Access-Control-Allow-Origin: *\r\n\r\n";
    match r.method {
        "HEAD" => {
            r.mq.info(&format!("HTTP200: {} {} [SSE]", r.method, r.path));
            if let Err(e) = write!(r.stream, "{}", header) {
                r.mq.error(&format!("SSE: HTTP connection dropped [{:?}]", e));
            }
        }
        "GET" => {
            r.mq.info(&format!("HTTP200: {} {} [SSE]", r.method, r.path));
            // Send Server-Sent Events HTTP header
            if let Err(e) = write!(r.stream, "{}", header) {
                r.mq.error(&format!("SSE: HTTP connection dropped [{:?}]", e));
                return;
            }
            let _ = r.stream.flush();
            // Pipe events from sse_rx channel to TcpStream using SSE protocol
            r.mq.tx_ready(true);
            for msg in sse_rx.iter() {
                let s = match msg {
                    Message::RemoteTrace(text) => format!("event: trace\ndata: {}", text),
                    Message::RemoteTerm(text) => format!("event: term\ndata: {}", text),
                    _ => format!("event: debug\ndata: {:?}", msg),
                };
                if let Err(e) = write!(r.stream, "{}\n\n", s) {
                    r.mq.error(&format!("SSE: HTTP connection [{:?}]", e));
                    break;
                }
                let _ = r.stream.flush();
            }
            r.mq.tx_ready(false);
        }
        _ => send_400(&mut r, &"Bad Request"),
    }
}

/// Write HTTP 200 OK response to TcpStream (omitting body for HEAD)
pub fn send_200(r: &mut Request, content_type: &str, body: &str) {
    r.mq.info(&format!("HTTP200: {} {} {}", r.method, r.path, r.query));
    let con_type = &format!("Content-Type: {}", content_type);
    let con_len = &format!("Content-Length: {}", body.len());
    let cors = &"Access-Control-Allow-Origin: *";
    let header = format!(
        "HTTP/1.1 200\r\n{}\r\n{}\r\n{}\r\n\r\n",
        con_type, con_len, cors
    );
    // Omit body for HEAD method
    let body_maybe = match r.method {
        "HEAD" => &"",
        _ => body,
    };
    if let Err(e) = write!(r.stream, "{}{}", header, body_maybe) {
        r.mq.error(&format!("respond_200() write!() [{}]", e));
    }
}

/// Write HTTP 400 Bad Request response to TcpStream
pub fn send_400(r: &mut Request, reason: &str) {
    r.mq.info(&format!(
        "HTTP400: [{}] {} {} {}",
        reason, r.method, r.path, r.query
    ));
    let header = &format!("HTTP/1.1 400 {}\r\nContent-Length: 0\r\n\r\n", reason);
    if let Err(e) = write!(r.stream, "{}", header) {
        r.mq.error(&format!("respond_404() write!() [{}]", e));
    }
}

/// Write HTTP 404 Not Found response to TcpStream
pub fn send_404(r: &mut Request) {
    r.mq.info(&format!("HTTP404: {} {}", r.method, r.path));
    if let Err(e) = write!(r.stream, "HTTP/1.1 404\r\nContent-Length: 0\r\n\r\n") {
        r.mq.error(&format!("respond_404() write!() [{}]", e));
    }
}

/// Write HTTP 501 Not Implemented response to TcpStream
pub fn send_501(mq: &mq::Mq, mut stream: TcpStream, first_line: &str) {
    mq.info(&format!("HTTP501: {}", first_line));
    if let Err(e) = write!(stream, "HTTP/1.1 501\r\nContent-Length: 0\r\n\r\n") {
        mq.error(&format!("respond_501() write!() [{}]", e));
    }
}
