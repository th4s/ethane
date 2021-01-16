use thiserror::Error;

pub mod ws;

pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, TransportError>;
}

// pub trait Subscribe {
//     fn subscribe(&mut self, cmd: String) -> Result<Receiver<String>, Box<dyn Error>>;
// }

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("{0}")]
    WebSocketError(#[from] ws::WebSocketError),
}
