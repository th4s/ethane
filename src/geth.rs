use crate::rpc::{Call, CallError, Rpc};
use crate::transport::ws::{WebSocket, WebSocketError};
use crate::transport::{JsonRequest, TransportError};
use crate::Credentials;
use log::{debug, trace};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::VecDeque;
use std::fmt::Debug;
use thiserror::Error;

pub struct GethConnector<T: JsonRequest>(T, VecDeque<u32>);

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

impl<T: JsonRequest> Call for GethConnector<T> {
    fn call<U: DeserializeOwned + Debug>(&mut self, mut rpc: Rpc<U>) -> Result<U, CallError> {
        let command_id = self.get_command_id()?;
        rpc.id = command_id;
        debug!("Calling rpc method: {:?}", &rpc);
        let response = self.send_request(&rpc)?;
        self.1.push_back(command_id);
        deserialize(&response).map_err(|err| err.into())
    }
}

impl<T: JsonRequest> GethConnector<T> {
    fn get_command_id(&mut self) -> Result<u32, GethError> {
        trace!("Retrieving id from pool...");
        match self.1.pop_front() {
            Some(inner) => Ok(inner),
            None => Err(GethError::NoTicketId),
        }
    }

    fn send_request<U: DeserializeOwned + Debug>(
        &mut self,
        rpc: &Rpc<U>,
    ) -> Result<String, GethError> {
        trace!("Sending request...");
        let response = self.0.json_request(serde_json::to_string(rpc)?)?;

        if !response.contains(&format!("\"id\":{}", rpc.id)) {
            return Err(GethError::WrongId);
        }
        Ok(response)
    }
}

fn deserialize<U: DeserializeOwned + Debug>(response: &str) -> Result<U, GethError> {
    trace!("Deserializing response {}", response);
    match serde_json::from_str::<Response<U>>(response) {
        Ok(Response {
            result_or_error: inner,
            ..
        }) => match inner {
            RpcResult::Result(result) => Ok(result),
            RpcResult::Error(err) => Err(GethError::JsonRpc(err)),
        },
        Err(err) => Err(GethError::from(err)),
    }
}

#[derive(Deserialize, Debug)]
pub struct JsonError {
    code: i32,
    message: String,
}

#[derive(Debug, Error)]
pub enum GethError {
    #[error("Geth Error: {0}")]
    Initialization(WebSocketError),
    #[error("Geth Error: No connection left in connection pool")]
    NoTicketId,
    #[error("Geth Error: {0:?}")]
    JsonRpc(JsonError),
    #[error("Geth Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Geth Error: {0}")]
    TransportError(#[from] TransportError),
    #[error("Geth Error: Received an unexpected message id")]
    WrongId,
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    pub id: u32,
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    #[serde(flatten)]
    pub result_or_error: RpcResult<T>,
}

#[derive(Deserialize, Debug)]
enum RpcResult<T> {
    #[serde(rename = "result")]
    Result(T),
    #[serde(rename = "error")]
    Error(JsonError),
}
