use super::Rpc;
use crate::types::{
    Block, BlockParameter, Bytes, Transaction, TransactionReceipt, TransactionRequest,
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
