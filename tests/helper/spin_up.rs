use ethane::connector::Subscription;
use ethane::rpc::{sub::SubscriptionRequest, Rpc};
use ethane::transport::{Http, Request, WebSocket};
use ethane::{Connector, ConnectorError};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::process::{Child, Command};

pub enum ClientWrapper {
    Websocket(Client<WebSocket>),
    Http(Client<Http>),
}

impl ClientWrapper {
    pub fn new_from_env() -> ClientWrapper {
        match std::env::var("CONNECTION")
            .unwrap_or(String::from("http"))
            .as_str()
        {
            "http" => Self::Http(Client::http()),
            "ws" => Self::Websocket(Client::ws()),
            _ => panic!("Please set environment variable 'CONNECTION'. Valid values are either 'http' or 'ws'"),
        }
    }

    pub fn call<U: DeserializeOwned + Debug + PartialEq>(
        &mut self,
        rpc: Rpc<U>,
    ) -> Result<U, ConnectorError> {
        match self {
            Self::Websocket(client) => client.call(rpc),
            Self::Http(client) => client.call(rpc),
        }
    }

    pub fn subscribe<U: DeserializeOwned + Debug>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Subscription<U>, ConnectorError> {
        match self {
            Self::Websocket(client) => client.subscribe(sub_request),
            _ => unimplemented!(),
        }
    }
}

#[allow(dead_code)]
pub struct Client<T: Request> {
    connector: Connector<T>,
    process: Process,
}

impl<T: Request> Client<T> {
    fn call<U: DeserializeOwned + Debug + PartialEq>(
        &mut self,
        rpc: Rpc<U>,
    ) -> Result<U, ConnectorError> {
        self.connector.call(rpc)
    }
}

impl Client<WebSocket> {
    pub fn subscribe<U: DeserializeOwned + Debug>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Subscription<U>, ConnectorError> {
        self.connector.subscribe(sub_request)
    }
}

impl Client<WebSocket> {
    pub fn ws() -> Self {
        let process = Process::new();
        std::thread::sleep(std::time::Duration::from_secs(5));
        let connector =
            Connector::websocket(&format!("ws://127.0.0.1:{}", process.ws_port), &None).unwrap();
        Client { connector, process }
    }
}

impl Client<Http> {
    pub fn http() -> Self {
        let process = Process::new();
        std::thread::sleep(std::time::Duration::from_secs(5));
        let connector =
            Connector::http(&format!("http://127.0.0.1:{}", process.http_port), &None).unwrap();
        Client { connector, process }
    }
}

struct Process {
    cmd: Child,
    http_port: u16,
    ws_port: u16,
}

impl Process {
    fn new() -> Self {
        let (http_port, ws_port) = (
            port_scanner::request_open_port().expect("No port available"),
            port_scanner::request_open_port().expect("No port available"),
        );
        let cmd = Command::new("geth")
            .args(&[
                "--dev",
                "--ws",
                "--ws.api",
                "personal,eth,net,web3,txpool",
                "--ws.port",
                &ws_port.to_string(),
                "--http",
                "--http.api",
                "personal,eth,net,web3,txpool",
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
        } else {
            println!("{}", &e_message);
        }
    }
}
