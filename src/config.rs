use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub enum DnsType {
    A,
    AAAA,
}

impl DnsType {
    pub fn as_str(&self) -> &str {
        match self {
            DnsType::A => "A",
            DnsType::AAAA => "AAAA",
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) account: AccountConfig,
    pub(crate) dns: DnsConfig,
}

#[derive(Deserialize, Debug)]
pub(crate) struct AccountConfig {
    pub(crate) auth_key: String,
    pub(crate) zone_id: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DnsConfig {
    pub(crate) record: String,
    pub(crate) r#type: DnsType,
    pub(crate) proxied: bool,
}

impl Config {
    pub fn new() -> Self {
        let config: Config =
            toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap();
        config
    }
}
