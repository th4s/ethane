use lazy_static::lazy_static;
use lucita::rpc::{self, Call, Rpc};
use lucita::WebSocket;
use lucita::{Credentials, GethConnector};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

fn get_node_info() -> (String, Option<Credentials>) {
    dotenv::from_filename("integration-test.env").expect(
        "Integration testing not possible.\
     File 'integration-test.env' is missing",
    );
    let address = dotenv::var("ETH_WS_TEST_SERVER").expect("Var ETH_WS_TEST_SERVER is not set");
    let credentials = if let Some(username) = dotenv::var("USERNAME").ok() {
        Some(Credentials {
            username,
            password: dotenv::var("PASSWORD").expect("Var PASSWORD is not set"),
        })
    } else {
        None
    };
    (address, credentials)
}

lazy_static! {
    static ref GETH: Arc<Mutex<GethConnector<WebSocket>>> = {
        let (address, credentials) = get_node_info();
        Arc::new(Mutex::new(
            GethConnector::ws(&address, credentials).unwrap(),
        ))
    };
}

fn rpc_call_test_expected<T: DeserializeOwned + Debug + PartialEq>(rpc: Rpc<T>, expected: T) {
    let call_result = {
        let geth = Arc::clone(&GETH);
        let mut geth = geth.lock().unwrap();
        geth.call(rpc)
    };
    match call_result {
        Ok(res) => {
            println!("{:?}", res);
            assert_eq!(res, expected);
        }
        Err(err) => assert!(false, format!("{}", err)),
    }
}

fn rpc_call_test_some<T: DeserializeOwned + Debug + PartialEq>(rpc: Rpc<T>) {
    let call_result = {
        let geth = Arc::clone(&GETH);
        let mut geth = geth.lock().unwrap();
        geth.call(rpc)
    };
    match call_result {
        Ok(res) => {
            println!("{:?}", res);
            assert!(true);
        }
        Err(err) => assert!(false, format!("{}", err)),
    }
}

#[test]
fn test_geth_net_version() {
    rpc_call_test_some(rpc::net_version());
}

#[test]
fn test_geth_net_peer_count() {
    rpc_call_test_some(rpc::net_peer_count());
}

#[test]
fn test_geth_net_listening() {
    rpc_call_test_some(rpc::net_listening());
}

#[test]
fn test_geth_eth_protocol_version() {
    rpc_call_test_some(rpc::eth_protocol_version());
}

#[test]
fn test_geth_eth_syncing() {
    rpc_call_test_some(rpc::eth_syncing());
}

// TODO: This behavior is implementation dependent and should be improved
#[test]
#[should_panic]
fn test_geth_eth_coinbase() {
    rpc_call_test_some(rpc::eth_coinbase());
}

#[test]
fn test_geth_eth_mining() {
    rpc_call_test_some(rpc::eth_mining());
}

#[test]
fn test_geth_eth_hashrate() {
    rpc_call_test_some(rpc::eth_hashrate());
}

#[test]
fn test_geth_eth_gas_price() {
    rpc_call_test_some(rpc::eth_gas_price());
}

#[test]
fn test_geth_eth_accounts() {
    rpc_call_test_some(rpc::eth_accounts());
}

#[test]
fn test_geth_eth_block_number() {
    rpc_call_test_some(rpc::eth_block_number());
}
