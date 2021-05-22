use derive_more::From;
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Read,
};

const CONFIG_FILE_NAME: &str = "media-server-1.toml";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ConfigRaw {
    general: ConfigGeneral,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ConfigGeneral {
    #[serde(rename = "base-dir")]
    base_dir: String,
    #[serde(rename = "exclude-patterns", default)]
    exclude_patterns: Vec<String>,
    #[serde(default = "default_bindings")]
    bindings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub base_dir: String,
    pub exclude_patterns: RegexSet,
    pub bindings: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Config, ConfigLoadError> {
        let mut cfg_file = File::open(CONFIG_FILE_NAME)?;
        let mut cfg_string = String::new();
        cfg_file.read_to_string(&mut cfg_string)?;

        let cfg_raw: ConfigRaw = toml::from_str(&cfg_string)?;

        Ok(Config {
            base_dir: cfg_raw.general.base_dir,
            exclude_patterns: RegexSet::new(cfg_raw.general.exclude_patterns)?,
            bindings: cfg_raw.general.bindings,
        })
    }
}

#[derive(Debug, From)]
pub enum ConfigLoadError {
    IOError(std::io::Error),
    DeserializeError(toml::de::Error),
    RegexError(regex::Error),
}

impl Display for ConfigLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigLoadError::IOError(e) => {
                write!(
                    f,
                    "Unable to open config file: '{}': {}",
                    CONFIG_FILE_NAME, e
                )
            }
            ConfigLoadError::DeserializeError(_) => write!(f, "Unable to parse config file."),
            ConfigLoadError::RegexError(_) => write!(f, "Regex error in config file."),
        }
    }
}

fn default_bindings() -> Vec<String> {
    return vec!["127.0.0.1:9090".to_string()];
}
