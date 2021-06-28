use crate::manifest::{Manifest, Versioned};
use crate::media_types::MediaType;
use crate::Descriptor;
use bytes::Bytes;
use serde::Serialize;
use std::io::Result;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Schema2Manifest {
    #[serde(flatten)]
    pub versioned: Versioned,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Descriptor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Descriptor>>,
}

impl Schema2Manifest {
    pub fn to_string_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

impl Manifest for Schema2Manifest {
    fn to_string(&self) -> Result<String> {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(crate::manifest::INDENT);
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        self.serialize(&mut ser)?;
        Ok(String::from_utf8(ser.into_inner()).unwrap())
    }

    fn payload(&self) -> Result<(MediaType, Bytes)> {
        Ok((MediaType::ManifestV2, Bytes::from(self.to_string()?)))
    }
}

#[cfg(test)]
mod tests {
    use crate::manifest::Manifest;
    use crate::manifest::{schema2::Schema2Manifest, Versioned};
    use crate::media_types::MediaType::{ContainerConfig, Layer, ManifestV2};
    use crate::Descriptor;

    use bytes::Bytes;
    use std::io::Result;

    static ALPINE_MANIFEST_STR: &str = r#"{
   "schemaVersion": 2,
   "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
   "config": {
      "mediaType": "application/vnd.docker.container.image.v1+json",
      "size": 1472,
      "digest": "sha256:d4ff818577bc193b309b355b02ebc9220427090057b54a59e73b79bdfe139b83"
   },
   "layers": [
      {
         "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
         "size": 2811478,
         "digest": "sha256:5843afab387455b37944e709ee8c78d7520df80f8d01cf7f861aae63beeddb6b"
      }
   ]
}"#;

    lazy_static! {
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
       pub static ref ALPINE_MANIFEST: Schema2Manifest = {
          let config = Descriptor{
            media_type: Some(ContainerConfig.to_string()),
            size:Some(1472),
            digest:Some(String::from("sha256:d4ff818577bc193b309b355b02ebc9220427090057b54a59e73b79bdfe139b83")),
            urls: None,
            annotations: None,
            platform: None
          };

          let mut layers = Vec::new();
          layers.push(Descriptor{
            media_type: Some(Layer.to_string()),
            size:Some(2811478),
            digest:Some(String::from("sha256:5843afab387455b37944e709ee8c78d7520df80f8d01cf7f861aae63beeddb6b")),
            urls: None,
            annotations: None,
            platform: None
          });
          Schema2Manifest {
          versioned: Versioned {
            schema_version: 2_isize,
            media_type: Some(ManifestV2.to_string()),
          },
          config: Some(config),
          layers: Some(layers),
        }
      };
    }

    #[test]
    fn schema2_to_json() -> Result<()> {
        let manifest = &*ALPINE_MANIFEST;
        let sered = manifest.to_string()?;
        println!("want manifest string: {}", ALPINE_MANIFEST_STR);
        println!("sered manifest string: {}", sered);
        assert_eq!(sered.as_str(), ALPINE_MANIFEST_STR);

        Ok(())
    }

    #[test]
    fn schema2_payload() -> Result<()> {
        let manifest = &*ALPINE_MANIFEST;
        let (media_type, sered) = manifest.payload()?;
        println!("sered manifest string: {:?}", sered);
        assert_eq!(media_type, ManifestV2);
        assert_eq!(sered, Bytes::from(ALPINE_MANIFEST_STR));

        Ok(())
    }

    #[test]
    fn schema2_de_alpine_manifest() -> Result<()> {
        let alpine_manifest: Schema2Manifest = serde_json::from_str(ALPINE_MANIFEST_STR)?;

        println!("deser alpine v2 manifest: {:?}", alpine_manifest);
        assert_eq!(alpine_manifest.versioned, ALPINE_MANIFEST.versioned);
        assert_eq!(alpine_manifest.config, ALPINE_MANIFEST.config);
        assert_eq!(alpine_manifest.layers, ALPINE_MANIFEST.layers);

        Ok(())
    }
}
