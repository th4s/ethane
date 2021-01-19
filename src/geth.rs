use crate::rpc::{Call, CallError, JsonError, Response, Rpc};
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
        debug!("Connecting to geth node...");
        Ok(GethConnector(
            WebSocket::new(domain, credentials).map_err(GethError::Initialization)?,
            (0..1000).collect(),
        ))
    }

    pub fn close(&mut self) -> Result<(), WebSocketError> {
        self.0.close()
    }
}

impl<T: Request> Call for GethConnector<T> {
    fn call<U: DeserializeOwned + Debug>(&mut self, mut rpc: Rpc<U>) -> Result<U, CallError> {
        let command_id = self.get_command_id()?;
        rpc.id(command_id);
        debug!("Calling rpc method: {:?}", &rpc);
        let response = self.send_request(rpc, command_id)?;
        self.1.push_back(command_id);
        deserialize(&response).map_err(|err| err.into())
    }
}

impl<T: Request> GethConnector<T> {
    fn get_command_id(&mut self) -> Result<u32, GethError> {
        trace!("Retrieving id from pool...");
        match self.1.pop_front() {
            Some(inner) => Ok(inner),
            None => Err(GethError::NoTicketId),
        }
    }

    fn send_request<U: DeserializeOwned + Debug>(
        &mut self,
        rpc: Rpc<U>,
        command_id: u32,
    ) -> Result<String, GethError> {
        trace!("Sending request...");
        let response = self.0.request(rpc.command)?;

        if !response.contains(&format!("\"id\":{}", command_id)) {
            return Err(GethError::WrongId);
        }
        Ok(response)
    }
}

fn deserialize<U: DeserializeOwned + Debug>(response: &str) -> Result<U, GethError> {
    trace!("Deserializing response {}", response);
    match serde_json::from_str::<Response<U>>(&response) {
        Ok(Response {
            error: Some(err), ..
        }) => Err(GethError::JsonRpc(err)),
        Ok(other) => match other.result {
            Some(inner) => Ok(inner),
            None => Err(GethError::NoResult),
        },
        Err(err) => Err(GethError::from(err)),
    }
}

#[derive(Debug, Error)]
pub enum GethError {
    #[error("Geth Error: {0}")]
    Initialization(WebSocketError),
    #[error("Geth Error: No connection left in connection pool")]
    NoTicketId,
    #[error("Geth Error: {0:?}")]
    JsonRpc(JsonError),
    #[error("Geth Error: No result provided")]
    NoResult,
    #[error("Geth Error: {0}")]
    Deserialization(#[from] serde_json::Error),
    #[error("Geth Error: {0}")]
    TransportError(#[from] TransportError),
    #[error("Geth Error: Received an unexpected message id")]
    WrongId,
}
