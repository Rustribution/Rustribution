#[macro_use]
extern crate slog;
#[macro_use]
extern crate std;
#[macro_use]
extern crate custom_error;
#[macro_use]
extern crate more_asserts;

use chrono::prelude::NaiveDateTime;
use serde::{de, Deserialize, Deserializer};
use slog::Logger;
use std::sync::Arc;
use storage::backend::BlobBackend;

pub mod base;
pub mod blob;
pub mod blob_upload;
pub mod errors;
pub mod hmac;
pub mod init_blob_upload;
pub mod manifest;
pub mod tags;

pub static DISTRIBUTION_API_VERSION: &str = "Docker-Distribution-Api-Version";
pub static DOCKER_CONTENT_DIGEST: &str = "Docker-Content-Digest";
pub static RUSTRIBUTION_VERSION: &str = "Rustribution-Version";
pub static DOCKER_UPLOAD_UUID: &str = "Docker-Upload-UUID";
pub static DATATIME_FMT: &str = "%Y-%m-%dT%H:%M:%S.%f";

// #[derive(Clone)]
pub struct AppState {
    pub logger: Logger,
    // pub config: config::Config,
    pub http_secret: String,
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

#[derive(Deserialize, Clone)]
pub struct NameUUID {
    name: String,
    uuid: String,
}

#[derive(Deserialize, Clone)]
pub struct QueryDigest {
    digest: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct QueryState {
    _state: String,
}

/// mount: digest
/// form: repository name
#[derive(Deserialize)]
pub struct QueryMount {
    mount: Option<String>,
    from: Option<String>,
}

pub fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, DATATIME_FMT).map_err(de::Error::custom)
}

pub fn build_blob_temp_upload_path(name: String, uuid: String) -> String {
    format!("/v2/repos/{}/_uploads/{}", name, uuid)
}

pub fn build_blob_path(digest: String) -> String {
    assert_eq!(digest.len(), 71);
    format!("/v2/blobs/sha256/{}/{}", digest[7..9].to_string(), digest)
}
