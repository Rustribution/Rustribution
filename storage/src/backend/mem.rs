use crate::backend::BlobBackend;
use bytes::Bytes;
use slog::Logger;
use std::collections::HashMap;
use std::io::Result;

#[derive(Debug, Clone)]
pub struct Mem {
    blobs: HashMap<String, Bytes>,
    logger: Logger,
    config: StorageMemCfg,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StorageMemCfg {
    pub maxbytes: u64,
}

impl Mem {
    pub fn new(config: toml::value::Value, logger: Logger) -> Mem {
        info!(logger, "storage config: {:?}", config["mem"]);
        let config: StorageMemCfg =
            toml::from_str(toml::to_string(&config["mem"].as_table()).unwrap().as_str()).unwrap();

        let blobs = HashMap::new();
        Mem {
            config,
            logger,
            blobs,
        }
    }
}

impl BlobBackend for Mem {
    fn info(&self) -> String {
        format!("[Memory storage config] maxbytes: {}", self.config.maxbytes,)
    }

    fn get_content(&self, path: String) -> Result<Bytes> {
        Ok(self.blobs.get(&path).unwrap().clone())
    }

    fn put_content(&self, _path: String, _data: Bytes) {
        // self.blobs.insert(path, data);
    }

    fn delete(&self, path: String) -> Result<()> {
        info!(self.logger, "delete blob";
            "path"=>&path,
        );
        Ok(())
    }
}
