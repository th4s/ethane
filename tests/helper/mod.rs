use ethane::geth::GethConnector;
use ethane::rpc::{self, Call, CallError, Rpc};
use ethane::transport::ws::WebSocket;
use ethane::transport::Request;
use ethane::types::{PrivateKey, TransactionRequest, H160, H256, U256};

use lazy_static::lazy_static;
use rand::Rng;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::path::Path;
use std::process::{Child, Command};
use std::str::FromStr;
use std::sync::Mutex;

pub const TEST_CONTRACT_PATH: &str = "./tests/fixtures/TestContract.sol";
pub const TEST_CONTRACT_NAME: &str = "TestContract";
pub const ACCOUNTS_PASSWORD: &str = "12345678";

lazy_static! {
    static ref PORT_POOL: Mutex<VecDeque<u32>> = Mutex::new((0..100).map(|x| x + 8547).collect());
}

pub struct Client<T: Request> {
    client: GethConnector<T>,
    #[allow(dead_code)]
    process: Process,
}

impl<T: Request> Client<T> {
    pub fn call<U: DeserializeOwned + Debug + PartialEq>(
        &mut self,
        rpc: Rpc<U>,
    ) -> Result<U, CallError> {
        self.client.call(rpc)
    }
}

impl Client<WebSocket> {
    pub fn ws() -> Self {
        let process = Process::new();
        std::thread::sleep(std::time::Duration::from_secs(5));
        let client =
            GethConnector::ws(&format!("ws://127.0.0.1:{}", process.ws_port), None).unwrap();
        Client { client, process }
    }
}

pub struct Process {
    cmd: Child,
    http_port: u32,
    ws_port: u32,
}

impl Process {
    pub fn new() -> Self {
        let (http_port, ws_port) = {
            let mut pool = PORT_POOL.lock().unwrap();
            (
                pool.pop_front().expect("No port left in port pool."),
                pool.pop_front().expect("No port left in port pool."),
            )
        };
        let cmd = Command::new("geth")
            .args(&[
                "--dev",
                "--ws",
                "--ws.api",
                "personal,eth,net,web3",
                "--ws.port",
                &ws_port.to_string(),
                "--http",
                "--http.api",
                "personal,eth,net,web3",
                "--http.port",
                &http_port.to_string(),
                "--allow-insecure-unlock",
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Unable to start local geth node for integration tests. Is geth installed?");
        Process {
            cmd,
            http_port,
            ws_port,
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        let e_message = format!(
            "Unable to tear down eth node. Please kill PID {} manually.",
            self.cmd.id()
        );
        let mut cmd = Command::new("kill");
        if let Ok(mut child) = cmd.arg(self.cmd.id().to_string()).spawn() {
            if !child.wait().expect(&e_message).success() {
                println!("{}", &e_message);
            }
            let mut pool = PORT_POOL.lock().unwrap();
            pool.push_back(self.ws_port);
            pool.push_back(self.http_port);
        } else {
            println!("{}", &e_message);
        }
    }
}

pub fn wait_for_transaction<U: Request>(client: &mut Client<U>, tx_hash: H256) {
    loop {
        let transaction = client
            .call(rpc::eth_get_transaction_by_hash(tx_hash))
            .unwrap();
        if let Some(_) = transaction.block_hash {
            break;
        }
    }
}

pub fn create_secret() -> H256 {
    const HEX_CHARSET: &[u8] = b"abcdef0123456789";
    const PK_LEN: usize = 64;
    let mut rng = rand::thread_rng();

    let secret: String = (0..PK_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..HEX_CHARSET.len());
            HEX_CHARSET[idx] as char
        })
        .collect();
    H256::from_str(&secret).unwrap()
}

pub fn import_account<U: Request>(client: &mut Client<U>, secret: H256) -> H160 {
    client
        .call(rpc::personal_import_raw_key(
            PrivateKey::NonPrefixed(secret),
            String::from(ACCOUNTS_PASSWORD),
        ))
        .unwrap()
}

pub fn unlock_account<U: Request>(client: &mut Client<U>, address: H160) -> bool {
    client
        .call(rpc::personal_unlock_account(
            address,
            String::from(ACCOUNTS_PASSWORD),
            None,
        ))
        .unwrap()
}

pub fn prefund_account<U: Request>(client: &mut Client<U>, address: H160) -> H256 {
    let coinbase = client.call(rpc::eth_coinbase()).unwrap();
    let tx = TransactionRequest {
        from: coinbase,
        to: Some(address),
        value: Some(U256::exp10(20)),
        ..Default::default()
    };
    let tx_hash = client.call(rpc::eth_send_transaction(tx)).unwrap();
    wait_for_transaction(client, tx_hash);
    tx_hash
}

pub fn create_account<U: Request>(client: &mut Client<U>) -> (H256, H160) {
    let secret = create_secret();
    let address = import_account(client, secret);
    unlock_account(client, address);
    prefund_account(client, address);
    (secret, address)
}

pub fn compile_contract(path: &Path, contract_name: &str) -> (String, String) {
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

pub fn rpc_call_test_expected<'a, T: DeserializeOwned + Debug + PartialEq, U: Request>(
    client: &mut Client<U>,
    rpc: Rpc<T>,
    expected: T,
) {
    match client.call(rpc) {
        Ok(res) => {
            println!("{:?}", res);
            assert_eq!(res, expected);
        }
        Err(err) => assert!(false, format!("{}", err)),
    }
}

pub fn rpc_call_test_some<T: DeserializeOwned + Debug + PartialEq, U: Request>(
    client: &mut Client<U>,
    rpc: Rpc<T>,
) {
    match client.call(rpc) {
        Ok(res) => {
            println!("{:?}", res);
            assert!(true);
        }
        Err(err) => assert!(false, format!("{}", err)),
    }
}
