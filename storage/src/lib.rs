#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod backend;
pub mod factory;
pub mod manifest;
pub mod media_types;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Descriptor {
  #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
  pub media_type: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub size: Option<u64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub digest: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub urls: Option<Vec<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub annotations: Option<HashMap<String, String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub platform: Option<String>,
}
