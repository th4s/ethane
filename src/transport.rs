use std::sync::mpsc::Receiver;
use thiserror::Error;

pub mod ws;

pub trait JsonRequest {
    fn json_request(&mut self, cmd: String) -> Result<String, TransportError>;
}

pub trait JsonSubscribe {
    fn json_subscribe(&mut self, cmd: String) -> Result<Receiver<String>, TransportError>;
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("{0}")]
    WebSocketError(#[from] ws::WebSocketError),
}
