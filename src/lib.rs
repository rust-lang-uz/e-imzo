#![allow(clippy::never_loop)]
#![allow(clippy::result_large_err)]

pub mod client;
pub mod error;
pub mod prelude;

// Public re-exports
pub use error::{EIMZOError as Error, Result};

use client::{Client, Connected, Disconnected};
use prelude::*;
use serde_json::json;
use tungstenite::Message;

pub struct EIMZO<State> {
    client: Client<State>,
}

impl EIMZO<Disconnected> {
    pub fn new() -> Result<EIMZO<Connected>> {
        Ok(EIMZO {
            client: Client::connect::<String>(None)?,
        })
    }
}

impl EIMZO<Connected> {
    pub fn list_all_certificates(&mut self) -> Result<Vec<Certificate>> {
        let _ = self.client.set_api_keys();

        let cmd: serde_json::Value = json!({
            "plugin": "pfx",
            "name": "list_all_certificates",
        });

        let value = match self
            .client
            .send_and_wait(Message::Text(cmd.to_string().into()))
        {
            Ok(Message::Text(str)) => serde_json::from_str::<ListAllCertificatesResponse>(&str),
            _ => Ok(ListAllCertificatesResponse::default()),
        };

        Ok(value.map(|s| s.certificates)?)
    }
}
