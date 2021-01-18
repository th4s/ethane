use lazy_static::lazy_static;
use lucita::rpc::{self, Call, CallError, Rpc};
use lucita::GethConnector;
use lucita::{BlockParameter, WebSocket};
use serde::de::DeserializeOwned;
use simple_logger::SimpleLogger;
use std::fmt::Debug;
use std::process::Command;
use std::sync::{Arc, Mutex};

const ETH_ENDPOINT: &str = "ws://127.0.0.1:8546";

struct EthProcess(u32);

impl Drop for EthProcess {
    fn drop(&mut self) {
        let e_message = format!(
            "Unable to tear down eth node. Please kill PID {} manually.",
            self.0
        );
        let mut cmd = Command::new("kill");
        if let Ok(mut child) = cmd.arg(self.0.to_string()).spawn() {
            if !child.wait().expect(&e_message).success() {
                println!("{}", &e_message);
            }
        } else {
            println!("{}", &e_message);
        }
    }
}

lazy_static! {
    static ref ETH_CLIENT: Arc<Mutex<GethConnector<WebSocket>>> = {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Off)
            .with_module_level(
                &std::env::var("CARGO_PKG_NAME").expect("Env var CARGO_PKG_NAME not found"),
                log::LevelFilter::Trace,
            )
            .init()
            .expect("Unable to start SimpleLogger");
        let _eth_process = spin_up_geth();
        std::thread::sleep(std::time::Duration::from_secs(3));
        let (address, credentials) = (ETH_ENDPOINT, None);
        Arc::new(Mutex::new(
            GethConnector::ws(&address, credentials).unwrap(),
        ))
    };
}

fn spin_up_geth() -> EthProcess {
    let cmd = Command::new("geth")
        .args(&["--dev", "--ws", "--http"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Unable to start local geth node in dev mode. Is geth installed?");
    EthProcess(cmd.id())
}

fn call<T: DeserializeOwned + Debug>(rpc: Rpc<T>) -> Result<T, CallError> {
    let geth = Arc::clone(&ETH_CLIENT);
    let mut geth = geth.lock().unwrap();
    geth.call(rpc)
}

fn rpc_call_test_expected<T: DeserializeOwned + Debug + PartialEq>(rpc: Rpc<T>, expected: T) {
    let call_result = call(rpc);
    match call_result {
        Ok(res) => {
            println!("{:?}", res);
            assert_eq!(res, expected);
        }
        Err(err) => assert!(false, format!("{}", err)),
    }
}

fn rpc_call_test_some<T: DeserializeOwned + Debug + PartialEq>(rpc: Rpc<T>) {
    let call_result = call(rpc);
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

#[test]
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

#[test]
fn test_geth_eth_get_balance() {
    let coinbase = call(rpc::eth_coinbase()).unwrap();
    rpc_call_test_some(rpc::eth_get_balance(coinbase, BlockParameter::Latest));
}
