use ethane::rpc;
use ethane::types::{BlockParameter, Bytes, TransactionRequest, U256, U64};
use std::path::Path;
use std::str::FromStr;

pub mod helper;
use helper::*;

#[test]
fn test_eth_protocol_version() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_protocol_version());
}

#[test]
fn test_eth_syncing() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_syncing());
}

#[test]
fn test_eth_coinbase() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_coinbase());
}

#[test]
fn test_eth_mining() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_mining());
}

#[test]
fn test_eth_hashrate() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_hashrate());
}

#[test]
fn test_eth_gas_price() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_gas_price());
}

#[test]
fn test_eth_accounts() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_accounts());
}

#[test]
fn test_eth_block_number() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::eth_block_number());
}

#[test]
fn test_eth_get_balance() {
    let mut client = Client::ws();
    let (_secret, address) = create_account(&mut client);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_balance(address, None),
        U256::exp10(20),
    );
}

#[test]
fn test_eth_send_transaction_contract_creation() {
    let mut client = Client::ws();
    let bin = bin(compile_contract(
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    ));
    let contract_bytes = Bytes::from_str(&bin).unwrap();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        data: Some(contract_bytes),
        ..Default::default()
    };
    rpc_call_test_some(&mut client, rpc::eth_send_transaction(transaction));
}

#[test]
fn test_eth_get_transaction_by_hash() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_transaction_by_hash(transaction_hash),
    );
}

#[test]
fn test_eth_get_transaction_receipt() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_transaction_receipt(transaction_hash),
    );
}

#[test]
fn test_eth_get_storage_at() {
    let mut client = Client::ws();
    let bin = bin(compile_contract(
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    ));
    let contract_bytes = Bytes::from_str(&bin).unwrap();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        data: Some(contract_bytes),
        ..Default::default()
    };
    let transaction_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, transaction_hash);
    let receipt = client
        .call(rpc::eth_get_transaction_receipt(transaction_hash))
        .unwrap();
    let contract_address = receipt.unwrap().contract_address.unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_storage_at(contract_address, U256::zero(), None),
        Bytes({
            let mut inner = vec![0; 32];
            inner[31] = 11;
            inner
        }),
    );
}

#[test]
fn test_eth_get_transaction_count() {
    let mut client = Client::ws();
    let sender = create_account(&mut client).1;
    let transaction = TransactionRequest {
        from: sender,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let transaction_hash_1 = client
        .call(rpc::eth_send_transaction(transaction.clone()))
        .unwrap();
    let transaction_hash_2 = client
        .call(rpc::eth_send_transaction(transaction.clone()))
        .unwrap();
    let transaction_hash_3 = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, transaction_hash_1);
    wait_for_transaction(&mut client, transaction_hash_2);
    wait_for_transaction(&mut client, transaction_hash_3);
    rpc_call_test_expected(
        &mut client,
        rpc::eth_get_transaction_count(sender, None),
        U256::from(3),
    );
}

#[test]
fn test_eth_get_block_by_number_full_tx() {
    let mut client = Client::ws();
    let sender = create_account(&mut client).1;
    let transaction = TransactionRequest {
        from: sender,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_block_by_number(None, true));
}

#[test]
fn test_eth_get_block_by_number_only_hashes() {
    let mut client = Client::ws();
    let sender = create_account(&mut client).1;
    let transaction = TransactionRequest {
        from: sender,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(&mut client, rpc::eth_get_block_by_number(None, false));
}

#[test]
fn test_eth_get_block_by_number_no_block() {
    let mut client = Client::ws();
    let sender = create_account(&mut client).1;
    let transaction = TransactionRequest {
        from: sender,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_block_by_number(Some(BlockParameter::Custom(U64::from(120))), false),
    );
}

#[test]
fn test_eth_get_block_transaction_count_by_hash() {
    let mut client = Client::ws();
    let transaction = TransactionRequest {
        from: create_account(&mut client).1,
        to: Some(create_account(&mut client).1),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(&mut client, tx_hash);
    let block = client
        .call(rpc::eth_get_block_by_number(None, false))
        .unwrap();
    rpc_call_test_some(
        &mut client,
        rpc::eth_get_block_transaction_count_by_hash(block.unwrap().hash.unwrap()),
    );
}
