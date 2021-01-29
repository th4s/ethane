use super::Rpc;
use crate::types::{Block, H256, U128};
use std::marker::PhantomData;

pub struct SubRequest<T> {
    pub rpc: Rpc<U128>,
    pub result_type: PhantomData<T>,
}

pub fn eth_subscribe_new_heads() -> SubRequest<Block> {
    let mut rpc = Rpc::new("eth_subscribe");
    rpc.add_param("newHeads");
    SubRequest {
        rpc,
        result_type: PhantomData,
    }
}

pub fn eth_subscribe_new_pending_transactions() -> SubRequest<H256> {
    let mut rpc = Rpc::new("eth_subscribe");
    rpc.add_param("newPendingTransactions");
    SubRequest {
        rpc,
        result_type: PhantomData,
    }
}
