use lucita::ws::Credentials;
use lucita::ws::WebSocket;

fn read_test_env() {
    dotenv::from_filename("integration-test.env").expect(
        "Integration testing not possible.\
     File 'integration-test.env' is missing",
    );
}

#[test]
fn test_basic_connection_tls() {
    read_test_env();
    let address = dotenv::var("ETH_WS_TEST_SERVER").expect("Var ETH_WS_TEST_SERVER is not set");
    let username = dotenv::var("USERNAME").expect("Var USERNAME is not set");
    let password = dotenv::var("PASSWORD").expect("Var PASSWORD is not set");
    let credentials = Credentials { username, password };
    let _ws_client = WebSocket::new(&address.parse().unwrap(), Some(credentials)).unwrap();
    assert!(true);
}
