//! Lucita is a simple library for communicating with Ethereum nodes using the JSON-RPC over websocket.

pub use eth_types::*;
pub use geth::GethConnector;
pub use rpc::{Call, Rpc};
pub use transport::ws::{WebSocket, WebSocketError};

mod eth_types;
mod geth;
mod rpc;
mod transport;

/// Used for HTTP basic authentication during the handshake request
pub struct Credentials {
    pub username: String,
    pub password: String,
}
