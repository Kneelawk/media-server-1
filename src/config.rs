use crate::error::{ErrorKind::ConfigLoadError, Result, ResultExt};
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

const CONFIG_FILE_NAME: &str = "media-server-1.toml";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ConfigRaw {
    #[serde(default)]
    general: ConfigGeneral,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ConfigGeneral {
    #[serde(rename = "base-dir", default = "default_base_dir")]
    base_dir: String,
    #[serde(rename = "exclude-patterns", default)]
    exclude_patterns: Vec<String>,
    #[serde(default = "default_bindings")]
    bindings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub base_dir: PathBuf,
    pub exclude_patterns: RegexSet,
    pub bindings: Vec<String>,
}

impl Default for ConfigGeneral {
    fn default() -> Self {
        ConfigGeneral {
            base_dir: default_base_dir(),
            exclude_patterns: Default::default(),
            bindings: default_bindings(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Config> {
        info!("Loading config: {}", CONFIG_FILE_NAME);

        let cfg_path = Path::new(CONFIG_FILE_NAME);

        let cfg_raw: ConfigRaw = if cfg_path.exists() {
            let mut cfg_file = File::open(CONFIG_FILE_NAME)
                .chain_err(|| ConfigLoadError("Error opening config file".into()))?;
            let mut cfg_string = String::new();
            cfg_file
                .read_to_string(&mut cfg_string)
                .chain_err(|| ConfigLoadError("Error reading config file".into()))?;

            toml::from_str(&cfg_string)
                .chain_err(|| ConfigLoadError("Error decoding config file".into()))?
        } else {
            toml::from_str("").chain_err(|| ConfigLoadError("Error loading blank config".into()))?
        };

        debug!("Writing config file...");
        let new_cfg_string = toml::to_string_pretty(&cfg_raw)
            .chain_err(|| ConfigLoadError("Error re-encoding config file".into()))?;
        let mut cfg_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(cfg_path)
            .chain_err(|| ConfigLoadError("Error opening config file for re-writing".into()))?;
        cfg_file
            .write_all(new_cfg_string.as_bytes())
            .chain_err(|| ConfigLoadError("Error re-writing config file".into()))?;

        Ok(Config {
            base_dir: cfg_raw.general.base_dir.into(),
            exclude_patterns: RegexSet::new(cfg_raw.general.exclude_patterns)
                .chain_err(|| ConfigLoadError("Error parsing regex".into()))?,
            bindings: cfg_raw.general.bindings,
        })
    }
}

fn default_base_dir() -> String {
    match dirs::video_dir() {
        None => match dirs::home_dir() {
            None => "~/Videos/".to_string(),
            Some(home) => {
                let mut dir = home.clone();
                dir.push("Videos");
                dir.to_string_lossy().to_string()
            }
        },
        Some(dir) => dir.to_string_lossy().to_string(),
    }
}

fn default_bindings() -> Vec<String> {
    return vec!["127.0.0.1:9090".to_string()];
}
