use std::sync::mpsc::Receiver;
use thiserror::Error;

pub mod http;
pub mod websocket;

pub trait JsonRequest {
    fn json_request(&mut self, cmd: String) -> Result<String, TransportError>;
}

pub trait JsonSubscribe {
    fn json_subscribe(&mut self, cmd: String) -> Result<Receiver<String>, TransportError>;
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
