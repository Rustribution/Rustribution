use crate::manifest::{Manifest, Versioned};
use crate::media_types::MediaType;
use crate::Descriptor;
use bytes::Bytes;
use serde::Serialize;
use std::io::Result;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct ManifestList {
    #[serde(rename = "manifests")]
    manifests: Vec<Descriptor>,

    #[serde(flatten)]
    pub versioned: Versioned,
}

impl ManifestList {
    pub fn to_string_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

impl Manifest for ManifestList {
    fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    fn payload(&self) -> Result<(MediaType, Bytes)> {
        let encoded = self.to_string()?;
        Ok((MediaType::ManifestList, Bytes::from(encoded)))
    }
}

#[cfg(test)]
mod tests {
    use crate::manifest::{manifestlist::ManifestList, Versioned};
    use crate::media_types::MediaType;
    use crate::{Descriptor, Platform};

    // use bytes::Bytes;
    use std::io::Result;

    static ALPINE_MANIFEST_LIST_STR: &str = r#"{"manifests":[{"digest":"sha256:1775bebec23e1f3ce486989bfc9ff3c4e951690df84aa9f926497d82f2ffca9d","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"amd64","os":"linux"},"size":528},{"digest":"sha256:1f66b8f3041ef8575260056dedd437ed94e7bfeea142ee39ff0d795f94ff2287","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"arm","os":"linux","variant":"v6"},"size":528},{"digest":"sha256:8d99168167baa6a6a0d7851b9684625df9c1455116a9601835c2127df2aaa2f5","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"arm","os":"linux","variant":"v7"},"size":528},{"digest":"sha256:53b74ddfc6225e3c8cc84d7985d0f34666e4e8b0b6892a9b2ad1f7516bc21b54","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"arm64","os":"linux","variant":"v8"},"size":528},{"digest":"sha256:52a197664c8ed0b4be6d3b8372f1d21f3204822ba432583644c9ce07f7d6448f","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"386","os":"linux"},"size":528},{"digest":"sha256:b421672fe4e74a3c7eff2775736e854d69e8d38b2c337063f8699de9c408ddd3","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platform":{"architecture":"ppc64le","os":"linux"},"size":528},{"digest":"sha256:8a22269106a31264874cc3a719c1e280e76d42dff1fa57bd9c7fe68dab574023","mediaType":"application\/vnd.docker.distribution.manifest.v2+json","platVersioned(2_isize, Some(MediaType::ManifestList.to_string()))docker.distribution.manifest.list.v2+json","schemaVersion":2}"#;

    // const DIGEST: &str = "234cb88d3020898631af0ccbbcca9a66ae7306ecd30c9720690858c1b007d2a0";

    lazy_static! {
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
       pub static ref ALPINE_MANIFEST_LIST: ManifestList = {
          let mut manifests = Vec::new();

          // 1. amd64
          manifests.push(Descriptor{
            digest:Some(String::from("sha256:1775bebec23e1f3ce486989bfc9ff3c4e951690df84aa9f926497d82f2ffca9d")),
            media_type: Some(MediaType::ManifestV2.to_string()),
            platform: Some(Platform{
              architecture: Some(String::from("amd64")),
              variant:None,
              os:Some(String::from("linux"))
            }),
            size:Some(528),
            urls: None,
            annotations: None,
          });

          // 2. arm v6
          manifests.push(Descriptor{
            media_type: Some(MediaType::ManifestV2.to_string()),
            size:Some(528),
            digest:Some(String::from("sha256:1f66b8f3041ef8575260056dedd437ed94e7bfeea142ee39ff0d795f94ff2287")),
            urls: None,
            annotations: None,
            platform: Some(Platform{
              architecture: Some(String::from("arm")),
              variant:Some(String::from("v6")),
              os:Some(String::from("linux"))
            }),
          });

          // 3. arm v7
          manifests.push(Descriptor{
            media_type: Some(MediaType::ManifestV2.to_string()),
            size:Some(528),
            digest:Some(String::from("sha256:8d99168167baa6a6a0d7851b9684625df9c1455116a9601835c2127df2aaa2f5")),
            urls: None,
            annotations: None,
            platform: Some(Platform{
              architecture: Some(String::from("arm")),
              variant:Some(String::from("v7")),
              os:Some(String::from("linux"))
            }),
          });

          // 3. arm v8
          manifests.push(Descriptor{
            media_type: Some(MediaType::ManifestV2.to_string()),
            size:Some(528),
            digest:Some(String::from("sha256:53b74ddfc6225e3c8cc84d7985d0f34666e4e8b0b6892a9b2ad1f7516bc21b54")),
            urls: None,
            annotations: None,
            platform: Some(Platform{
              architecture: Some(String::from("arm64")),
              variant:Some(String::from("v8")),
              os:Some(String::from("linux"))
            }),
          });

          // 4. 386
          manifests.push(Descriptor{
            media_type: Some(MediaType::ManifestV2.to_string()),
            size:Some(528),
            digest:Some(String::from("sha256:52a197664c8ed0b4be6d3b8372f1d21f3204822ba432583644c9ce07f7d6448f")),
            urls: None,
            annotations: None,
            platform: Some(Platform{
              architecture: Some(String::from("386")),
              variant:None,
              os:Some(String::from("linux"))
            }),
          });

          // 5. ppc64le
          manifests.push(Descriptor{
            media_type: Some(MediaType::ManifestV2.to_string()),
            size:Some(528),
            digest:Some(String::from("sha256:b421672fe4e74a3c7eff2775736e854d69e8d38b2c337063f8699de9c408ddd3")),
            urls: None,
            annotations: None,
            platform: Some(Platform{
              architecture: Some(String::from("ppc64le")),
              variant:None,
              os:Some(String::from("linux"))
            }),
          });

          // 6. s390x
          manifests.push(Descriptor{
            media_type: Some(MediaType::ManifestV2.to_string()),
            size:Some(528),
            digest:Some(String::from("sha256:8a22269106a31264874cc3a719c1e280e76d42dff1fa57bd9c7fe68dab574023")),
            urls: None,
            annotations: None,
            platform: Some(Platform{
              architecture: Some(String::from("s390x")),
              variant:None,
              os:Some(String::from("linux"))
            }),
          });

          ManifestList {
          versioned: Versioned {
            schema_version: 2_isize,
            media_type: Some(MediaType::ManifestList.to_string()),
          },
          manifests: manifests,
        }
      };
    }

    #[test]
    fn schema2_de_alpine_manifestlist() -> Result<()> {
        let alpine_manifest_list: ManifestList = serde_json::from_str(ALPINE_MANIFEST_LIST_STR)?;

        println!("deser alpine v2 manifest: {:?}", alpine_manifest_list);
        assert_eq!(
            alpine_manifest_list.versioned,
            ALPINE_MANIFEST_LIST.versioned
        );
        assert_eq!(
            alpine_manifest_list.manifests,
            ALPINE_MANIFEST_LIST.manifests
        );

        Ok(())
    }

    #[test]
    fn schema2_ser_alpine_manifestlist() -> Result<()> {
        // !!!WARNING: not some with dockerhub
        Ok(())
    }
}
