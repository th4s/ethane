use crate::transport::{Request, Subscribe, TransportError};
use log::{debug, error, trace};
use std::io::{BufRead, BufReader, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::str;
use thiserror::Error;

/// An interprocess connection using a unix domain socket
pub struct Uds {
    pub path: String,
    read_stream: BufReader<UnixStream>,
    write_stream: UnixStream,
}

impl Uds {
    pub(crate) fn new(path: String) -> Result<Self, UdsError> {
        debug!("Opening connection to unix domain socket: {}", &path);
        let write_stream = UnixStream::connect(path.clone()).map_err(UdsError::UdsInit)?;
        let read_stream = write_stream.try_clone().map_err(UdsError::UdsInit)?;
        Ok(Self {
            path,
            read_stream: BufReader::new(read_stream),
            write_stream,
        })
    }

    fn read_json(&mut self) -> Result<String, UdsError> {
        let mut buffer = Vec::<u8>::new();
        loop {
            let _read_bytes = self
                .read_stream
                .read_until(b'}', &mut buffer)
                .map_err(UdsError::Read)?;
            let utf8_slice = str::from_utf8(&buffer).map_err(UdsError::Utf8)?;
            if utf8_slice.matches('{').count() == utf8_slice.matches('}').count() {
                trace!("Reading from Unix domain socket: {}", utf8_slice);
                break Ok(utf8_slice.to_string());
            }
        }
    }

    fn write(&mut self, message: String) -> Result<(), UdsError> {
        trace!("Writing to Unix domain socket: {}", &message);
        let _write = self
            .write_stream
            .write_all(message.as_bytes())
            .map_err(UdsError::Write)?;
        let _flush = self.write_stream.flush().map_err(UdsError::Write)?;
        Ok(())
    }
}

impl Request for Uds {
    fn request(&mut self, cmd: String) -> Result<String, TransportError> {
        let _write = self.write(cmd)?;
        self.read_json().map_err(TransportError::UdsError)
    }
}

impl Subscribe for Uds {
    fn read_next(&mut self) -> Result<String, TransportError> {
        self.read_json().map_err(TransportError::UdsError)
    }

    fn fork(&self) -> Result<Self, TransportError>
    where
        Self: Sized,
    {
        Self::new(self.path.clone()).map_err(TransportError::from)
    }
}

impl Drop for Uds {
    fn drop(&mut self) {
        debug!("Closing unix domain socket connection");
        let close = self.write_stream.shutdown(Shutdown::Both);
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
    #[error("Uds Error: {0}")]
    Utf8(std::str::Utf8Error),
    #[error("Uds Error: {0}")]
    Write(std::io::Error),
}
