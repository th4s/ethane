use crate::eth_types::*;
use crate::geth::GethError;
use ethereum_types::{Address, H256, U256, U64};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use serde::Deserialize;
use std::fmt::Debug;
use thiserror::Error;

// TODO: eventually remove ethereum_types?
// TODO: eventually use Serde Serialize, or stay with Display?

const CMD: &str = r#"{"jsonrpc":"2.0","method":"_METHOD_","params":[_PARAMS_],"id":_ID_}"#;
const ID: &str = "_ID_";
const PARAMS: &str = "_PARAMS_";
const METHOD: &str = "_METHOD_";

pub trait Call {
    fn call<T: DeserializeOwned + Debug>(&mut self, rpc: Rpc<T>) -> Result<T, CallError>;
}

#[derive(Debug, Error)]
pub enum CallError {
    #[error("{0}")]
    GethError(#[from] GethError),
}

#[derive(Deserialize, Debug)]
pub(crate) struct Response<T> {
    pub id: u32,
    pub jsonrpc: String,
    pub result: Option<T>,
    pub error: Option<JsonError>,
}

#[derive(Deserialize, Debug)]
pub struct JsonError {
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct Rpc<T: DeserializeOwned + Debug> {
    pub command: String,
    pub result: PhantomData<T>,
}

impl<T: DeserializeOwned + Debug> Rpc<T> {
    pub fn id(&mut self, id: u32) {
        self.command = self.command.replace(ID, &id.to_string())
    }
}

pub fn net_version() -> Rpc<String> {
    let command = String::from(CMD)
        .replace(METHOD, "net_version")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_protocol_version() -> Rpc<String> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_protocolVersion")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn net_peer_count() -> Rpc<U64> {
    let command = String::from(CMD)
        .replace(METHOD, "net_peerCount")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn net_listening() -> Rpc<bool> {
    let command = String::from(CMD)
        .replace(METHOD, "net_listening")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_syncing() -> Rpc<bool> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_syncing")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_coinbase() -> Rpc<Address> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_coinbase")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_mining() -> Rpc<bool> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_mining")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_hashrate() -> Rpc<U256> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_hashrate")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_gas_price() -> Rpc<U256> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_gasPrice")
        .replace(PARAMS, "");

    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_accounts() -> Rpc<Vec<Address>> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_accounts")
        .replace(PARAMS, "");

    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_block_number() -> Rpc<U64> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_blockNumber")
        .replace(PARAMS, "");
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_get_balance(address: Address, block_param: Option<BlockParameter>) -> Rpc<U256> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let params: String = vec![
        serde_json::to_string(&address)
            .expect("Serialization of block parameter failed. Should not happen"),
        serde_json::to_string(&block_param)
            .expect("Serialization of address failed. Should not happen"),
    ]
    .join(", ");
    let command = String::from(CMD)
        .replace(METHOD, "eth_getBalance")
        .replace(PARAMS, &params);
    Rpc {
        command,
        result: PhantomData,
    }
}

pub fn eth_get_storage_at(
    address: Address,
    storage_pos: U256,
    block_param: Option<BlockParameter>,
) -> String {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let params: String = vec![
        serde_json::to_string(&address)
            .expect("Serialization of address failed. Should not happen"),
        serde_json::to_string(&storage_pos)
            .expect("Serialization of U256 failed. Should not happen"),
        serde_json::to_string(&block_param)
            .expect("Serialization of block parameter failed. Should not happen"),
    ]
    .join(", ");

    String::from(CMD)
        .replace(METHOD, "eth_getStorageAt")
        .replace(PARAMS, &params)
}

pub fn eth_send_transaction(transaction: Transaction) -> Rpc<H256> {
    let command = String::from(CMD)
        .replace(METHOD, "eth_sendTransaction")
        .replace(
            PARAMS,
            &serde_json::to_string(&transaction)
                .expect("Serialization of transaction failed, Should not happen"),
        );
    Rpc {
        command,
        result: PhantomData,
    }
}

//     fn eth_get_transaction_count(id: u32, address: Address, block_param: BlockParameter) -> String {
//         let params: String = vec![
//             address.to_string(),
//             serde_json::to_string(&block_param).expect("Should not happen"),
//         ]
//         .join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_getTransactionCount")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_get_block_transaction_count_by_hash(id: u32, block_hash: H256) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getBlockTransactionCountByHash")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &block_hash.to_string())
//     }
//
//     fn eth_get_block_transaction_count_by_number(id: u32, block_param: BlockParameter) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getBlockTransactionCountByNumber")
//             .replace(ID, &id.to_string())
//             .replace(
//                 PARAMS,
//                 &serde_json::to_string(&block_param).expect("Should not happen"),
//             )
//     }
//
//     fn eth_get_uncle_count_by_block_hash(id: u32, block_hash: H256) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getUncleCountByBlockHash")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &block_hash.to_string())
//     }
//
//     fn eth_get_uncle_count_by_block_number(id: u32, block_param: BlockParameter) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getUncleCountByBlockNumber")
//             .replace(ID, &id.to_string())
//             .replace(
//                 PARAMS,
//                 &serde_json::to_string(&block_param).expect("Should not happen"),
//             )
//     }
//
//     fn eth_get_code(id: u32, address: Address, block_param: BlockParameter) -> String {
//         let params: String = vec![
//             address.to_string(),
//             serde_json::to_string(&block_param).expect("Should not happen"),
//         ]
//         .join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_getCode")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_sign(id: u32, address: Address, bytes: HexBytes) -> String {
//         let params: String = vec![address.to_string(), bytes.to_string()].join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_sign")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_sign_transaction(id: u32, transaction: Transaction) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_signTransaction")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &transaction.to_string())
//     }
//
//     fn eth_send_transaction(id: u32, transaction: Transaction) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_sendTransaction")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &transaction.to_string())
//     }
//
//     fn eth_send_raw_transaction(id: u32, raw_transaction: HexBytes) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_sendRawTransaction")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &raw_transaction.to_string())
//     }
//
//     fn eth_call(id: u32, transaction: Transaction, block_param: BlockParameter) -> String {
//         let params: String = vec![
//             transaction.to_string(),
//             serde_json::to_string(&block_param).expect("Should not happen"),
//         ]
//         .join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_call")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_estimate_gas(id: u32, transaction: Transaction, block_param: BlockParameter) -> String {
//         let params: String = vec![
//             transaction.to_string(),
//             serde_json::to_string(&block_param).expect("Should not happen"),
//         ]
//         .join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_estimateGas")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_get_block_by_hash(id: u32, block_hash: H256, full_transactions: bool) -> String {
//         let params: String = vec![block_hash.to_string(), full_transactions.to_string()].join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_getBlockByHash")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_get_block_by_number(
//         id: u32,
//         block_param: BlockParameter,
//         full_transactions: bool,
//     ) -> String {
//         let params: String = vec![
//             serde_json::to_string(&block_param).expect("Should not happen"),
//             full_transactions.to_string(),
//         ]
//         .join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_getBlockByNumber")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_get_transaction_by_hash(id: u32, transaction_hash: H256) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getTransactionByHash")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &transaction_hash.to_string())
//     }
//
//     fn eth_get_transaction_by_block_hash_and_index(
//         id: u32,
//         block_hash: H256,
//         index_position: u32,
//     ) -> String {
//         let params: String =
//             vec![block_hash.to_string(), format!("{:#x}", index_position)].join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_getTransactionByBlockHashAndIndex")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_get_transaction_by_block_number_and_index(
//         id: u32,
//         block_param: BlockParameter,
//         index_position: u32,
//     ) -> String {
//         let params: String = vec![
//             serde_json::to_string(&block_param).expect("Should not happen"),
//             format!("{:#x}", index_position),
//         ]
//         .join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_getTransactionByBlockNumberAndIndex")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_get_transaction_receipt(id: u32, transaction_hash: H256) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getTransactionReceipt")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &transaction_hash.to_string())
//     }
//
//     fn eth_get_uncle_by_block_hash_and_index(id: u32, block_hash: H256) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getUncleByBlockHashAndIndex")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &block_hash.to_string())
//     }
//
//     fn eth_get_uncle_by_block_number_and_index(
//         id: u32,
//         block_param: BlockParameter,
//         index_position: u32,
//     ) -> String {
//         let params: String = vec![
//             serde_json::to_string(&block_param).expect("Should not happen"),
//             format!("{:#x}", index_position),
//         ]
//         .join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_getUncleByBlockNumberAndIndex")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_get_compilers(id: u32) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getCompilers")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, "")
//     }
//
//     fn eth_compile_lll(id: u32, source_code: String) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_compileLLL")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &source_code)
//     }
//
//     fn eth_compile_solidity(id: u32, source_code: String) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_compileSolidity")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &source_code)
//     }
//
//     fn eth_compile_serpent(id: u32, source_code: String) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_compileSerpent")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &source_code)
//     }
//
//     fn eth_new_filter(id: u32, filter: Filter) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_newFilter")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &filter.to_string())
//     }
//
//     fn eth_new_block_filter(id: u32) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_newBlockFilter")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, "")
//     }
//
//     fn eth_new_pending_transaction_filter(id: u32) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_newPendingTransactionFilter")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, "")
//     }
//
//     fn eth_uninstall_filter(id: u32, filter_id: u32) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_uninstallFilter")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &format!("{:#x}", filter_id))
//     }
//
//     fn eth_get_filter_changes(id: u32, filter_id: u32) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getFilterChanges")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &format!("{:#x}", filter_id))
//     }
//
//     fn eth_get_filter_logs(id: u32, filter_id: u32) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getFilterLogs")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &format!("{:#x}", filter_id))
//     }
//
//     fn eth_get_logs(id: u32, filter: Filter234) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getLogs")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &filter.to_string())
//     }
//
//     fn eth_get_work(id: u32) -> String {
//         String::from(CMD)
//             .replace(METHOD, "eth_getWork")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, "")
//     }
//
//     fn eth_submit_work(id: u32, nonce: H64, hash: H256, digest: H256) -> String {
//         let params: String =
//             vec![nonce.to_string(), hash.to_string(), digest.to_string()].join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_submitWork")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
//
//     fn eth_submit_hashrate(id: u32, hash_rate: H256, client_id: H256) -> String {
//         let params: String = vec![hash_rate.to_string(), client_id.to_string()].join(", ");
//
//         String::from(CMD)
//             .replace(METHOD, "eth_submitHashrate")
//             .replace(ID, &id.to_string())
//             .replace(PARAMS, &params)
//     }
// }
