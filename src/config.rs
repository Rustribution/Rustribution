use std::fs::File;
use std::io::{self, Read};
use storage::backend::StorageCfg;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: String,
    pub log: LogCfg,
    pub http: HttpCfg,
    pub storage: StorageCfg,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LogCfg {
    pub level: LogLevel,
    pub environment: Option<String>,
    pub service: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub enum LogLevel {
    #[serde(rename = "off")]
    Off = 0,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "trace")]
    Trace,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct HttpCfg {
    pub addr: String,
    pub compress: Option<bool>,
    pub debug_addr: Option<String>,
    pub prometheus: Option<PrometheusCfg>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PrometheusCfg {
    pub enabled: bool,
    pub path: String,
}

pub fn parse(path: String) -> Result<Config, io::Error> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let config: Config = toml::from_str(&buf).unwrap();

    Ok(config)
}
