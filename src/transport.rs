use thiserror::Error;

pub mod http;
pub mod websocket;

pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, TransportError>;
}

/// Used for HTTP basic authentication during the handshake request
#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("{0}")]
    WebSocketError(#[from] websocket::WebSocketError),
    #[error("{0}")]
    HttpError(#[from] http::HttpError),
}
