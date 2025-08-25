// use tokio_tungstenite::{ connect_async_tls_with_config, tungstenite::{protocol::{Message, WebSocketConfig}}, Connector };
// use serde_json::Value;
// use log::{ info, error, LevelFilter };
// use futures_util::StreamExt;

// #[tokio::main]
// async fn main() {
//     env_logger::builder().filter_level(LevelFilter::Info).init();

//     let url = "wss://127.0.0.1:64443/service/cryptapi";
//     info!("Connecting to {}", url);

//     let tls_connector = native_tls::TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
//     let connector  = Connector::NativeTls(tls_connector);

//     match connect_async_tls_with_config(
//         url,
//         Some(WebSocketConfig::default()),
//         false,
//         Some(connector)
//     ).await {
//         Ok((mut ws_stream, _)) => {
//             info!("Connected to the WebSocket server");

//             // Start receiving messages from the WebSocket stream
//             while let Some(message) = ws_stream.next().await {
//                 match message {
//                     Ok(Message::Text(text)) => handle_message(&text),
//                     Ok(_) => (),
//                     Err(e) => error!("Error during the WebSocket communication: {}", e),
//                 }
//             }
//         }
//         Err(e) => error!("Failed to connect: {}", e),
//     }
// }

// // Function to handle incoming WebSocket messages
// fn handle_message(text: &str) {
//     println!("asdasdasd");
//     match serde_json::from_str::<Value>(&text) {
//         Ok(data) => {
//             info!("{:?}", data); 
//         }
//         Err(_) => error!("Failed to parse the message: {}", text),
//     }
// }


use std::collections::HashMap;

use tungstenite::{client::client, handshake::client::{generate_key, Request}, http::Uri, protocol::{frame::coding::CloseCode, CloseFrame}, Message};
use native_tls::TlsConnector;
use url::Url;
use serde_json::json;
use serde_json::{Result, Value};
use log::error;

type JsonMap = HashMap<String, serde_json::Value>;

fn main() {
    env_logger::init();
    let ws_url = Url::parse("wss://127.0.0.1:64443/service/cryptapi").unwrap();

    // Establish a TCP connection, then wrap the TCP stream with TLS and connect to the server
    let tls_connector = TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
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
    let tls_stream = tls_connector.connect(remote_addr.as_str(), tcp_stream).unwrap();

    let (mut socket, response) = client(
        req,
        tls_stream

    ).expect("Can't connect");

    // Origin: 'https://localhost'
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (header, value) in response.headers() {
        println!("* {header}: {:?}", value);
    }


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
    socket.send(Message::Text(onopen.to_string().into())).unwrap();
    socket.send(Message::Text(list_all_certificates.to_string().into())).unwrap();
    // socket.close(Some(CloseFrame {
    //     code: CloseCode::Normal,
    //     // run.rs:96 implement that later
    //     reason: std::borrow::Cow::Borrowed("finished task successfully"),
    // })).expect("Unable to close socket");
    // socket.close(None).expect("Unable to close socket");

    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {msg}");

        // Parse the string of data
        let foo = msg.to_string();
        let v: Value = json!(&foo);
        // println!("+++++++++++++++++++++++++ {:?}", serde_json::from_str(v));

    let map: JsonMap = serde_json::from_str(&foo).expect("msg");
    for (key, value) in map.iter() {
        println!("KEEEEEEEEEEEEEEEEEEEEEEEY: {:?}----{:?}", key,value);
        match key.as_str() {
            "certificates" => {
                
                socket.close(None).expect("Unable to close socket");
                break;
        },
            _ => (),
        }
    };
//         for item in &v {
//     println!("AND___----___---{:?}\n", item[0]);
// }

        // if let Some(certificates) = v["certificates"].as_array() {
        //     // Access parts of the data by indexing with square brackets.
        //     println!("Please call_____________________________________________________________ {:?}", v);
        //     println!("Please call_____________________________________________________________ ");
        //      break;
        // }

    }
}