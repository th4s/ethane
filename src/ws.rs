use http::header::InvalidHeaderValue;
use http::{Request, Uri};
use log::{debug, trace};
use std::error::Error;
use tungstenite::client::AutoStream;
use tungstenite::{connect as ws_connect, Message, WebSocket as WebSocketTungstenite};

/// Convenience wrapper over a [websocket](tungstenite::WebSocket) connection of the [tungstenite crate](tungstenite)
pub struct WebSocket(WebSocketTungstenite<AutoStream>);

impl WebSocket {
    /// Create a new websocket connection to the specified Uri.
    /// When used with [credentials](Credentials), will try to attempt HTTP basic authentication for the handshake request.
    pub fn new(uri: &Uri, credentials: Option<Credentials>) -> Result<WebSocket, WebSocketError> {
        debug!("Initiating websocket connection to {}", &uri.to_string());
        let handshake_request = create_handshake_request(&uri, credentials)?;
        let ws = ws_connect(handshake_request)?;
        trace!("Handshake Response: {:?}", ws.1);
        Ok(WebSocket(ws.0))
    }

    fn read(&mut self) -> Result<Message, WebSocketError> {
        let message = self.0.read_message()?;
        trace!("Reading from websocket: {}", &message);
        Ok(message)
    }

    fn write_text(&mut self, message: &str) -> Result<(), WebSocketError> {
        trace!("Writing to websocket: {}", message);
        self.0.write_message(Message::Text(message.to_string()))?;
        Ok(())
    }
}

fn create_handshake_request(
    uri: &Uri,
    credentials: Option<Credentials>,
) -> Result<Request<()>, WebSocketError> {
    trace!("Building websocket handshake request");
    let mut req_builder = Request::get(uri);
    if let Some(credentials) = credentials {
        let auth_string_base64 = String::from("Basic ")
            + &base64::encode(credentials.username + ":" + &credentials.password);
        let headers = req_builder
            .headers_mut()
            .ok_or_else(|| WebSocketError::new("Error while building headers for handshake"))?;
        headers.insert("Authorization", auth_string_base64.parse()?);
    }
    let request = req_builder.body(())?;
    trace!("Built request: {:?}", &request);
    Ok(request)
}

/// Used for HTTP basic authentication during the handshake request
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// Collect all kinds of possible websocket errors
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

impl std::fmt::Debug for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut err_string = String::from("WebSocketError: ");
        if let Some(ref source) = self.source {
            err_string.push_str(&format!("{}: {:?}", "Source: ", source.as_ref()));
        }
        if let Some(ref context) = self.context {
            err_string.push_str(&format!(" Context: {}", context));
        }
        write!(f, "{:?}", err_string)
    }
}

impl std::fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut err_string = String::from("WebSocketError: ");
        if let Some(ref source) = self.source {
            err_string.push_str(&format!("{}: {}", "Source: ", source.as_ref()));
        }
        if let Some(ref context) = self.context {
            err_string.push_str(&format!(" Context: {}", context));
        }
        write!(f, "{}", err_string)
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

/// Used to wrap errors of underlying libraries into [WebSocketError](WebSocketError)
pub trait ErrMarker {}

impl ErrMarker for http::Error {}
impl ErrMarker for InvalidHeaderValue {}
impl ErrMarker for tungstenite::Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddr, TcpStream};
    use tungstenite::{accept, Message};

    fn spawn_websocket_server<F>(mut handle_ws_stream: F, port: u16)
    where
        F: FnMut(&mut WebSocketTungstenite<TcpStream>) + Send + 'static,
    {
        let tcp_listener =
            std::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();

        let _thread = std::thread::Builder::new()
            .name("Websocket Server".to_string())
            .spawn(move || loop {
                match tcp_listener.accept() {
                    Ok((tcp_stream, _address)) => match accept(tcp_stream) {
                        Ok(mut websocket) => handle_ws_stream(&mut websocket),
                        Err(err) => panic!("{}", err),
                    },
                    Err(err) => panic!("{}", err),
                }
            })
            .is_ok();
    }

    fn ping_pong(ws_stream: &mut WebSocketTungstenite<TcpStream>) {
        match ws_stream.read_message() {
            Ok(message) => match message {
                Message::Text(echo) => ws_stream
                    .write_message(Message::Text(echo + " Pong"))
                    .unwrap(),
                _ => panic!("Received other message type."),
            },
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn test_create_handshake_request_with_credentials() {
        let uri = Uri::from_static("localhost");
        let credentials = Credentials {
            username: String::from("abc"),
            password: String::from("123"),
        };
        let request = create_handshake_request(&uri, Some(credentials)).unwrap();
        assert_eq!(
            request.headers().get("Authorization").unwrap(),
            "Basic YWJjOjEyMw=="
        );
    }

    #[test]
    fn test_create_handshake_request_without_credentials() {
        let uri = Uri::from_static("localhost");
        let request = create_handshake_request(&uri, None).unwrap();
        assert_eq!(request.method(), http::method::Method::GET);
        assert_eq!(request.uri(), &uri);
    }

    #[test]
    fn test_new() {
        spawn_websocket_server(ping_pong, 3001);
        let uri = Uri::from_static("ws://localhost:3001");
        let mut ws_client = WebSocket::new(&uri, None).unwrap();
        ws_client.write_text("Ping").unwrap();
        match ws_client.read() {
            Ok(Message::Text(text)) => assert_eq!(text, "Ping Pong"),
            _ => assert!(false),
        };
    }
}
