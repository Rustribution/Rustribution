#[macro_use]
extern crate slog;
#[macro_use]
extern crate std;

use serde::Deserialize;
use slog::Logger;
use std::sync::Arc;
use storage::backend::BlobBackend;

pub mod base;
pub mod blob;
pub mod init_blob_upload;
pub mod manifest;
pub mod media_types;
pub mod tags;

#[derive(Clone)]
pub struct AppState {
    pub logger: Logger,
    // pub config: config::Config,
    pub backend: Arc<dyn BlobBackend + Send + Sync>,
}

#[derive(Deserialize)]
pub struct NameReference {
    name: String,
    reference: String,
}

#[derive(Deserialize)]
pub struct NameDigest {
    name: String,
    digest: String,
}

#[derive(Deserialize)]
pub struct NameUUID {
    name: String,
    uuid: String,
}

#[derive(Deserialize)]
pub struct QueryDigest {
    digest: Option<String>,
}

/// mount: digest
/// form: repository name
#[derive(Deserialize)]
pub struct QueryMount {
    mount: Option<String>,
    from: Option<String>,
}
