use crate::rpc::RemoteProcedures;
use crate::transport::ws::WebSocket;
use crate::transport::ws::WebSocketError;
use crate::transport::Request;
use std::error::Error;
use std::marker::PhantomData;

struct GethConnector<T: Request<U>, U: Error>(T, PhantomData<U>);

impl Request<WebSocketError> for GethConnector<WebSocket, WebSocketError> {
    fn request(&mut self, cmd: String) -> Result<String, WebSocketError> {
        self.0.request(cmd)
    }
}

impl RemoteProcedures<WebSocketError> for GethConnector<WebSocket, WebSocketError> {}
