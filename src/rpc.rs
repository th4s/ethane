use log::error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;
use std::marker::PhantomData;

pub use eth::*;
pub use net::*;
pub use personal::*;
pub use txpool::*;
pub use web3::*;

mod eth;
mod net;
mod personal;
mod txpool;
mod web3;

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

    pub fn new(method: &str) -> Rpc<T> {
        Rpc {
            json_rpc: Self::JSON_RPC,
            method: String::from(method),
            params: Vec::new(),
            id: Default::default(),
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
