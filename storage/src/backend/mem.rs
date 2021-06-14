use crate::backend::BlobBackend;
use slog::Logger;
use slog_scope;
use std::io::Result;

#[derive(Debug)]
pub struct Mem {
    logger: Logger,
    config: StorageMemCfg,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StorageMemCfg {
    pub maxbytes: u64,
}

impl BlobBackend for Mem {
    fn set_logger(&mut self, logger: Logger) {
        self.logger = logger;
    }

    fn info(&self) -> String {
        format!("[Memory storage config] maxbytes: {}", self.config.maxbytes,)
    }
}

pub fn new(config: toml::value::Value) -> Result<Mem> {
    let logger = slog_scope::logger();

    info!(logger, "storage config: {:?}", config["mem"]);
    let config: StorageMemCfg =
        toml::from_str(toml::to_string(&config["mem"].as_table()).unwrap().as_str()).unwrap();

    Ok(Mem { config, logger })
}
