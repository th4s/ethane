use ethane::connector::Subscription;
use ethane::rpc::{sub::SubscriptionRequest, Rpc};
use ethane::transport::{Http, Request, Subscribe, WebSocket};
use ethane::{Connector, ConnectorError};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::process::{Child, Command};

pub enum ConnectorWrapper {
    Websocket(ConnectorNodeBundle<WebSocket>),
    Http(ConnectorNodeBundle<Http>),
}

impl ConnectorWrapper {
    pub fn new_from_env() -> ConnectorWrapper {
        match std::env::var("CONNECTION")
            .unwrap_or_else(|_| String::from("http"))
            .as_str()
        {
            "http" => Self::Http(ConnectorNodeBundle::http()),
            "ws" => Self::Websocket(ConnectorNodeBundle::ws()),
            _ => panic!("Please set environment variable 'CONNECTION'. Valid values are either 'http' or 'ws'"),
        }
    }

    pub fn call<U: DeserializeOwned + Debug + PartialEq>(
        &mut self,
        rpc: Rpc<U>,
    ) -> Result<U, ConnectorError> {
        match self {
            Self::Websocket(connector) => connector.call(rpc),
            Self::Http(connector) => connector.call(rpc),
        }
    }

    pub fn subscribe<U: DeserializeOwned + Debug + PartialEq, T: Subscribe>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Subscription<U, T>, ConnectorError> {
        match self {
            Self::Websocket(connector) => connector.subscribe(sub_request),
            _ => unimplemented!(),
        }
    }
}

#[allow(dead_code)]
pub struct ConnectorNodeBundle<T> {
    connector: Connector<T>,
    process: NodeProcess,
}

impl<T: Request> ConnectorNodeBundle<T> {
    fn call<U: DeserializeOwned + Debug + PartialEq>(
        &mut self,
        rpc: Rpc<U>,
    ) -> Result<U, ConnectorError> {
        self.connector.call(rpc)
    }
}

impl<T: Subscribe + Request> ConnectorNodeBundle<T> {
    pub fn subscribe<U: DeserializeOwned + Debug>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Subscription<U, T>, ConnectorError> {
        self.connector.subscribe(sub_request)
    }
}

impl<T> ConnectorNodeBundle<T> {}

impl ConnectorNodeBundle<WebSocket> {
    pub fn ws() -> Self {
        let process = NodeProcess::new(None, None);
        let connector =
            Connector::websocket(&format!("ws://127.0.0.1:{}", process.ws_port), None).unwrap();
        ConnectorNodeBundle { connector, process }
    }
}

impl ConnectorNodeBundle<Http> {
    pub fn http() -> Self {
        let process = NodeProcess::new(None, None);
        let connector =
            Connector::http(&format!("http://127.0.0.1:{}", process.http_port), None).unwrap();
        ConnectorNodeBundle { connector, process }
    }
}

pub struct NodeProcess {
    pub http_port: u16,
    pub ws_port: u16,
    cmd: Child,
}

impl NodeProcess {
    pub fn new(mut http_port: Option<u16>, mut ws_port: Option<u16>) -> Self {
        if http_port.is_none() {
            http_port = Some(port_scanner::request_open_port().expect("No port available"));
        }
        if ws_port.is_none() {
            ws_port = Some(port_scanner::request_open_port().expect("No port available"));
        }

        let cmd = Command::new("geth")
            .args(&[
                "--dev",
                "--ws",
                "--ws.api",
                "personal,eth,net,web3,txpool",
                "--ws.port",
                &ws_port.unwrap().to_string(),
                "--http",
                "--http.api",
                "personal,eth,net,web3,txpool",
                "--http.port",
                &http_port.unwrap().to_string(),
                "--allow-insecure-unlock",
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("Unable to start local geth node for integration tests. Is geth installed?");
        std::thread::sleep(std::time::Duration::from_secs(5));
        NodeProcess {
            cmd,
            http_port: http_port.unwrap(),
            ws_port: ws_port.unwrap(),
        }
    }
}

impl Drop for NodeProcess {
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
