//! Allows connecting to an ethereum node

use crate::rpc::{sub::SubscriptionRequest, Rpc};
use crate::transport::{Credentials, Request, TransportError};
use crate::transport::{Http, HttpError, WebSocket, WebSocketError};
use log::{debug, error, info, trace};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;
use thiserror::Error;

mod subscription;
pub use subscription::{Subscription, SubscriptionError};

/// Used to to interact with ethereum nodes
///
/// This is basically a connection wrapper for sending requests or starting subscriptions.
pub struct Connector<T> {
    connection: T,
    id_pool: VecDeque<usize>,
}

impl Connector<Http> {
    /// Create a connector with a http connection. Does **not** allow to subscribe to node events.
    /// If your node supports http basic authentication you can use the credentials.
    pub fn http(domain: &str, credentials: &Option<Credentials>) -> Result<Self, ConnectorError> {
        info!("Creating connector over http");
        Ok(Connector {
            connection: Http::new(String::from(domain), credentials.clone())
                .map_err(ConnectorError::from)?,
            id_pool: (0..1000).collect(),
        })
    }
}

impl Connector<WebSocket> {
    /// Create a connector with a websocket connection.
    /// If your node supports http basic authentication you can use the credentials.
    pub fn websocket(
        domain: &str,
        credentials: &Option<Credentials>,
    ) -> Result<Self, ConnectorError> {
        info!("Creating connector over websocket");
        Ok(Connector {
            connection: WebSocket::new(String::from(domain), credentials.clone())?,
            id_pool: (0..1000).collect(),
        })
    }

    /// Starts a new subscription.
    /// Use one of these rpc generating [functions](crate::rpc::sub) to provide the subscription request.
    /// Returns a [subscription](Subscription) which you can poll for new items.
    pub fn subscribe<U: DeserializeOwned + Debug>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Subscription<U>, ConnectorError> {
        info!("Starting a new subscription");
        let mut connector =
            Connector::websocket(&self.connection.address, &self.connection.credentials)?;
        let subscription_id = connector.call(sub_request.rpc)?;
        Ok(Subscription {
            id: subscription_id,
            connector,
            result_type: PhantomData,
        })
    }
}

impl<T: Request> Connector<T> {
    /// Sends a request to an ethereum node. Use a function in one of these namespaces
    /// [functions](crate::rpc) to generate the request. Does **not** support the
    /// subscription(crate::rpc::sub) namespace.
    pub fn call<U: DeserializeOwned + Debug>(
        &mut self,
        mut rpc: Rpc<U>,
    ) -> Result<U, ConnectorError> {
        let command_id = self.get_command_id()?;
        rpc.id = command_id;
        debug!("Calling rpc method: {:?}", &rpc);
        let response = self.connection.request(serde_json::to_string(&rpc)?)?;
        self.id_pool.push_back(command_id);
        deserialize_from_rpc(&response)
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

fn deserialize_from_rpc<U: DeserializeOwned + Debug>(response: &str) -> Result<U, ConnectorError> {
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

/// Used to deserialize errors returned from the ethereum node
#[derive(Deserialize, Debug, Error)]
#[error("{message}")]
pub struct JsonError {
    code: i32,
    message: String,
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

/// An error type collecting what can go wrong using a connector
#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("Connector Error: {0}")]
    WsInit(#[from] WebSocketError),
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
}
