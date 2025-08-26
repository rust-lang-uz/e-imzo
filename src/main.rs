use native_tls::TlsConnector;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tungstenite::{
    client::client,
    handshake::client::{generate_key, Request},
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};
use url::Url;

#[derive(Serialize, Deserialize)]
struct UserData {
    certificates: Vec<HashMap<String, String>>,
    success: bool,
    status: u32,
}

fn main() {
    env_logger::init();
    let ws_url = Url::parse("wss://127.0.0.1:64443/service/cryptapi").unwrap();

    // Establish a TCP connection, then wrap the TCP stream with TLS and connect to the server
    let tls_connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let remote_addr = format!("{}:{}", ws_url.host().unwrap(), ws_url.port().unwrap());

    let req = Request::builder()
        .method("GET")
        .header("Host", "localhost")
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Origin", "https://localhost")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .uri("wss://127.0.0.1:64443/service/cryptapi")
        .body(())
        .unwrap();
    let tcp_stream = std::net::TcpStream::connect(remote_addr.clone()).unwrap();
    let tls_stream = tls_connector
        .connect(remote_addr.as_str(), tcp_stream)
        .unwrap();

    let (mut socket, response) = client(req, tls_stream).expect("Can't connect");
    println!("Response HTTP code: {}", response.status());

    // Always send first to verify that we use apikey
    let onopen = json!({
        "plugin": "apikey",
        "name": "apikey",
        "arguments": [
            "localhost",
            "96D0C1491615C82B9A54D9989779DF825B690748224C2B04F500F370D51827CE2644D8D4A82C18184D73AB8530BB8ED537269603F61DB0D03D2104ABF789970B",
        ]
    });

    let list_all_certificates = json!({
        "plugin": "pfx",
        "name": "list_all_certificates",
    });
    socket
        .send(Message::Text(onopen.to_string().into()))
        .unwrap();
    socket
        .send(Message::Text(list_all_certificates.to_string().into()))
        .unwrap();

    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {msg}");

        // Parse the string of data
        let foo = msg.to_string();
        let v: Value = serde_json::from_str(&foo).unwrap();
        println!("+++++++++++++++++++++++++ {:?}", v["certificates"]);

        if v["certificates"].is_array() {
            socket
                .close(Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: std::borrow::Cow::Borrowed("finished task successfully"),
                }))
                .expect("Unable to close socket");
            break;
        }
    }

    // let e_imzo_client = {
    //     let url = "wss://127.0.0.1:64443/service/cryptapi";

    // };
    // println!("{:?}" , e_imzo_client.url);
}
