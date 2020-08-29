//! UI Simulation for handheld device with LCD and switchable keyboard layouts
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

mod http;
mod mq;
use mq::{EventLoopTx, EventLoopRx, SseRx};

const WEB_SERVER_BIND: &str = "127.0.0.1:8000";
const WEB_SERVER_THREADS: usize = 3;

/// Main: Spawn server threads, start event loop to route messages.
/// Channel Message Queue Plan:
/// 1. Inbound Channel: Server threads each get a moved clone of inbound
///    channel's Sender. Servers share inbound queue connected to event loop's
///    single Receiver.
/// 2. Outbound messages are broadcast to server threads, subject to
///    per-channel flow control as requested by server threads.
/// 3. Outbound Channels: Because Receivers cannot be cloned, event loop uses
///    separate outbound channel for each server. Event loop thread keeps
///    ownership of Senders. Receivers are moved to server threads.
fn main() {
    // Create shared inbound channel and webserver listener
    let (in_tx, in_rx) = mpsc::channel::<mq::Message>();
    let listener = TcpListener::bind(WEB_SERVER_BIND).unwrap();
    // For each server thread...
    let mut loop_to_server_mqs = Vec::<mq::Mq>::new();
    for tid in 0..WEB_SERVER_THREADS {
        // Create outbound channel for this thread
        let (out_tx, out_rx) = mpsc::channel::<mq::Message>();
        let loop_mq = mq::Mq::new(out_tx, false, tid as u32);
        loop_to_server_mqs.push(loop_mq);
        // Clone shared inbound Sender and webserver listener
        let in_tx = in_tx.clone();
        let t_listener = listener.try_clone().unwrap();
        // Start webserver thread
        let _ = thread::spawn(move || {
            let mq = mq::Mq::new(in_tx, true, tid as u32);
            let mut sse_rx = out_rx as SseRx;
            mq.info(&format!("Server {} ready: http://{}", tid, WEB_SERVER_BIND));
            web_server(&mq, &mut sse_rx, t_listener);
        });
    }
    // Start event loop
    event_loop(in_rx as EventLoopRx, in_tx as EventLoopTx, &mut loop_to_server_mqs);
}

/// Event loop for main thread
fn event_loop(in_rx: EventLoopRx, in_tx: EventLoopTx, mqs_to_servers: &mut Vec<mq::Mq>) {
    for message in in_rx.iter() {
        match message {
            mq::Message::LogError(msg) => println!("Err: {}", msg),
            mq::Message::LogInfo(msg) => println!("{}", msg),
            mq::Message::KbdScanCode(sc) => {
                let _ = in_tx.send(mq::Message::Repaint(format!("text: {}", sc)));
                println!("Keyscan: {}", sc);
            }
            mq::Message::Repaint(msg) => {
                for mq in mqs_to_servers.iter_mut() {
                    mq.send(mq::Message::Repaint(msg.clone()));
                }
                println!("Repaint: {:?}", msg);
            }
            mq::Message::TxReady(ready, tid) => {
                for mq in mqs_to_servers.iter_mut() {
                    if mq.tid() == tid {
                        mq.set_tx_ready(ready)
                    }
                }
                println!("TxReady(ready:{},tid:{})", ready, tid);
            }
        }
    }
}

/// HTTP/1.1 web server for one request per connection (no keep-alive)
fn web_server(mq: &mq::Mq, mut sse_rx: &mut SseRx, listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connect(&mq, &mut sse_rx, s),
            Err(e) => mq.error(&format!("web_server() .incoming() [{}]", e)),
        }
    }
}

/// Attempt to read HTTP header from TcpStream
fn handle_connect(mq: &mq::Mq, mut sse_rx: &mut SseRx, mut stream: TcpStream) {
    let mut request_buf = [0; 3000];
    let max_attempts = 20;
    let mut header_too_big = false;
    let mut tail = 0;
    // Allow for possibility of header split across multiple reads.
    for _ in 0..max_attempts {
        match stream.read(&mut request_buf[tail..]) {
            Ok(bytes_read) => {
                tail += bytes_read;
                if contains_blank_line(&request_buf[..tail]) {
                    header_too_big = false;
                    break;
                }
            }
            Err(e) => {
                mq.error(&format!("handle_connect() .read() [{}]", e));
                break;
            }
        }
    }
    if header_too_big {
        mq.error(&"handle_connect(): header too big; closing connection");
    } else {
        match std::str::from_utf8(&request_buf[0..tail]) {
            Ok(request) => handle_request(&mq, &mut sse_rx, stream, request),
            Err(e) => mq.error(&format!("handle_connection() ::from_utf8() [{}]", e)),
        }
    }
}

/// Test if byte array contains a blank line
/// For manual requests with netcat, use `nc -c ...` to make CRLF line endings.
fn contains_blank_line(buf: &[u8]) -> bool {
    for window in buf.windows(4) {
        // Look for "\r\n\r\n"
        if window == [13, 10, 13, 10] {
            return true;
        }
    }
    return false;
}

/// Handle an HTTP request (possible: GET, HEAD, POST, or 501)
/// Potential surprises:
/// 1. This checks only first line of header. Discarding remainder of
///    request creates requirement to pass POST data in a ?query_string.
///    POSTing path with query is valid HTTP/1.1 and useful for this
///    purpose, but it does not obey conventions for HTML forms.
/// 2. This omits all header based security checks. As long as this server
///    only binds to localhost, skipping the checks should be fine.
fn handle_request(mq: &mq::Mq, mut sse_rx: &mut SseRx, stream: TcpStream, request: &str) {
    let first_line = request.lines().next().unwrap_or(&"").trim();
    let mut itfl = first_line.split(' ');
    let method = itfl.next().unwrap_or(&"");
    let full_path = itfl.next().unwrap_or(&"");
    let protocol = itfl.next().unwrap_or(&"");
    match (method, full_path, protocol) {
        ("GET", fp, "HTTP/1.1") => {
            let mut r = http::Request::new(mq, &stream, &"GET", &fp);
            handle_route(&mut r, &mut sse_rx);
        }
        ("HEAD", fp, "HTTP/1.1") => {
            let mut r = http::Request::new(mq, &stream, &"HEAD", &fp);
            handle_route(&mut r, &mut sse_rx)
        }
        ("POST", fp, "HTTP/1.1") => {
            let mut r = http::Request::new(mq, &stream, &"POST", &fp);
            handle_route(&mut r, &mut sse_rx);
        }
        _ => http::send_501(&mq, stream, &first_line),
    }
}

/// Handle HTTP request for GET/HEAD/POST.
/// Possible surprise:
/// - Keyscan enpoint expects POST to /io/kbd/scancode?query where entire query
///   after "?" is scancode (not multipart/form-data in body, nor ?key=value).
///   This is valid by HTTP spec but does not follow HTML form conventions.
/// Example JS:
///   for (const k of ['P13p', P13r', 'P14p', 'P14r']) {
///      fetch('http://localhost:8000/io/kbd/scancode?'+k, {method: 'POST'});
///   }
fn handle_route(mut r: &mut http::Request, mut sse_rx: &mut SseRx) {
    match r.method {
        "HEAD" | "GET" => match r.path {
            "/" => http::send_file(&mut r, &"www/index.html"),
            "/main.js" => http::send_file(&mut r, &"www/main.js"),
            "/bkit.js" => http::send_file(&mut r, &"www/bkit.js"),
            "/bkbd.js" => http::send_file(&mut r, &"www/bkbd.js"),
            "/style.css" => http::send_file(&mut r, &"www/style.css"),
            "/io/screen" => handle_io_screen(&mut r, &mut sse_rx),
            _ => http::send_404(&mut r),
        },
        "POST" => match (r.path, r.query) {
            ("/io/kbd/scancode", sc) => handle_io_scancode(&mut r, sc),
            _ => http::send_404(&mut r),
        },
        _ => http::send_404(&mut r),
    }
}

/// Handle GET request for /io/screen with Server Sent Events (SSE). This will
/// pipe messages received from message queue to a long-lived connection that
/// follows HTTP SSE protocol and generates javascript events on client side.
fn handle_io_screen(mut r: &mut http::Request, mut sse_mq: &mut SseRx) {
    http::pipe_mq_rx_to_sse(&mut r, &mut sse_mq);
}

/// Handle POST request for a keyboard scancode
fn handle_io_scancode(mut r: &mut http::Request, scancode: &str) {
    if scancode.len() == 4 {
        r.mq.kbd_driver(scancode);
        http::send_200(&mut r, http::TEXT_PLAIN, &"OK");
    } else {
        http::send_400(&mut r, &"Bad Scancode");
    }
}
