use ethereum_types::Address;
use ethereum_types::H256;
use std::fmt::{Display, Formatter, Result as FmtResult};

mod geth;

pub trait RemoteProcedures {
    const ID: &'static str = "_ID_";
    const PARAMS: &'static str = "_PARAMS_";
    const METHOD: &'static str = "_METHOD_";
    const CMD: &'static str =
        r#"{"jsonrpc":"2.0","method":"_METHOD_","params":[_PARAMS_],"id":_ID_}"#;

    fn net_version(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "net_version")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
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

    fn eth_sign() -> &'static str;
    fn eth_signTransaction() -> &'static str;
    fn eth_sendTransaction() -> &'static str;
    fn eth_sendRawTransaction() -> &'static str;
    fn eth_call() -> &'static str;
    fn eth_estimateGas() -> &'static str;
    fn eth_getBlockByHash() -> &'static str;
    fn eth_getBlockByNumber() -> &'static str;
    fn eth_getTransactionByHash() -> &'static str;
    fn eth_getTransactionByBlockHashAndIndex() -> &'static str;
    fn eth_getTransactionByBlockNumberAndIndex() -> &'static str;
    fn eth_getTransactionReceipt() -> &'static str;
    fn eth_getUncleByBlockHashAndIndex() -> &'static str;
    fn eth_getUncleByBlockNumberAndIndex() -> &'static str;
    fn eth_getCompilers() -> &'static str;
    fn eth_compileLLL() -> &'static str;
    fn eth_compileSolidity() -> &'static str;
    fn eth_compileSerpent() -> &'static str;
    fn eth_newFilter() -> &'static str;
    fn eth_newBlockFilter() -> &'static str;
    fn eth_newPendingTransactionFilter() -> &'static str;
    fn eth_uninstallFilter() -> &'static str;
    fn eth_getFilterChanges() -> &'static str;
    fn eth_getFilterLogs() -> &'static str;
    fn eth_getLogs() -> &'static str;
    fn eth_getWork() -> &'static str;
    fn eth_submitWork() -> &'static str;
    fn eth_submitHashrate() -> &'static str;
}

pub enum BlockParameter {
    LATEST,
    EARLIEST,
    PENDING,
    CUSTOM(u32),
}

impl Display for BlockParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let block_param = match *self {
            BlockParameter::LATEST => String::from("latest"),
            BlockParameter::EARLIEST => String::from("earliest"),
            BlockParameter::PENDING => String::from("pending"),
            BlockParameter::CUSTOM(num) => format!("{:#x}", num),
        };
        write!(f, "{}", block_param)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_for_block_parameter() {
        assert_eq!(BlockParameter::CUSTOM(17).to_string(), "0x11");
    }
}
