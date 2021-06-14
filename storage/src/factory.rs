use crate::backend::*;
// use crate::backend::*;
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
        "mem" => Ok(Arc::new(mem::new(config.backend_config)?)),

        #[cfg(feature = "backend-filesystem")]
        "filesystem" => Ok(Arc::new(filesystem::new(config.backend_config)?)),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("not support storage: {}", config.backend_type),
        )),
    }
}
