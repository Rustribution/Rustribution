extern crate fs_extra;

use crate::backend::BlobBackend;
use bytes::Bytes;
use fs_extra::dir::remove as rmdir;
use fs_extra::file::{move_file, CopyOptions};
use slog::Logger;
use std::fs::{metadata, DirBuilder, File, OpenOptions};
use std::io::{Read, Result, Write};

#[derive(Debug, Clone)]
pub struct Filesystem {
    logger: Logger,
    config: StorageFilesystemCfg,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StorageFilesystemCfg {
    pub rootdir: String,
}

impl Filesystem {
    pub fn new(config: toml::value::Value, logger: Logger) -> Self {
        info!(logger, "storage config: {:?}", config["filesystem"]);
        let config: StorageFilesystemCfg = toml::from_str(
            toml::to_string(&config["filesystem"].as_table())
                .unwrap()
                .as_str(),
        )
        .unwrap();
        Filesystem { config, logger }
    }
}

impl BlobBackend for Filesystem {
    fn info(&self) -> String {
        format!(
            "[Filesystem storage config] rootdir: {}",
            self.config.rootdir,
        )
    }

    fn stat(&self, path: String) -> Result<usize> {
        let filepath = format!("{}{}/data", self.config.rootdir, path);
        let md = metadata(filepath);
        match md {
            Ok(md) => Ok(md.len() as usize),
            Err(e) => Err(e),
        }
    }

    fn get_content(&self, path: String) -> Result<Bytes> {
        match self.stat(path.clone()) {
            Ok(_) => {
                let mut file =
                    File::open(format!("{}/{}/data", self.config.rootdir, path)).unwrap();
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();
                Ok(Bytes::from(data))
            }
            Err(e) => Err(e),
        }
    }

    fn put_content(&self, path: String, data: Bytes) {
        let dirpath = format!("{}{}", self.config.rootdir, path);
        DirBuilder::new().recursive(true).create(dirpath).unwrap();
        let filepath = format!("{}{}/data", self.config.rootdir, path);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filepath.clone());
        debug!(self.logger,"write data"; "filepath"=>filepath.clone());
        file.unwrap().write_all(data.as_ref()).unwrap();
    }

    fn mov(&self, src_path: String, dst_path: String) -> Result<()> {
        let src_dir = format!("{}{}", self.config.rootdir, src_path);
        let dst_dir = format!("{}{}", self.config.rootdir, dst_path);
        DirBuilder::new().recursive(true).create(dst_dir)?;

        let src_file = format!("{}/data", src_dir);
        let dst_file = format!("{}{}/data", self.config.rootdir, dst_path);

        move_file(src_file, dst_file, &CopyOptions::new()).unwrap_or(0);
        rmdir(src_dir).unwrap();
        Ok(())
    }

    fn delete(&self, path: String) -> Result<()> {
        let file = format!("{}/{}/data", self.config.rootdir, path);
        match self.stat(path.clone()) {
            Ok(size) => {
                warn!(
                    self.logger,
                    "delete blob success";
                    "size"=>size,
                    "path"=>&path,
                    "file"=>&file,
                );
                std::fs::remove_dir_all(file)
            }
            Err(e) => {
                error!(
                    self.logger,
                    "delete blob failed";
                    "path"=>&path,
                    "file"=>&file,
                    "error"=>&e,
                );

                Err(e)
            }
        }
    }
}
