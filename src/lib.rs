#![allow(clippy::never_loop)]
#![allow(clippy::result_large_err)]

pub mod client;
pub mod error;
pub mod prelude;

// Public re-exports
pub use error::{EIMZOError as Error, Result};

use chrono::{Local, NaiveDateTime};
use client::{Client, Connected, Disconnected};
use locale_rs::Locale;
use prelude::*;
use serde_json::json;
use std::str::FromStr;
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
    /// Информацио о версии JVM
    /// doc: https://127.0.0.1:64443/apidoc.html#app.show_menu
    ///
    ///```
    /// let mut eimzo = EIMZO::new().unwrap();
    /// match eimzo.show_menu() {
    ///     Ok(res) => println!("show_menu: {res:#?}"),
    ///     Err(e) => println!("{e}"),
    ///}
    ///```
    pub fn show_menu(&mut self) -> Result<(), error::EIMZOError> {
        let cmd: serde_json::Value = json!({
            "plugin"    :"app",
            "name"      :"show_menu",
        });

        self.client
            .send_and_wait(Message::Text(cmd.to_string().into()))?;

        Ok(())
    }
    /// Информацио о версии JVM
    /// doc: https://127.0.0.1:64443/apidoc.html#app.get_jvm_version
    ///
    ///```
    /// let mut eimzo = EIMZO::new().unwrap();
    /// match eimzo.get_jvm_version() {
    ///     Ok(res) => println!("get_jvm_version: {res:#?}"),
    ///     Err(e) => println!("{e}"),
    ///}
    ///```
    pub fn get_jvm_version(&mut self) -> Result<String, error::EIMZOError> {
        let cmd: serde_json::Value = json!({
            "plugin"    :"app",
            "name"      :"get_jvm_version",
        });

        let response = self
            .client
            .send_and_wait(Message::Text(cmd.to_string().into()))?;

        let msg = match response {
            Message::Text(msg) => msg,
            _ => todo!(), // pattern matching handled on send_and_wait so do something with this
        };

        let version = serde_json::from_str::<GenericTextMessage>(&msg)?;
        Ok(version.message)
    }

    /// Изменить язык интерфейса (не сохраняя в настройках)
    /// doc: https://127.0.0.1:64443/apidoc.html#app.change_ui_lang
    ///
    ///```
    /// let mut eimzo = EIMZO::new().unwrap();
    /// eimzo.change_ui_lang("uz")
    ///```
    pub fn change_ui_lang<T>(&mut self, lang: T) -> Result<(), error::EIMZOError>
    where
        T: AsRef<str> + serde::Serialize,
    {
        let locale = Locale::from_str(lang.as_ref())?;

        let cmd: serde_json::Value = json!({
            "plugin"    :"app",
            "name"      :"change_ui_lang",
            "arguments": [
              locale.as_str()
            ],
        });

        self.client
            .send_and_wait(Message::Text(cmd.to_string().into()))?;

        Ok(())
    }

    /// Получить список всех сертификатов пользователя
    /// doc: https://127.0.0.1:64443/apidoc.html#pfx.list_all_certificates
    ///
    /// ```
    /// let mut eimzo = EIMZO::new()?;
    /// match eimzo.list_all_certificates() {
    ///    Ok(pfx) => {
    ///      let a: Vec<_> = pfx.iter().map(|c| (c, c.get_alias())).collect();
    ///        println!("this is resut list_all_certificates; {a:?}");
    ///        pfx.iter().map(|c| (c, c.get_alias())).for_each(|(c, a)| {
    ///            let validfrom: Vec<_> = a.get("validfrom").unwrap().split(" ").collect();
    ///            let mut year_month_day: Vec<_> = validfrom[0].split(".").collect();
    ///            year_month_day.reverse();
    ///
    ///            println!("CERT: {c:#?}");
    ///           println!("ALIAS: {a:#?}");
    ///            println!("-----");
    ///            println!("DATE: {:#?}", year_month_day.join("."));
    ///        });
    ///    }
    ///    Err(e) => println!("{e}"),
    /// }
    ///```
    pub fn list_all_certificates(&mut self) -> Result<Vec<Certificate>, error::EIMZOError> {
        let cmd: serde_json::Value = json!({
            "plugin": "pfx",
            "name": "list_all_certificates",
        });

        let response = self
            .client
            .send_and_wait(Message::Text(cmd.to_string().into()))?;

        let msg = match response {
            Message::Text(msg) => msg,
            _ => todo!(), // pattern matching handled on send_and_wait so do something with this
        };

        let certs = serde_json::from_str::<ListAllCertificatesResponse>(&msg)
            .unwrap_or_default()
            .certificates
            .into_iter()
            .map(|mut x| {
                let _a = x.get_alias();

                x.valid_from = Some(
                    NaiveDateTime::parse_from_str(
                        _a.get("validfrom").unwrap(),
                        "%Y.%m.%d %H:%M:%S",
                    )
                    .unwrap_or_default(),
                );

                x.valid_to = Some(
                    NaiveDateTime::parse_from_str(_a.get("validto").unwrap(), "%Y.%m.%d %H:%M:%S")
                        .unwrap_or_default(),
                );

                let now = Local::now().naive_local();
                x.is_expired =
                    Some(now.signed_duration_since(x.valid_to.unwrap()).num_seconds() > 0);

                x
            })
            .collect();

        Ok(certs)
    }
}
