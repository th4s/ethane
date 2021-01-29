use crate::rpc::Rpc;
use crate::transport::http::{Http, HttpError};
use crate::transport::websocket::{WebSocket, WebSocketError};
use crate::transport::{Credentials, Request, TransportError};
use log::{debug, info, trace};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::VecDeque;
use std::fmt::Debug;
use thiserror::Error;

pub struct Connector<T> {
    connection: T,
    id_pool: VecDeque<usize>,
}

impl Connector<WebSocket> {
    pub fn websocket(
        domain: &str,
        credentials: Option<Credentials>,
    ) -> Result<Self, ConnectorError> {
        info!("Creating connector over websocket...");
        Ok(Connector {
            connection: WebSocket::new(domain, credentials).map_err(ConnectorError::WsInit)?,
            id_pool: (0..1000).collect(),
        })
    }

    pub fn close(&mut self) -> Result<(), ConnectorError> {
        self.connection.close().map_err(ConnectorError::WsClose)
    }
}

impl Connector<Http> {
    pub fn http(domain: &str, credentials: Option<Credentials>) -> Result<Self, ConnectorError> {
        info!("Creating connector over http...");
        Ok(Connector {
            connection: Http::new(domain, credentials).map_err(ConnectorError::from)?,
            id_pool: (0..1000).collect(),
        })
    }
}

impl<T: Request> Connector<T> {
    pub fn call<U: DeserializeOwned + Debug>(
        &mut self,
        mut rpc: Rpc<U>,
    ) -> Result<U, ConnectorError> {
        let command_id = self.get_command_id()?;
        rpc.id = command_id;
        debug!("Calling rpc method: {:?}", &rpc);
        let response = self.connection.request(serde_json::to_string(&rpc)?)?;
        self.id_pool.push_back(command_id);
        deserialize(&response)
    }
}

impl<T> Connector<T> {
    fn get_command_id(&mut self) -> Result<usize, ConnectorError> {
        match self.id_pool.pop_front() {
            Some(inner) => {
                trace!("Using id {} for request", inner);
                Ok(inner)
            }
            None => Err(ConnectorError::NoTicketId),
        }
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
            RpcResult::Error(err) => Err(ConnectorError::from(err)),
        },
        Err(err) => Err(ConnectorError::from(err)),
    }
}

#[derive(Deserialize, Debug, Error)]
#[error("{message}")]
pub struct JsonError {
    code: i32,
    message: String,
}

#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("Connector Error: {0}")]
    WsInit(WebSocketError),
    #[error("Connector Error: {0}")]
    WsClose(WebSocketError),
    #[error("Connector Error: {0}")]
    HttpInit(#[from] HttpError),
    #[error("Connector Error: Maximum number of connections reached")]
    NoTicketId,
    #[error("Connector Error: {0}")]
    Transport(#[from] TransportError),
    #[error("Connector Error: {0:?}")]
    JsonRpc(#[from] JsonError),
    #[error("Connector Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Connector Error: Received an unexpected message id")]
    WrongId,
}

#[derive(Deserialize, Debug)]
struct Response<T> {
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
