use crate::error::Result;
use native_tls::{TlsConnector, TlsStream};
use serde_json::json;
use std::marker::{self, PhantomData};
use std::net::TcpStream;
use tungstenite::{
    Message, WebSocket,
    client::client,
    handshake::client::{Request, generate_key},
};
use url::Url;

static BASE_URL: &str = "wss://127.0.0.1:64443";

/// Gateway of connected state for `Client` struct.
pub struct Connected;

/// Gateway of disconnected state for `Client` struct, which is defaulted.
pub struct Disconnected;

/// The client instance for transmitting data through WebSocket connection
pub struct Client<State> {
    pub socket: WebSocket<TlsStream<TcpStream>>,
    /// Connected Marker
    _state: marker::PhantomData<State>,
}

impl Client<Disconnected> {
    pub fn connect() -> Result<Client<Connected>> {
        let ws_url = Url::parse
          (&format!("{BASE_URL}/service/cryptapi")))?;

        // Establish a TCP connection, then wrap the TCP stream with TLS and connect to the server
        let tls_connector = TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let remote_addr = match (ws_url.host(), ws_url.port()) {
            (Some(host), Some(port)) => Some(format!("{host}:{port}")),
            _ => None,
        }
        .ok_or(())
        .unwrap();

        let req = Request::builder()
            .method("GET")
            .header("Host", "localhost")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Origin", "https://localhost")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", generate_key())
            .uri(ws_url.to_string())
            .body(())
            .unwrap();
        let tcp_stream = std::net::TcpStream::connect(remote_addr.clone())?;
        let tls_stream = tls_connector.connect(remote_addr.as_str(), tcp_stream)?;

        let (socket, _) = client(req, tls_stream)?;

        let connection = Client {
            socket,
            _state: PhantomData,
        };

        Ok(connection)
    }
}

impl Client<Connected> {
    pub fn send_and_wait(&mut self, message: Message) -> tungstenite::Result<Message> {
        self.socket.send(message)?;

        while let Ok(message) = self.socket.read() {
            return Ok(message);
        }

        unreachable!();
    }

    pub fn set_api_keys(&mut self) -> tungstenite::Result<Message> {
        let set_api_keys = json!({
            "plugin": "apikey",
            "name": "apikey",
            "arguments": [
                "localhost",
                "96D0C1491615C82B9A54D9989779DF825B690748224C2B04F500F370D51827CE2644D8D4A82C18184D73AB8530BB8ED537269603F61DB0D03D2104ABF789970B",
            ]
        });

        self.send_and_wait(Message::Text(set_api_keys.to_string().into()))
    }
}
