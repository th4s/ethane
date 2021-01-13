use crate::rpc::{Call, Rpc};
use crate::transport::ws::{WebSocket, WebSocketError};
use crate::transport::Request;
use crate::Credentials;
use serde::de::DeserializeOwned;
use std::error::Error;

pub struct GethConnector<T: Request>(T);

impl GethConnector<WebSocket> {
    pub fn ws(domain: &str, credentials: Option<Credentials>) -> Result<Self, WebSocketError> {
        Ok(GethConnector(WebSocket::new(domain, credentials)?))
    }

    pub fn close(&mut self) -> Result<(), WebSocketError> {
        self.0.close()
    }
}

impl<T: Request> Call for GethConnector<T> {
    fn call<U: DeserializeOwned, V: FnOnce() -> Rpc<U>>(
        &mut self,
        rpc: V,
    ) -> Result<U, Box<dyn Error>> {
        let rpc = rpc();
        let response = self.0.request(rpc.command)?;
        let result = serde_json::from_str::<U>(&response)?;
        Ok(result)
    }
}
