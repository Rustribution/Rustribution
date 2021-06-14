use crate::backend::BlobBackend;
use slog::Logger;
use std::io::Result;

#[derive(Debug)]
pub struct Filesystem {
    logger: Logger,
    config: StorageFilesystemCfg,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StorageFilesystemCfg {
    pub rootdir: String,
}

impl BlobBackend for Filesystem {
    fn set_logger(&mut self, logger: Logger) {
        self.logger = logger;
    }

    fn info(&self) -> String {
        format!(
            "[Filesystem storage config] rootdir: {}",
            self.config.rootdir,
        )
    }
}

pub fn new(config: toml::value::Value) -> Result<Filesystem> {
    let logger = slog_scope::logger();
    info!(logger, "storage config: {:?}", config["filesystem"]);
    let config: StorageFilesystemCfg = toml::from_str(
        toml::to_string(&config["filesystem"].as_table())
            .unwrap()
            .as_str(),
    )
    .unwrap();
    Ok(Filesystem { config, logger })
}
