mod fancy_file;
#[cfg(feature = "ffmpeg")]
mod ffmpeg;

use crate::logging::fancy_file::FancyFileAppenderDeserializer;
use log4rs::config::Deserializers;
use std::{fs::OpenOptions, io::Write, path::Path};

#[cfg(feature = "ffmpeg")]
pub use ffmpeg::log_callback;

const DEFAULT_CONFIG_FILE: &str = "media-server-1.log4rs.yaml";
const DEFAULT_CONFIG: &[u8] = include_bytes!("default.log4rs.yaml");

pub fn init() {
    let config_file_path = Path::new(DEFAULT_CONFIG_FILE);
    if !config_file_path.exists() {
        let mut write_cfg_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_file_path)
            .unwrap();
        write_cfg_file.write_all(DEFAULT_CONFIG).unwrap();
    }

    let mut deserializers = Deserializers::new();
    deserializers.insert("fancy_file", FancyFileAppenderDeserializer);

    log4rs::init_file(config_file_path, deserializers).unwrap();
}
