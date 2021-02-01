use ethane::rpc::{self};
use ethane::types::{BlockHeader, TransactionRequest, U256};

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
    let mut tx_count = 0;
    loop {
        let transaction = TransactionRequest {
            from: create_account(&mut client).1,
            to: Some(create_account(&mut client).1),
            value: Some(U256::zero()),
            ..Default::default()
        };
        let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
        wait_for_transaction(&mut client, tx_hash);
        tx_count = tx_count + 1;
        blocks.push(subscription.next().unwrap());
        if blocks.len() >= 2 {
            break assert!(true);
        }
        if tx_count > 3 {
            break assert!(false);
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
    let mut tx_count = 0;
    loop {
        let transaction = TransactionRequest {
            from: create_account(&mut client).1,
            to: Some(create_account(&mut client).1),
            value: Some(U256::zero()),
            ..Default::default()
        };
        let tx_hash = client.call(rpc::eth_send_transaction(transaction)).unwrap();
        wait_for_transaction(&mut client, tx_hash);
        tx_count = tx_count + 1;
        transactions.push(subscription.next().unwrap());
        if transactions.len() >= 2 {
            break assert!(true);
        }
        if tx_count > 3 {
            break assert!(false);
        }
    }
}
