#[cfg(feature = "backend-filesystem")]
pub mod filesystem;
#[cfg(feature = "backend-mem")]
pub mod mem;

use slog::Logger;
use toml::value::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StorageCfg {
    #[serde(rename = "type")]
    pub backend_type: String,
    #[serde(rename = "config")]
    pub backend_config: Value,
}

pub trait BlobBackend {
    fn set_logger(&mut self, logger: Logger);

    fn info(&self) -> String;
}
