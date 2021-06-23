use crate::backend::BlobBackend;
use bytes::Bytes;
use slog::Logger;
use slog_scope;
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

impl BlobBackend for Mem {
    fn set_logger(&mut self, logger: Logger) {
        self.logger = logger;
    }

    fn info(&self) -> String {
        format!("[Memory storage config] maxbytes: {}", self.config.maxbytes,)
    }

    fn get_content(&self, path: String) -> Bytes {
        self.blobs.get(&path).unwrap().clone()
    }

    fn put_content(&mut self, path: String, data: Bytes) {
        self.blobs.insert(path, data);
    }

    // fn read(&self, path: String) {}

    // fn write(&self, path: String) {}

    fn stat(&self, path: String) -> (bool, usize) {
        // self.blobs.contains_key(&path)
        match self.blobs.get(&path) {
            Some(v) => (true, v.len()),
            None => (false, 0),
        }
    }

    // fn list(&self, path: String) {}

    // fn mov(&self, src_path: String, dst_path: String) {}

    // fn delete(&self, path: String) {}
}

pub fn new(config: toml::value::Value) -> Result<Mem> {
    let logger = slog_scope::logger();

    info!(logger, "storage config: {:?}", config["mem"]);
    let config: StorageMemCfg =
        toml::from_str(toml::to_string(&config["mem"].as_table()).unwrap().as_str()).unwrap();

    let blobs = HashMap::new();
    Ok(Mem {
        config,
        logger,
        blobs,
    })
}
