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

        let value: Result<ListAllCertificatesResponse, ()> = match self
            .client
            .send_and_wait(Message::Text(cmd.to_string().into()))
        {
            Ok(Message::Text(str)) => {
                let _c = serde_json::from_str::<ListAllCertificatesResponse>(&str)
                    .unwrap_or_default()
                    .certificates;

                let new_c = _c.into_iter().map(|x| x.clone()).collect();

                Ok(ListAllCertificatesResponse {
                    certificates: new_c,
                })

                // convert string "2027.07.23 17:44:06" into "23.07.2027"
                // let validfrom: Vec<_> = c.get_alias().get("validfrom").unwrap().split(" ").collect();
                // let mut validfrom_dmy: Vec<_> = validfrom[0].split(".").collect();
                // validfrom_dmy.reverse();

                // let validto: Vec<_> = c.get_alias().get("validto").unwrap().split(" ").collect();
                // let mut validto_dmy: Vec<_> = validto[0].split(".").collect();
                // validto_dmy.reverse();
            }
            _ => Ok(ListAllCertificatesResponse::default()),
        };

        Ok(value.map(|s| s.certificates)?)
    }
}
