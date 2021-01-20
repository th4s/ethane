use ethane::rpc::{self, Call, CallError, Rpc};
use ethane::{Bytes, GethConnector, TransactionRequest, WebSocket};
use ethereum_types::{Address, H256, U256};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde_json::Value;
use simple_logger::SimpleLogger;
use std::fmt::Debug;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;

const ETH_ENDPOINT: &str = "ws://127.0.0.1:8546";
const TEST_CONTRACT_PATH: &str = "./tests/TestContract.sol";
const TEST_CONTRACT_NAME: &str = "TestContract";

struct GethProcess(u32);

impl Drop for GethProcess {
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
    static ref ETH_CLIENT: Mutex<GethConnector<WebSocket>> = {
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
        Mutex::new(GethConnector::ws(&address, credentials).unwrap())
    };
}

fn spin_up_geth() -> GethProcess {
    let cmd = Command::new("geth")
        .args(&["--dev", "--ws", "--http"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Unable to start local geth node for integration tests. Is geth installed?");
    GethProcess(cmd.id())
}

fn compile_contract(path: &Path, contract_name: &str) -> (String, String) {
    let path_as_str = path.to_str().unwrap();
    let output = Command::new("solc")
        .args(&[path_as_str, "--optimize", "--combined-json", "abi,bin"])
        .output()
        .expect("Failed to compile contract. Is Solidity compiler solc installed?")
        .stdout;
    let output = std::str::from_utf8(output.as_slice()).expect("Failed to parse compiled contract");
    let output: Value =
        serde_json::from_str(output).expect("Failed to deserialize compiled contract");
    let output = output["contracts"][String::from(path_as_str) + ":" + contract_name].clone();
    (output["abi"].to_string(), output["bin"].to_string())
}

fn call<T: DeserializeOwned + Debug>(rpc: Rpc<T>) -> Result<T, CallError> {
    let mut geth = ETH_CLIENT.lock().unwrap();
    geth.call(rpc)
}

fn wait_for_transaction(tx_hash: H256) {
    loop {
        let transaction = call(rpc::eth_get_transaction_by_hash(tx_hash)).unwrap();
        if let Some(_) = transaction.block_hash {
            break;
        }
    }
}

fn rpc_call_test_expected<'a, T: DeserializeOwned + Debug + PartialEq>(rpc: Rpc<T>, expected: T) {
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
    rpc_call_test_some(rpc::eth_get_balance(coinbase, None));
}

#[test]
fn test_geth_eth_send_transaction_value() {
    let transaction = TransactionRequest {
        from: call(rpc::eth_coinbase()).unwrap(),
        to: Some(Address::zero()),
        value: Some(U256::from(1337)),
        ..Default::default()
    };
    let transaction_hash = call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(transaction_hash);
    rpc_call_test_expected(
        rpc::eth_get_balance(Address::zero(), None),
        U256::from(1337),
    );
}

#[test]
fn test_geth_eth_send_transaction_contract_creation() {
    let (_abi, bin) = compile_contract(&Path::new(TEST_CONTRACT_PATH), TEST_CONTRACT_NAME);
    let contract_bytes = serde_json::from_str::<Bytes>(&bin).unwrap();
    let transaction = TransactionRequest {
        from: call(rpc::eth_coinbase()).unwrap(),
        data: Some(contract_bytes),
        ..Default::default()
    };
    rpc_call_test_some(rpc::eth_send_transaction(transaction));
}

#[test]
fn test_geth_eth_get_transaction_by_hash() {
    let transaction = TransactionRequest {
        from: call(rpc::eth_coinbase()).unwrap(),
        to: Some(Address::zero()),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let transaction_hash = call(rpc::eth_send_transaction(transaction)).unwrap();
    rpc_call_test_some(rpc::eth_get_transaction_by_hash(transaction_hash))
}

#[test]
fn test_geth_eth_get_transaction_receipt() {
    let transaction = TransactionRequest {
        from: call(rpc::eth_coinbase()).unwrap(),
        to: Some(Address::zero()),
        value: Some(U256::zero()),
        ..Default::default()
    };
    let transaction_hash = call(rpc::eth_send_transaction(transaction)).unwrap();
    rpc_call_test_some(rpc::eth_get_transaction_receipt(transaction_hash));
}

#[test]
fn test_geth_eth_get_storage_at() {
    let (_abi, bin) = compile_contract(&Path::new(TEST_CONTRACT_PATH), TEST_CONTRACT_NAME);
    let contract_bytes = serde_json::from_str::<Bytes>(&bin).unwrap();
    let transaction = TransactionRequest {
        from: call(rpc::eth_coinbase()).unwrap(),
        data: Some(contract_bytes),
        ..Default::default()
    };
    let transaction_hash = call(rpc::eth_send_transaction(transaction)).unwrap();
    wait_for_transaction(transaction_hash);
    let receipt = call(rpc::eth_get_transaction_receipt(transaction_hash)).unwrap();
    let contract_address = receipt.unwrap().contract_address.unwrap();
    rpc_call_test_expected(
        rpc::eth_get_storage_at(contract_address, U256::zero(), None),
        Bytes({
            let mut inner = vec![0; 32];
            inner[31] = 11;
            inner
        }),
    )
}
