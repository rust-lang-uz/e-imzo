#![allow(clippy::never_loop)]
#![allow(clippy::result_large_err)]

pub mod client;
pub mod error;
pub mod prelude;

use client::Client;
use error::Result;
use prelude::*;
use serde_json::json;
use tungstenite::Message;

pub fn list_all_certificates() -> Result<Vec<Certificate>> {
    let mut conn = Client::connect()?;

    let _ = conn.set_api_keys();

    let cmd: serde_json::Value = json!({
        "plugin": "pfx",
        "name": "list_all_certificates",
    });

    let value = match conn.send_and_wait(Message::Text(cmd.to_string().into())) {
        Ok(Message::Text(str)) => serde_json::from_str::<ListAllCertificatesResponse>(&str),
        _ => Ok(ListAllCertificatesResponse::default()),
    };

    Ok(value.map(|s| s.certificates)?)
}
