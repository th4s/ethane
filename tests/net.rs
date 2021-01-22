use ethane::rpc;

pub mod helper;
use helper::*;

#[test]
fn test_net_version() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::net_version());
}

#[test]
fn test_net_peer_count() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::net_peer_count());
}

#[test]
fn test_net_listening() {
    let mut client = Client::ws();
    rpc_call_test_some(&mut client, rpc::net_listening());
}
