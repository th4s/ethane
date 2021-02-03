//! Possible transports which are supported by the [Connector](crate::Connector)

use thiserror::Error;

pub use self::http::{Http, HttpError};
pub use websocket::{WebSocket, WebSocketError};
mod http;
mod websocket;

///Implemented by transports which offer request functionality
pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, TransportError>;
}

/// Can be used for authentication
#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// An error type collecting what can go wrong during transport
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum TransportError {
    #[error("{0}")]
    WebSocketError(#[from] websocket::WebSocketError),
    #[error("{0}")]
    HttpError(#[from] http::HttpError),
}
