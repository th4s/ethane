use dns_lookup::lookup_host;
use http::header::InvalidHeaderValue;
use http::uri::InvalidUri;
use http::{Request, Uri};
use log::{debug, trace};
use mio::net::TcpStream;
use native_tls::{TlsConnector, TlsStream};
use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{AddrParseError, SocketAddr};
use tungstenite::client as ws_client;
use tungstenite::handshake::HandshakeRole;
use tungstenite::WebSocket as WebSocketTungstenite;

pub struct WebSocket<T: Read + Write>(WebSocketTungstenite<T>);

impl WebSocket<TcpStream> {
    #[allow(dead_code)]
    pub fn from_tcp(
        address: &str,
        credentials: Option<Credentials>,
    ) -> Result<WebSocket<TcpStream>, WebSocketError> {
        debug!("Initiating websocket connection to {}...", address);
        let address_with_protocol = String::from("ws://") + &address;
        let handshake_request = create_handshake_request(&address_with_protocol, credentials)?;
        let tcp_stream = open_tcp_stream(address)?;
        let ws = ws_client::client(handshake_request, tcp_stream)?;
        trace!("Handshake Response: {:?}", ws.1);
        Ok(WebSocket(ws.0))
    }
}

impl WebSocket<TlsStream<TcpStream>> {
    #[allow(dead_code)]
    pub fn from_tls_tcp(
        domain: &str,
        port: u16,
        credentials: Option<Credentials>,
    ) -> Result<WebSocket<TlsStream<TcpStream>>, WebSocketError> {
        debug!("Initiating websocket connection to {}...", domain);
        let ip = lookup_host(domain)?;
        let address = ip
            .first()
            .ok_or_else(|| WebSocketError::new("No ip address for domain"))?
            .to_string()
            + ":"
            + &port.to_string();
        let address_with_protocol = String::from("wss://") + &address;
        let handshake_request = create_handshake_request(&address_with_protocol, credentials)?;
        let tcp_stream = open_tcp_stream(&address)?;
        let tls_stream = tls_encrypt_stream(domain, tcp_stream)?;
        let ws = ws_client::client(handshake_request, tls_stream)?;
        trace!("Handshake Response: {:?}", ws.1);
        Ok(WebSocket(ws.0))
    }
}

fn open_tcp_stream(address: &str) -> Result<TcpStream, WebSocketError> {
    trace!("Opening tcp stream to {}...", address);
    let parsed_address: SocketAddr = address.parse()?;
    let tcp_stream = TcpStream::connect(parsed_address)?;
    Ok(tcp_stream)
}

fn tls_encrypt_stream<T: Read + Write + Debug + 'static>(
    domain: &str,
    stream: T,
) -> Result<TlsStream<T>, WebSocketError> {
    trace!("Encrypting TLS stream to {}", domain);
    let connector = TlsConnector::new()?;
    let tls_stream = connector.connect(domain, stream)?;
    Ok(tls_stream)
}

fn create_handshake_request(
    address_with_protocol: &str,
    credentials: Option<Credentials>,
) -> Result<Request<()>, WebSocketError> {
    trace!("Building websocket request...");
    let mut req_builder = Request::get(address_with_protocol.parse::<Uri>()?);
    if let Some(credentials) = credentials {
        let auth_string_base64 = String::from("Basic ")
            + &base64::encode(credentials.username + ":" + &credentials.password);
        let headers = req_builder
            .headers_mut()
            .ok_or_else(|| WebSocketError::new("Error while building headers for handshake"))?;
        headers.insert("Authorization", auth_string_base64.parse()?);
    }
    let request = req_builder.body(())?;
    Ok(request)
}

pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]
pub struct WebSocketError {
    source: Option<Box<dyn Error>>,
    context: Option<String>,
}

impl WebSocketError {
    fn new(context_info: &str) -> Self {
        WebSocketError {
            source: None,
            context: Some(context_info.to_string()),
        }
    }
}

impl std::fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WebSocketError, Source: {},\nContext: {}",
            self.source
                .as_ref()
                .map_or(String::from("Unknown"), |inner| inner.to_string()),
            self.context.as_ref().map_or("None", |inner| inner.as_str())
        )
    }
}

impl Error for WebSocketError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(inner) => Some(inner.as_ref()),
            None => None,
        }
    }
}

impl<T: ErrMarker + Error + 'static> From<T> for WebSocketError {
    fn from(err: T) -> Self {
        WebSocketError {
            source: Some(Box::new(err)),
            context: None,
        }
    }
}

pub trait ErrMarker {}

impl ErrMarker for std::io::Error {}
impl ErrMarker for AddrParseError {}
impl<T: Debug + 'static> ErrMarker for native_tls::HandshakeError<T> {}
impl ErrMarker for native_tls::Error {}
impl ErrMarker for http::Error {}
impl ErrMarker for InvalidUri {}
impl ErrMarker for InvalidHeaderValue {}
impl<T: HandshakeRole + 'static> ErrMarker for tungstenite::HandshakeError<T> {}
