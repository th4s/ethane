//! Possible transports which are supported by the [Connector](crate::Connector)

use thiserror::Error;

pub use self::http::{Http, HttpError};
pub use websocket::{WebSocket, WebSocketError};
mod http;
mod uds;
mod websocket;

///Implemented by transports which offer request functionality
pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, TransportError>;
}

/// Credentials can be used for authentication
///
/// Use this when creating a [connector](crate::Connector). Supports Basic and Bearer authentication.
/// So you can easily add HTTP Basic or JWT authentication. This will add an authorization header
/// to your requests and works for [websockets](crate::Connector::websocket) and
/// [http requests](crate::Connector::http).
#[derive(Debug, Clone)]
pub enum Credentials {
    Bearer(String),
    Basic(String),
}

impl Credentials {
    pub fn to_auth_string(&self) -> String {
        match self {
            Self::Bearer(token) => String::from("Bearer ") + &token,
            Self::Basic(token) => String::from("Basic ") + &token,
        }
    }
}

/// Wraps the different transport errors that can happen
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum TransportError {
    #[error("{0}")]
    WebSocketError(#[from] websocket::WebSocketError),
    #[error("{0}")]
    HttpError(#[from] http::HttpError),
    #[error("{0}")]
    UdsError(#[from] uds::UdsError),
}
