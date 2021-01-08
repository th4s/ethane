//! Lucita is a simple library for communicating with Ethereum nodes using the JSON-RPC over websocket.

pub use ws::{Credentials, WebSocket, WebSocketError};

mod rpc;
pub mod ws;
