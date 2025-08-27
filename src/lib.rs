use native_tls::{TlsConnector, TlsStream};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, net::TcpStream};
use tungstenite::{
    client::client,
    handshake::client::{generate_key, Request},
    Message, WebSocket,
};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Certificate {
    pub disk: String,
    pub path: String,
    pub name: String,
    pub alias: String,
}

impl Certificate {
    pub fn get_alias(&self) -> HashMap<&str, &str> {
        self.alias
            .split(",")
            .filter_map(|kv| {
                let mut kv = kv.split("=");
                match (kv.next(), kv.next()) {
                    (Some(k), Some(v)) => Some((k, v)),
                    _ => None,
                }
            })
            .collect()
    }
}

pub struct EIMZOConnection {
    pub socket: WebSocket<TlsStream<TcpStream>>,
}

impl EIMZOConnection {
    fn connect() -> Result<Self, ()> {
        let ws_url = Url::parse("wss://127.0.0.1:64443/service/cryptapi").map_err(|_| ())?;

        // Establish a TCP connection, then wrap the TCP stream with TLS and connect to the server
        let tls_connector = TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|_| ())?;

        let remote_addr = match (ws_url.host(), ws_url.port()) {
            (Some(host), Some(port)) => Some(format!("{host}:{port}")),
            _ => None,
        }
        .ok_or(())?;

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
        let tcp_stream = std::net::TcpStream::connect(remote_addr.clone()).map_err(|_| ())?;
        let tls_stream = tls_connector
            .connect(remote_addr.as_str(), tcp_stream)
            .map_err(|_| ())?;

        let (socket, _) = client(req, tls_stream).map_err(|_| ())?;

        let connection = Self { socket };

        Ok(connection)
    }

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

        self.send_and_wait(Message::Text(set_api_keys.to_string()))
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ListAllCertificatesResponse {
    pub certificates: Vec<Certificate>,
}

pub fn list_all_certificates() -> serde_json::Result<Vec<Certificate>> {
    let mut conn: EIMZOConnection = EIMZOConnection::connect().expect("should connect");

    let _ = conn.set_api_keys();

    let cmd: serde_json::Value = json!({
        "plugin": "pfx",
        "name": "list_all_certificates",
    });

    let value = match conn.send_and_wait(Message::Text(cmd.to_string())) {
        Ok(Message::Text(str)) => serde_json::from_str::<ListAllCertificatesResponse>(&str),
        _ => Ok(ListAllCertificatesResponse::default()),
    };

    value.map(|s| s.certificates)
}
