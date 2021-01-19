//! Ethane is an alternative web3 implementation with the aim of being simple, slim and interoperable.

pub use eth_types::*;
pub use geth::GethConnector;
pub use transport::ws::{WebSocket, WebSocketError};

mod eth_types;
mod geth;
pub mod rpc;
mod transport;

/// Used for HTTP basic authentication during the handshake request
pub struct Credentials {
    pub username: String,
    pub password: String,
}
