use log::{debug, error};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use thiserror::Error;

/// An interprocess connection using a unix domain socket
pub struct Uds {
    pub path: String,
    stream: UnixStream,
}

impl Uds {
    pub(crate) fn new(path: &str) -> Result<Self, UdsError> {
        debug!("Opening connection to unix domain socket: {}", path);
        Ok(Self {
            path: String::from(path),
            stream: UnixStream::connect(path).map_err(UdsError::UdsInit)?,
        })
    }
}

impl Drop for Uds {
    fn drop(&mut self) {
        debug!("Closing unix domain socket connection");
        let close = self.stream.shutdown(Shutdown::Both);
        if let Err(err) = close {
            error!("{}", err);
        }
    }
}

/// An error type collecting what can go wrong with unix domain sockets
#[derive(Debug, Error)]
pub enum UdsError {
    #[error("Uds Error: {0}")]
    UdsInit(std::io::Error),
    #[error("Uds Error: {0}")]
    UdsClose(std::io::Error),
    #[error("Uds Error: {0}")]
    Read(std::io::Error),
}
