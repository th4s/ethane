use lazy_static::lazy_static;
use lucita::WebSocket;
use lucita::{Call, Credentials, GethConnector, Rpc};
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
    let geth = Arc::clone(&GETH);
    let mut geth = geth.lock().unwrap();
    let call_result: T = geth.call(rpc).unwrap();
    assert_eq!(call_result, expected);
}

fn rpc_call_test_some<T: DeserializeOwned + Debug + PartialEq>(rpc: Rpc<T>) {
    let geth = Arc::clone(&GETH);
    let mut geth = geth.lock().unwrap();
    let _call_result: T = geth.call(rpc).unwrap();
    assert!(true);
}

#[test]
fn test_geth_net_version() {
    rpc_call_test_some(Rpc::net_version(1));
}

#[test]
fn test_geth_net_peer_count() {
    rpc_call_test_some(Rpc::net_peer_count(2));
}
