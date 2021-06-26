use crate::backend::*;
use slog::Logger;
use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;

pub fn new_backend(
    config: StorageCfg,
    logger: Logger,
) -> Result<Arc<dyn BlobBackend + Send + Sync>> {
    info!(logger, "try create backend of type {}", config.backend_type);
    match config.backend_type.as_str() {
        #[cfg(feature = "backend-mem")]
        "mem" => {
            let backend = Arc::new(mem::Mem::new(config.backend_config, logger));
            Ok(backend)
        }

        #[cfg(feature = "backend-filesystem")]
        "filesystem" => {
            let backend = Arc::new(filesystem::Filesystem::new(config.backend_config, logger));
            Ok(backend)
        }
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("not support storage: {}", config.backend_type),
        )),
    }
}
