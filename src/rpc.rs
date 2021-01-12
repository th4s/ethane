// TODO: eventually remove ethereum_types?
// TODO: eventually use Serde Serialize, or stay with Display?
use crate::eth_types::*;
use crate::transport::Request;
use ethereum_types::{Address, H256, H64};
use std::error::Error;

mod geth;

pub trait RemoteProcedures<T: Error>: Request<T> {
    const ID: &'static str = "_ID_";
    const PARAMS: &'static str = "_PARAMS_";
    const METHOD: &'static str = "_METHOD_";
    const CMD: &'static str =
        r#"{"jsonrpc":"2.0","method":"_METHOD_","params":[_PARAMS_],"id":_ID_}"#;

    fn net_version(&mut self, id: u32) -> Result<String, T> {
        // serialize, send, wait, deserialize
        let cmd = String::from(Self::CMD)
            .replace(Self::METHOD, "net_version")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "");
        self.request(cmd)
    }

    fn net_peer_count(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "net_peerCount")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn net_listening(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "net_listening")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_protocol_version(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_protocol_version")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_syncing(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_syncing")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_coinbase(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_coinbase")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_mining(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_mining")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_hashrate(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_hashrate")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_gas_price(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_gasPrice")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_accounts(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_accounts")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_block_number(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_blockNumber")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_get_balance(id: u32, address: Address, block_param: BlockParameter) -> String {
        let params: String = vec![address.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBalance")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_storage_at(
        id: u32,
        address: Address,
        storage_pos: u32,
        block_param: BlockParameter,
    ) -> String {
        let params: String = vec![
            address.to_string(),
            format!("{:#x}", storage_pos),
            block_param.to_string(),
        ]
        .join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getStorageAt")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }
    fn eth_get_transaction_count(id: u32, address: Address, block_param: BlockParameter) -> String {
        let params: String = vec![address.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionCount")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_block_transaction_count_by_hash(id: u32, block_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBlockTransactionCountByHash")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_hash.to_string())
    }

    fn eth_get_block_transaction_count_by_number(id: u32, block_param: BlockParameter) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBlockTransactionCountByNumber")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_param.to_string())
    }

    fn eth_get_uncle_count_by_block_hash(id: u32, block_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getUncleCountByBlockHash")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_hash.to_string())
    }

    fn eth_get_uncle_count_by_block_number(id: u32, block_param: BlockParameter) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getUncleCountByBlockNumber")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_param.to_string())
    }

    fn eth_get_code(id: u32, address: Address, block_param: BlockParameter) -> String {
        let params: String = vec![address.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getCode")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_sign(id: u32, address: Address, bytes: Bytes) -> String {
        let params: String = vec![address.to_string(), bytes.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_sign")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_sign_transaction(id: u32, transaction: Transaction) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_signTransaction")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &transaction.to_string())
    }

    fn eth_send_transaction(id: u32, transaction: Transaction) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_sendTransaction")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &transaction.to_string())
    }

    fn eth_send_raw_transaction(id: u32, raw_transaction: Bytes) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_sendRawTransaction")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &raw_transaction.to_string())
    }

    fn eth_call(id: u32, transaction: Transaction, block_param: BlockParameter) -> String {
        let params: String = vec![transaction.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_call")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_estimate_gas(id: u32, transaction: Transaction, block_param: BlockParameter) -> String {
        let params: String = vec![transaction.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_estimateGas")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_block_by_hash(id: u32, block_hash: H256, full_transactions: bool) -> String {
        let params: String = vec![block_hash.to_string(), full_transactions.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBlockByHash")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_block_by_number(
        id: u32,
        block_param: BlockParameter,
        full_transactions: bool,
    ) -> String {
        let params: String =
            vec![block_param.to_string(), full_transactions.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBlockByNumber")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_transaction_by_hash(id: u32, transaction_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionByHash")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &transaction_hash.to_string())
    }

    fn eth_get_transaction_by_block_hash_and_index(
        id: u32,
        block_hash: H256,
        index_position: u32,
    ) -> String {
        let params: String =
            vec![block_hash.to_string(), format!("{:#x}", index_position)].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionByBlockHashAndIndex")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_transaction_by_block_number_and_index(
        id: u32,
        block_param: BlockParameter,
        index_position: u32,
    ) -> String {
        let params: String =
            vec![block_param.to_string(), format!("{:#x}", index_position)].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionByBlockNumberAndIndex")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_transaction_receipt(id: u32, transaction_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionReceipt")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &transaction_hash.to_string())
    }

    fn eth_get_uncle_by_block_hash_and_index(id: u32, block_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getUncleByBlockHashAndIndex")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_hash.to_string())
    }

    fn eth_get_uncle_by_block_number_and_index(
        id: u32,
        block_param: BlockParameter,
        index_position: u32,
    ) -> String {
        let params: String =
            vec![block_param.to_string(), format!("{:#x}", index_position)].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getUncleByBlockNumberAndIndex")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_compilers(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getCompilers")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_compile_lll(id: u32, source_code: String) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_compileLLL")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &source_code)
    }

    fn eth_compile_solidity(id: u32, source_code: String) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_compileSolidity")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &source_code)
    }

    fn eth_compile_serpent(id: u32, source_code: String) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_compileSerpent")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &source_code)
    }

    fn eth_new_filter(id: u32, filter: Filter) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_newFilter")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &filter.to_string())
    }

    fn eth_new_block_filter(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_newBlockFilter")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_new_pending_transaction_filter(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_newPendingTransactionFilter")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_uninstall_filter(id: u32, filter_id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_uninstallFilter")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &format!("{:#x}", filter_id))
    }

    fn eth_get_filter_changes(id: u32, filter_id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getFilterChanges")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &format!("{:#x}", filter_id))
    }

    fn eth_get_filter_logs(id: u32, filter_id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getFilterLogs")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &format!("{:#x}", filter_id))
    }

    fn eth_get_logs(id: u32, filter: Filter234) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getLogs")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &filter.to_string())
    }

    fn eth_get_work(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getWork")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_submit_work(id: u32, nonce: H64, hash: H256, digest: H256) -> String {
        let params: String =
            vec![nonce.to_string(), hash.to_string(), digest.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_submitWork")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_submit_hashrate(id: u32, hash_rate: H256, client_id: H256) -> String {
        let params: String = vec![hash_rate.to_string(), client_id.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_submitHashrate")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }
}
