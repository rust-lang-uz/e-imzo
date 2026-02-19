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
        let cmd: serde_json::Value = json!({
            "plugin": "pfx",
            "name": "list_all_certificates",
        });

        let Ok(Message::Text(msg)) = self
            .client
            .send_and_wait(Message::Text(cmd.to_string().into()))
        else {
            return Ok(Default::default());
        };

        let certs = serde_json::from_str::<ListAllCertificatesResponse>(&msg)
            .unwrap_or_default()
            .certificates
            .into_iter()
            .map(|x| x.clone())
            .collect();

        Ok(certs)
    }
}
