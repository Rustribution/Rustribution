// extern crate base64;

use crate::errors::HandlerError;
use crate::naive_date_time_from_str;

use base64::{decode_config, encode_config, URL_SAFE};
use chrono::prelude::NaiveDateTime;
use hmac::{Hmac, Mac, NewMac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::result::Result;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct BlobUploadState {
    // the primary repository under which the blob will be linked.
    pub name: String,
    // identifies the upload.
    pub uuid: String,
    // the current progress of the upload.
    pub offset: u64,
    // the original start time of the upload.
    #[serde(deserialize_with = "naive_date_time_from_str")]
    pub started_at: NaiveDateTime,
}

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

pub struct UploadStater(String);

/// UploadStater: Pack and unpack `BlobUploadState` with `Secret` String.
impl UploadStater {
    pub fn new(key: String) -> UploadStater {
        UploadStater(key)
    }

    pub fn pack(&self, bus: BlobUploadState) -> Result<String, HandlerError> {
        assert_ge!(self.0.len(), 32);

        let json = serde_json::to_vec(&bus).ok();

        if json == None {
            return Err(HandlerError::InvalidEndcodeJSON);
        }
        let mut json = json.unwrap();

        let mac = HmacSha256::new_from_slice(self.0.as_bytes());
        match mac {
            Ok(mut mac) => {
                mac.update(&*json);
                let mut result = mac.finalize().into_bytes().to_vec();
                // base64urlsafe(result+json)
                result.append(&mut json);
                Ok(encode_config(result, URL_SAFE))
            }
            Err(e) => Err(HandlerError::HamcError {
                msg: format!("{:?}", e),
            }),
        }
    }

    pub fn unpack(self, token: String) -> Result<BlobUploadState, HandlerError> {
        assert_ge!(self.0.len(), 32);

        let token_bytes = decode_config(&token, URL_SAFE).ok();
        if token_bytes == None {
            return Err(HandlerError::InvalidBase64);
        }
        let token_bytes = token_bytes.unwrap();

        let mac = HmacSha256::new_from_slice(self.0.as_bytes());
        match mac {
            Ok(mut mac) => {
                let result = mac.clone().finalize().into_bytes();
                let mac_size = result.len();

                // println!(
                //     "mac size: {}, teken bytes size: {}",
                //     mac_size,
                //     token_bytes.len(),
                // );

                if token_bytes.len() < mac_size {
                    return Err(HandlerError::InvalidSecret {
                        msg: format!("token to short"),
                    });
                }

                let mac_bytes = token_bytes[0..mac_size].to_vec();
                let msg_bytes = &token_bytes[mac_size..];
                // println!("msg_bytes: {}", std::str::from_utf8(msg_bytes).unwrap());

                mac.update(msg_bytes.clone());
                let calc_bytes = mac.finalize().into_bytes().to_vec();

                if calc_bytes != mac_bytes {
                    return Err(HandlerError::InvalidSecret {
                        msg: format!("valid failed"),
                    });
                }

                let state: BlobUploadState = serde_json::from_slice(msg_bytes.clone()).unwrap();
                // println!("unpacked state: {:?}", state);

                Ok(state)
            }
            Err(e) => Err(HandlerError::HamcError {
                msg: format!("{:?}", e),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hmac::{BlobUploadState, HmacSha256, UploadStater};
    use crate::DATATIME_FMT;
    use chrono::prelude::NaiveDateTime;
    use hmac::{Mac, NewMac};

    const SECRET: &str = "9fa65e8af8a0480ba88dcdca3d83184b";
    const ENCODED: &str ="jNPCCtoc7KjOBgPAPRqctheFzTFQOMAsYZ29hg2aenp7Im5hbWUiOiJhbHBpbmUiLCJ1dWlkIjoiMmQ2MzQ3YjUtYjhlYy00OTdlLWIzYmQtYWMyZDBmNDgzYzdkIiwib2Zmc2V0IjoyODExNDc4LCJzdGFydGVkX2F0IjoiMjAyMS0wNi0xOVQwNjozNjowNC45Nzg1OSJ9";

    fn raw_state() -> BlobUploadState {
        BlobUploadState {
            name: "alpine".to_string(),
            offset: 2811478,
            uuid: "2d6347b5-b8ec-497e-b3bd-ac2d0f483c7d".to_string(),
            started_at: NaiveDateTime::parse_from_str("2021-06-19T06:36:04.97859", DATATIME_FMT)
                .unwrap(),
        }
    }

    #[test]
    fn test_pack() {
        let packed = UploadStater(String::from(SECRET))
            .pack(raw_state())
            .unwrap();

        let unpacked = UploadStater(String::from(SECRET)).unpack(packed).unwrap();
        assert_eq!(unpacked, raw_state());
    }

    #[test]
    fn test_unpack() {
        let state = UploadStater(String::from(SECRET))
            .unpack(String::from(ENCODED))
            .unwrap();
        // println!("\nunpacked state: {:?}", state);

        assert_eq!(state, raw_state());
    }

    #[test]
    fn verify_hmac() {
        let mac = HmacSha256::new_from_slice(b"9fa65e8af8a0480ba88dcdca3d83184b");
        match mac {
            Ok(mut mac) => {
                mac.update(b"hello,world!");
                let result = mac.finalize().into_bytes().to_vec();
                // println!("{:X?}", result);
                assert_eq!(format!("{:x?}",result),"[d4, 37, de, 92, 55, f, a5, 5f, e2, bf, 9f, 5b, ac, 9a, d5, 15, ee, 4e, e0, cf, 83, 5a, 4a, a1, ea, a9, 51, 12, d5, 26, 58, 69]")
            }
            Err(e) => println!("verify_hmac failed: {}", e),
        }
    }
}
