#[cfg(feature = "backend-filesystem")]
pub mod filesystem;
#[cfg(feature = "backend-mem")]
pub mod mem;

use bytes::Bytes;
use slog::Logger;
use std::io::{Error, ErrorKind, Result};
use toml::value::Value;

#[derive(Debug)]
pub struct Blob {
    pub data: Bytes,
    pub size: u64,
}

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

    fn get_content(&self, path: String) -> Bytes;

    fn put_content(&mut self, path: String, data: Bytes);

    fn stat(&self, _path: String) -> (bool, usize) {
        (false, 0)
    }

    fn list(&self, _path: String) {}

    fn mov(&self, _src_path: String, _dst_path: String) {}

    fn delete(&self, _path: String) {}

    fn url_for(&self, _path: String) -> Result<String> {
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("not support url_for"),
        ))
    }
}
