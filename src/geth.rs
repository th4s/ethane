use crate::rpc::{Call, CallError, Response, Rpc};
use crate::transport::ws::{WebSocket, WebSocketError};
use crate::transport::{Request, TransportError};
use crate::Credentials;
use log::{debug, trace};
use serde::de::DeserializeOwned;
use std::collections::VecDeque;
use std::fmt::Debug;
use thiserror::Error;

pub struct GethConnector<T: Request>(T, VecDeque<u32>);

impl GethConnector<WebSocket> {
    pub fn ws(domain: &str, credentials: Option<Credentials>) -> Result<Self, GethError> {
        debug!("Connecting to geth node");
        Ok(GethConnector(
            WebSocket::new(domain, credentials).map_err(|err| GethError::Initialization(err))?,
            (0..1000).collect(),
        ))
    }

    pub fn close(&mut self) -> Result<(), WebSocketError> {
        self.0.close()
    }
}

impl<T: Request> Call for GethConnector<T> {
    fn call<U: DeserializeOwned + Debug>(&mut self, mut rpc: Rpc<U>) -> Result<U, CallError> {
        trace!("Calling rpc method: {:?}", &rpc);
        let command_id = match self.1.pop_front() {
            Some(inner) => inner,
            None => return Err(GethError::NoTicketId.into()),
        };
        rpc.id(command_id);

        let response = self
            .0
            .request(rpc.command)
            .map_err(|err| CallError::from(GethError::from(err)))?;
        let response = match serde_json::from_str::<Response<U>>(&response) {
            Ok(Response {
                error: Some(err), ..
            }) => return Err(GethError::JsonRpc(err).into()),
            Ok(other) => match other.result {
                Some(inner) => Ok(inner),
                None => return Err(GethError::NoResult.into()),
            },
            Err(err) => return Err(CallError::from(GethError::from(err))),
        };

        self.1.push_back(command_id);
        response
    }
}

#[derive(Debug, Error)]
pub enum GethError {
    #[error("Geth Initialization Error: {0}")]
    Initialization(WebSocketError),
    #[error("No ticket id left")]
    NoTicketId,
    #[error("Json RPC Error: {0}")]
    JsonRpc(String),
    #[error("No result provided")]
    NoResult,
    #[error("Deserialization Error: {0}")]
    Deserialization(#[from] serde_json::Error),
    #[error("Geth Error: {0}")]
    TransportError(#[from] TransportError),
}
