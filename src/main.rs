use native_tls::{TlsConnector, TlsStream};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};
use tungstenite::{
    client::client,
    handshake::client::{generate_key, Request},
    ClientHandshake, HandshakeError, Message, WebSocket,
};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Certificate {
    disk: String,
    path: String,
    name: String,
    alias: String,
}

#[derive(Serialize, Deserialize)]
struct UserData {
    certificates: Vec<Certificate>,
    success: bool,
    status: u32,
}

fn main() {
    env_logger::init();
    let pfx = list_all_certificates();
    println!("this is resut list_all_certificates; {:?}", pfx);
}

struct EIMZOConnection {
    socket: WebSocket<TlsStream<TcpStream>>,
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

        println!("{remote_addr}");

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

    fn send_and_wait(&mut self, message: Message) -> tungstenite::Result<Message> {
        self.socket.send(message)?;

        while let Ok(message) = self.socket.read() {
            return Ok(message);
        }

        unreachable!();
    }

    fn set_api_keys(&mut self) -> tungstenite::Result<Message> {
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
struct ListAllCertificatesResponse {
    pub certificates: Vec<Certificate>,
}

fn list_all_certificates() -> serde_json::Result<Vec<Certificate>> {
    let mut conn = EIMZOConnection::connect().expect("should connect");

    let _ = conn.set_api_keys();

    let cmd = json!({
        "plugin": "pfx",
        "name": "list_all_certificates",
    });

    let value = match conn.send_and_wait(Message::Text(cmd.to_string())) {
        Ok(Message::Text(str)) => serde_json::from_str::<ListAllCertificatesResponse>(&str),
        _ => Ok(ListAllCertificatesResponse::default()),
    };

    value.map(|s| s.certificates)
}
