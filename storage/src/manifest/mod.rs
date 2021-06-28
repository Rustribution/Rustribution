use crate::media_types::MediaType;
use crate::Descriptor;

use bytes::Bytes;
use std::io::Result;
use std::sync::Arc;

pub mod manifestlist;
pub mod schema2;

pub static INDENT: &[u8] = b"   ";

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Versioned {
    #[serde(rename = "schemaVersion")]
    pub schema_version: isize,

    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

pub trait Manifest {
    fn to_string(&self) -> Result<String>;

    fn references(&self) -> Result<Vec<Descriptor>> {
        let refers: Vec<Descriptor> = Vec::new();
        Ok(refers)
    }

    // Returns media type and bytes.
    fn payload(&self) -> Result<(MediaType, Bytes)> {
        let encoded = self.to_string()?;
        Ok((MediaType::None, Bytes::from(encoded)))
    }
}

pub trait ManifestBuilder {
    fn build() -> Result<Arc<dyn Manifest + Send + Sync>>;

    fn references(&self) -> Result<Vec<Descriptor>>;

    fn append_reference(&mut self, dep: Descriptor) -> Result<()>;
}

pub trait ManifestService {
    // return true if the manifest exists.
    fn exists(&self, digest: String) -> Result<bool>;

    // retrieves the manifest specified by the given digest.
    fn get(&self, digest: String) -> Result<String>;

    //
    fn put(&self, manifests: Bytes) -> Result<String>;

    //
    fn delete(&self, digest: String) -> Result<()>;
}
