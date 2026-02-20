use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Certificate {
    pub disk: String,
    pub path: String,
    pub name: String,
    pub alias: String,
    pub valid_from: Option<NaiveDateTime>,
    pub valid_to: Option<NaiveDateTime>,
    pub is_expired: Option<bool>

}

impl Certificate {
    pub fn get_alias(&self) -> HashMap<String, String> {
        self.alias
            .split(",")
            .filter_map(|kv| {
                let mut kv = kv.split("=");
                match (kv.next(), kv.next()) {
                    (Some(k), Some(v)) => Some((k.to_string(), v.to_string())),
                    _ => None,
                }
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ListAllCertificatesResponse {
    pub certificates: Vec<Certificate>,
}
