use crate::rpc::Rpc;
use crate::transport::ws::{WebSocket, WebSocketError};
use crate::transport::{JsonRequest, TransportError};
use crate::Credentials;
use log::{debug, trace};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::VecDeque;
use std::fmt::Debug;
use thiserror::Error;

pub struct Connector<T>(T, VecDeque<u32>);

impl Connector<WebSocket> {
    pub fn ws(domain: &str, credentials: Option<Credentials>) -> Result<Self, ConnectorError> {
        debug!("Connecting to geth node...");
        Ok(Connector(
            WebSocket::new(domain, credentials).map_err(ConnectorError::Initialization)?,
            (0..1000).collect(),
        ))
    }

    pub fn close(&mut self) -> Result<(), WebSocketError> {
        self.0.close()
    }
}

impl<T: JsonRequest> Connector<T> {
    pub fn call<U: DeserializeOwned + Debug>(
        &mut self,
        mut rpc: Rpc<U>,
    ) -> Result<U, ConnectorError> {
        let command_id = self.get_command_id()?;
        rpc.id = command_id;
        debug!("Calling rpc method: {:?}", &rpc);
        let response = self.send_request(&rpc)?;
        self.1.push_back(command_id);
        deserialize(&response)
    }

    fn get_command_id(&mut self) -> Result<u32, ConnectorError> {
        trace!("Retrieving id from pool...");
        match self.1.pop_front() {
            Some(inner) => Ok(inner),
            None => Err(ConnectorError::NoTicketId),
        }
    }

    fn send_request<U: DeserializeOwned + Debug>(
        &mut self,
        rpc: &Rpc<U>,
    ) -> Result<String, ConnectorError> {
        trace!("Sending request...");
        let response = self.0.json_request(serde_json::to_string(rpc)?)?;

        if !response.contains(&format!("\"id\":{}", rpc.id)) {
            return Err(ConnectorError::WrongId);
        }
        Ok(response)
    }
}

fn deserialize<U: DeserializeOwned + Debug>(response: &str) -> Result<U, ConnectorError> {
    trace!("Deserializing response {}", response);
    match serde_json::from_str::<Response<U>>(response) {
        Ok(Response {
            result_or_error: inner,
            ..
        }) => match inner {
            RpcResult::Result(result) => Ok(result),
            RpcResult::Error(err) => Err(ConnectorError::JsonRpc(err)),
        },
        Err(err) => Err(ConnectorError::from(err)),
    }
}

#[derive(Deserialize, Debug)]
pub struct JsonError {
    code: i32,
    message: String,
}

#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("Connector Error: {0}")]
    Initialization(WebSocketError),
    #[error("Connector Error: Maximum number of connections reached")]
    NoTicketId,
    #[error("Connector Error: {0:?}")]
    JsonRpc(JsonError),
    #[error("Connector Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Connector Error: {0}")]
    TransportError(#[from] TransportError),
    #[error("Connector Error: Received an unexpected message id")]
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
