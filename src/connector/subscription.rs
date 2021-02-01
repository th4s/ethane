use super::{Connector, ConnectorError};
use crate::rpc::eth_unsubscribe;
use crate::transport::websocket::{WebSocket, WebSocketError};
use crate::types::U128;
use log::{error, trace};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;
use std::marker::PhantomData;
use thiserror::Error;

pub struct Subscription<T: DeserializeOwned + Debug> {
    pub id: U128,
    pub(crate) connector: Connector<WebSocket>,
    pub(crate) result_type: PhantomData<T>,
}

impl<T: DeserializeOwned + Debug> Subscription<T> {
    pub fn next(&mut self) -> Result<T, SubscriptionError> {
        trace!("Fetching next object from subscription");
        let response = self.connector.connection.read_message()?;
        deserialize_from_sub(&response)
    }

    pub fn cancel(self) {}
}

impl<T: DeserializeOwned + Debug> Drop for Subscription<T> {
    fn drop(&mut self) {
        match self.connector.call(eth_unsubscribe(self.id)) {
            Ok(true) => (),
            Ok(_) => error!("Unable to cancel subscription"),
            Err(err) => error!("{}", err),
        }
        match self.connector.close() {
            Ok(()) => (),
            Err(err) => error!("{}", err),
        }
    }
}

fn deserialize_from_sub<U: DeserializeOwned + Debug>(
    response: &str,
) -> Result<U, SubscriptionError> {
    trace!("Deserializing response {}", response);
    let value = serde_json::from_str::<Value>(response)?;
    serde_json::from_value::<U>(value["params"]["result"].clone()).map_err(SubscriptionError::from)
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Subscription Error {0}")]
    Read(#[from] WebSocketError),
    #[error("Subscription Error: Error during canceling subscription: {0}")]
    Cancel(#[from] ConnectorError),
    #[error("Subscription Error: {0}")]
    Serde(#[from] serde_json::Error),
}
