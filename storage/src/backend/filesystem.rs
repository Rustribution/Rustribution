extern crate fs_extra;

use crate::backend::BlobBackend;
use bytes::Bytes;
use fs_extra::file::{move_file, CopyOptions};
use slog::Logger;
use std::fs::{metadata, DirBuilder, File, OpenOptions};
use std::io::Result;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct Filesystem {
    logger: Logger,
    config: StorageFilesystemCfg,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StorageFilesystemCfg {
    pub rootdir: String,
}

impl BlobBackend for Filesystem {
    fn set_logger(&mut self, logger: Logger) {
        self.logger = logger;
    }

    fn info(&self) -> String {
        format!(
            "[Filesystem storage config] rootdir: {}",
            self.config.rootdir,
        )
    }

    fn stat(&self, path: String) -> (bool, usize) {
        let filepath = format!("{}/{}/data", self.config.rootdir, path);
        let md = metadata(filepath);
        match md {
            Ok(md) => (true, md.len() as usize),
            Err(e) => {
                error!(self.logger,"get file metadata failed";"error"=>e);
                (false, 0)
            }
        }
    }

    fn get_content(&self, path: String) -> Bytes {
        let mut file = File::open(format!("{}/{}/data", self.config.rootdir, path)).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        Bytes::from(data)
    }

    fn put_content(&mut self, path: String, data: Bytes) {
        let dirpath = format!("{}/{}", self.config.rootdir, path);
        DirBuilder::new().recursive(true).create(dirpath).unwrap();
        let filepath = format!("{}/{}/data", self.config.rootdir, path);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filepath.clone());
        debug!(self.logger,"write data"; "filepath"=>filepath.clone());
        file.unwrap().write_all(data.as_ref()).unwrap();
    }

    fn mov(&self, src_path: String, dst_path: String) {
        let dst_dir = format!("{}/{}", self.config.rootdir, dst_path);
        DirBuilder::new().recursive(true).create(dst_dir).unwrap();

        let src_file = format!("{}/{}/data", self.config.rootdir, src_path);
        let dst_file = format!("{}/{}/data", self.config.rootdir, dst_path);

        move_file(src_file, dst_file, &CopyOptions::new()).unwrap();
    }

    fn delete(&self, path: String) {
        let file = format!("{}/{}", self.config.rootdir, path);
        let (exist, _) = self.stat(path.clone());
        warn!(
            self.logger,
            "delete blob";
            "file"=>file.clone(),
            "exist"=>exist.clone(),
        );
        if exist {
            std::fs::remove_dir_all(file).unwrap();
        }
    }
}

pub fn new(config: toml::value::Value) -> Result<Filesystem> {
    let logger = slog_scope::logger();
    info!(logger, "storage config: {:?}", config["filesystem"]);
    let config: StorageFilesystemCfg = toml::from_str(
        toml::to_string(&config["filesystem"].as_table())
            .unwrap()
            .as_str(),
    )
    .unwrap();
    Ok(Filesystem { config, logger })
}
