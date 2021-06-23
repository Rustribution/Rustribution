use crate::backend::*;
use crate::metadata::*;
use slog::Logger;
use std::io::{Error, ErrorKind, Result};
use std::sync::{Arc, Mutex};

pub fn new_backend(
    config: StorageCfg,
    logger: Logger,
) -> Result<Arc<Mutex<dyn BlobBackend + Send + Sync>>> {
    info!(logger, "try create backend of type {}", config.backend_type);
    match config.backend_type.as_str() {
        #[cfg(feature = "backend-mem")]
        "mem" => {
            let backend = Arc::new(Mutex::new(mem::new(config.backend_config)?));
            backend.lock().unwrap().set_logger(logger);
            Ok(backend)
        }

        #[cfg(feature = "backend-filesystem")]
        "filesystem" => {
            let backend = Arc::new(Mutex::new(filesystem::new(config.backend_config)?));
            backend.lock().unwrap().set_logger(logger);
            return Ok(backend);
        }
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("not support storage: {}", config.backend_type),
        )),
    }
}

pub fn new_metadata() -> Result<Arc<dyn Metadata + Send + Sync>> {
    Err(Error::new(ErrorKind::InvalidData, ""))
}
