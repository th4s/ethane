use ethane::rpc;
use ethane::types::{PrivateKey, H160, H256};
use std::str::FromStr;

pub mod helper;
use helper::*;

#[test]
fn test_personal_list_accounts() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::personal_list_accounts());
}

#[test]
fn test_personal_import_raw_key() {
    let mut client = Client::ws();
    let pk: PrivateKey = PrivateKey::NonPrefixed(
        H256::from_str("fdc861959d1768d936bf17eec56260d4de3a7473e58c349e31beba539e5fc88d").unwrap(),
    );
    let expected_address: H160 =
        H160::from_str("0xDc677f7C5060B0b441d30F361D0c8529Ac04E099").unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::personal_import_raw_key(pk, String::from(ACCOUNTS_PASSWORD)),
        expected_address,
    )
}

#[test]
fn test_personal_unlock_account() {
    let mut client = Client::ws();
    let secret = create_secret();
    let pw = String::from(ACCOUNTS_PASSWORD);
    let address = client
        .call(rpc::personal_import_raw_key(
            PrivateKey::NonPrefixed(secret),
            pw.clone(),
        ))
        .unwrap();
    rpc_call_test_expected(
        &mut client,
        rpc::personal_unlock_account(address, pw, None),
        true,
    );
}
