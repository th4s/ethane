use lucita::{Call, Credentials, GethConnector, Rpc};

fn read_test_env() {
    dotenv::from_filename("integration-test.env").expect(
        "Integration testing not possible.\
     File 'integration-test.env' is missing",
    );
}

fn get_connection() -> (String, Option<Credentials>) {
    read_test_env();
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

#[test]
fn test_basic_connection_tls() {
    let (address, credentials) = get_connection();
    let mut geth = GethConnector::ws(&address, credentials).unwrap();
    let _closed = geth.close().unwrap();
    assert!(true);
}

#[test]
fn test_geth_net_version() {
    let (address, credentials) = get_connection();
    let mut geth = GethConnector::ws(&address, credentials).unwrap();
    let net_version = geth.call(Rpc::net_version(1)).unwrap();
    assert_eq!(net_version.result, "1");
}
