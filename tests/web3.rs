use ethane::rpc;
use ethane::types::{Bytes, H256};
use std::str::FromStr;

pub mod helper;
use helper::*;

#[test]
fn test_web3_client_version() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::web3_client_version());
}

#[test]
fn test_web3_sha3() {
    let mut client = Client::ws();
    let empty = Bytes::from_slice("".as_bytes());
    let expected = H256::from_str(KECCAK_HASH_OF_EMPTY_STRING).unwrap();
    rpc_call_test_expected(&mut client, rpc::web3_sha3(empty), expected);
}
