//! Provides channel message queue communciation for coordinating threads
#![allow(dead_code)]
use std::sync::mpsc::{Receiver, Sender};

/// Message types
#[derive(Debug, Clone)]
pub enum Message {
    LogError(String),
    LogInfo(String),
    KbdScanCode(String),
    Repaint(String),
    TxReady(bool, u32),
}

/// Type for event loop inbound channel's Sender
pub type EventLoopTx = Sender<Message>;

/// Type for event loop inbound channel's Receiver
pub type EventLoopRx = Receiver<Message>;

/// Type for HTTP Server-Sent Event (SSE) channel's Receiver
pub type SseRx = Receiver<Message>;

/// Mq can model:
/// 1. Channel endpoint from Event loop thread's point of view:
///    - Outbound tx to server thread needs flow control
/// 2. Channel endpoint from Server thread's point of view:
///    - Outbound tx flow control to event loop is always ready
pub struct Mq {
    /// Outbound to queue from owning thread
    tx: Sender<Message>,
    /// Flow control for outbound messages
    tx_ready: bool,
    /// Identifier for this thread (used in tx flow control)
    tid: u32,
}
impl Mq {
    /// Initialize
    pub fn new(tx: Sender<Message>, tx_ready: bool, tid: u32) -> Mq {
        Mq { tx, tx_ready, tid }
    }
    /// Send a message subject to flow control
    pub fn send(&self, msg: Message) {
        if self.tx_ready {
            let _ = self.tx.send(msg);
        }
    }
    /// Send keyboard scancode message to keyboard driver sink
    pub fn kbd_driver(&self, scancode: &str) {
        self.send(Message::KbdScanCode(String::from(scancode)));
    }
    /// Send string to the error log sink
    pub fn error(&self, message: &str) {
        self.send(Message::LogError(String::from(message)));
    }
    /// Send string to the info log sink
    pub fn info(&self, message: &str) {
        self.send(Message::LogInfo(String::from(message)));
    }
    /// Send drawing commands to screen repaint sink
    pub fn repaint(&self, message: &str) {
        self.send(Message::Repaint(String::from(message)));
    }
    /// Send flow control status to tx_ready sink.
    /// Subtle point: Possible confusion here about point of view. tx_ready()
    /// has nothing to do with self.tx_ready. The purpose is to inform the
    /// thread on the other end of the channel that it should update its value
    /// of Mq.tx_ready in the struct associated with thread of id=self.tid
    pub fn tx_ready(&self, ready: bool) {
        self.send(Message::TxReady(ready, self.tid));
    }
    /// Get thread id for owner of this Mq (self)
    pub fn tid(&self) -> u32 {
        self.tid
    }
    /// Update self.tx_ready (intende for responding to Message::TxReady)
    pub fn set_tx_ready(&mut self, ready: bool) {
        self.tx_ready = ready;
    }
}
