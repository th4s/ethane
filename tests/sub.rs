use ethane::rpc::{self};
use ethane::types::{
    BlockHeader, Bytes, FilterSubscription, Log, TransactionRequest, ValueOrVec, U256,
};
use std::path::Path;

pub mod helper;
use helper::*;

pub mod fixtures;
use ethereum_types::H256;
use fixtures::*;

#[test]
fn eth_subscribe_new_heads() {
    let mut client = ClientWrapper::Websocket(Client::ws());
    let mut subscription = client.subscribe(rpc::eth_subscribe_new_heads()).unwrap();
    let mut blocks = Vec::<BlockHeader>::new();
    loop {
        let transaction = TransactionRequest {
            from: create_account(&mut client).1,
            to: Some(create_account(&mut client).1),
            value: Some(U256::zero()),
            ..Default::default()
        };
        let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
        wait_for_transaction(&mut client, tx_hash);
        blocks.push(subscription.next_item().unwrap());
        if blocks.len() >= 2 {
            break assert!(true);
        }
    }
}

#[test]
fn eth_subscribe_new_pending_transactions() {
    let mut client = ClientWrapper::Websocket(Client::ws());
    let mut subscription = client
        .subscribe(rpc::eth_subscribe_new_pending_transactions())
        .unwrap();
    let mut transactions = Vec::<H256>::new();
    loop {
        let transaction = TransactionRequest {
            from: create_account(&mut client).1,
            to: Some(create_account(&mut client).1),
            value: Some(U256::zero()),
            ..Default::default()
        };
        let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
        wait_for_transaction(&mut client, tx_hash);
        transactions.push(subscription.next_item().unwrap());
        if transactions.len() >= 2 {
            break assert!(true);
        }
    }
}

#[test]
fn eth_subscribe_logs() {
    let mut client = ClientWrapper::Websocket(Client::ws());
    let address = create_account(&mut client).1;
    let (contract_address, _) = deploy_contract(
        &mut client,
        address,
        &Path::new(TEST_CONTRACT_PATH),
        TEST_CONTRACT_NAME,
    );

    let topic = keccak(b"Solution(uint256)");
    let filter = FilterSubscription {
        address: Some(ValueOrVec::Value(contract_address)),
        topics: Some(vec![Some(ValueOrVec::Value(H256::from_slice(&topic)))]),
    };
    let mut logs = Vec::<Log>::new();
    let mut subscription = client.subscribe(rpc::eth_subscribe_logs(filter)).unwrap();
    let out = keccak(b"set_pos0()");

    loop {
        let tx = TransactionRequest {
            from: create_account(&mut client).1,
            to: Some(contract_address),
            data: Some(Bytes::from_slice(&out[..4])),
            ..Default::default()
        };
        let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
        wait_for_transaction(&mut client, tx_hash);
        logs.push(subscription.next_item().unwrap());

        if logs.len() >= 2 {
            break assert!(true);
        }
    }
}
