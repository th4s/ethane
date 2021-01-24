use super::Rpc;
use crate::types::{
    Block, BlockParameter, Bytes, Call, GasCall, Transaction, TransactionReceipt,
    TransactionRequest,
};
use ethereum_types::{H160, H256, U256, U64};

pub fn eth_protocol_version() -> Rpc<String> {
    Rpc::new("eth_protocolVersion")
}

pub fn eth_syncing() -> Rpc<bool> {
    Rpc::new("eth_syncing")
}

pub fn eth_coinbase() -> Rpc<H160> {
    Rpc::new("eth_coinbase")
}

pub fn eth_mining() -> Rpc<bool> {
    Rpc::new("eth_mining")
}

pub fn eth_hashrate() -> Rpc<U256> {
    Rpc::new("eth_hashrate")
}

pub fn eth_gas_price() -> Rpc<U256> {
    Rpc::new("eth_gasPrice")
}

pub fn eth_accounts() -> Rpc<Vec<H160>> {
    Rpc::new("eth_accounts")
}

pub fn eth_block_number() -> Rpc<U64> {
    Rpc::new("eth_blockNumber")
}

pub fn eth_get_balance(address: H160, block_param: Option<BlockParameter>) -> Rpc<U256> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getBalance");
    rpc.add_param(address);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_send_transaction(transaction: TransactionRequest) -> Rpc<H256> {
    let mut rpc = Rpc::new("eth_sendTransaction");
    rpc.add_param(transaction);
    rpc
}

pub fn eth_get_transaction_by_hash(transaction_hash: H256) -> Rpc<Transaction> {
    let mut rpc = Rpc::new("eth_getTransactionByHash");
    rpc.add_param(transaction_hash);
    rpc
}

pub fn eth_get_transaction_receipt(transaction_hash: H256) -> Rpc<Option<TransactionReceipt>> {
    let mut rpc = Rpc::new("eth_getTransactionReceipt");
    rpc.add_param(transaction_hash);
    rpc
}

pub fn eth_get_storage_at(
    address: H160,
    storage_pos: U256,
    block_param: Option<BlockParameter>,
) -> Rpc<Bytes> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getStorageAt");
    rpc.add_param(address);
    rpc.add_param(storage_pos);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_transaction_count(address: H160, block_param: Option<BlockParameter>) -> Rpc<U256> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getTransactionCount");
    rpc.add_param(address);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_block_by_number(
    block_param: Option<BlockParameter>,
    full_transactions: bool,
) -> Rpc<Option<Block>> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getBlockByNumber");
    rpc.add_param(block_param);
    rpc.add_param(full_transactions);
    rpc
}

pub fn eth_get_block_transaction_count_by_hash(block_hash: H256) -> Rpc<U64> {
    let mut rpc = Rpc::new("eth_getBlockTransactionCountByHash");
    rpc.add_param(block_hash);
    rpc
}

pub fn eth_get_block_transaction_count_by_number(block_param: Option<BlockParameter>) -> Rpc<U64> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getBlockTransactionCountByNumber");
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_uncle_count_by_block_hash(block_hash: H256) -> Rpc<U64> {
    let mut rpc = Rpc::new("eth_getUncleCountByBlockHash");
    rpc.add_param(block_hash);
    rpc
}

pub fn eth_get_uncle_count_by_block_number(block_param: Option<BlockParameter>) -> Rpc<U64> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getUncleCountByBlockNumber");
    rpc.add_param(block_param);
    rpc
}

pub fn eth_get_code(address: H160, block_param: Option<BlockParameter>) -> Rpc<Bytes> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_getCode");
    rpc.add_param(address);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_sign(address: H160, data: Bytes) -> Rpc<Bytes> {
    let mut rpc = Rpc::new("eth_sign");
    rpc.add_param(address);
    rpc.add_param(data);
    rpc
}

pub fn eth_sign_transaction(transaction: TransactionRequest) -> Rpc<Bytes> {
    let mut rpc = Rpc::new("eth_signTransaction");
    rpc.add_param(transaction);
    rpc
}

pub fn eth_send_raw_transaction(raw_transaction: Bytes) -> Rpc<H256> {
    let mut rpc = Rpc::new("eth_sendRawTransaction");
    rpc.add_param(raw_transaction);
    rpc
}

pub fn eth_call(call: Call, block_param: Option<BlockParameter>) -> Rpc<Bytes> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_call");
    rpc.add_param(call);
    rpc.add_param(block_param);
    rpc
}

pub fn eth_estimate_gas(gas_call: GasCall, block_param: Option<BlockParameter>) -> Rpc<U256> {
    let block_param = block_param.unwrap_or(BlockParameter::Latest);
    let mut rpc = Rpc::new("eth_estimateGas");
    rpc.add_param(gas_call);
    rpc.add_param(block_param);
    rpc
}

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
