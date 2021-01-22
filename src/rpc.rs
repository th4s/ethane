use crate::geth::GethError;
use log::error;
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use thiserror::Error;

pub use eth::*;
pub use net::*;
pub use personal::*;

mod eth;
mod net;
mod personal;

pub trait Call {
    fn call<T: DeserializeOwned + Debug>(&mut self, rpc: Rpc<T>) -> Result<T, CallError>;
}

#[derive(Debug, Error)]
pub enum CallError {
    #[error("{0}")]
    GethError(#[from] GethError),
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub id: u32,
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    #[serde(flatten)]
    pub result_or_error: RpcResult<T>,
}

#[derive(Deserialize, Debug)]
pub enum RpcResult<T> {
    #[serde(rename = "result")]
    Result(T),
    #[serde(rename = "error")]
    Error(JsonError),
}

#[derive(Deserialize, Debug)]
pub struct JsonError {
    code: i32,
    message: String,
}

#[derive(Serialize, Debug)]
pub struct Rpc<T: DeserializeOwned + Debug> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: &'static str,
    pub method: String,
    pub params: Vec<Value>,
    pub id: u32,
    #[serde(skip_serializing)]
    result: PhantomData<T>,
}

impl<T: DeserializeOwned + Debug> Rpc<T> {
    const JSON_RPC: &'static str = "2.0";
    const ID: u32 = 0;

    pub fn new(method: &str) -> Rpc<T> {
        Rpc {
            json_rpc: Self::JSON_RPC,
            method: String::from(method),
            params: Vec::new(),
            id: Self::ID,
            result: PhantomData,
        }
    }

    pub fn add_param<U: Serialize + Debug>(&mut self, parameter: U) {
        match serde_json::to_value(&parameter) {
            Ok(serialized_param) => self.params.push(serialized_param),
            Err(err) => error!("Error during serialization: {}", err),
        }
    }
}
