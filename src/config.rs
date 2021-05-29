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
    #[serde(rename = "exclude-patterns", default = "default_exclude_patterns")]
    exclude_patterns: Vec<String>,
    #[serde(default = "default_bindings")]
    bindings: Vec<String>,
    #[serde(rename = "welcome-title", default = "default_welcome_title")]
    welcome_title: String,
    #[serde(rename = "welcome-content", default = "default_welcome_content")]
    welcome_content: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub base_dir: PathBuf,
    pub exclude_patterns: RegexSet,
    pub bindings: Vec<String>,
    pub welcome_title: String,
    pub welcome_content: String,
}

impl Default for ConfigGeneral {
    fn default() -> Self {
        ConfigGeneral {
            base_dir: default_base_dir(),
            exclude_patterns: default_exclude_patterns(),
            bindings: default_bindings(),
            welcome_title: default_welcome_title(),
            welcome_content: default_welcome_content(),
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
            welcome_title: cfg_raw.general.welcome_title,
            welcome_content: cfg_raw.general.welcome_content,
        })
    }

    pub fn is_legal_path(&self, path: &str) -> bool {
        !self.exclude_patterns.is_match(path)
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

fn default_exclude_patterns() -> Vec<String> {
    vec![
        "^.env".to_owned(),
        "^logs".to_owned(),
        "^media-server-1$".to_owned(),
        "\\.jar$".to_owned(),
        "\\.json$".to_owned(),
        "\\.toml$".to_owned(),
        "\\.ya?ml$".to_owned(),
    ]
}

fn default_bindings() -> Vec<String> {
    vec!["127.0.0.1:9090".to_owned()]
}

fn default_welcome_title() -> String {
    "Media Server 1".to_string()
}

fn default_welcome_content() -> String {
    r#"<p>
Welcome! This media server seems to be working correctly.
</p>
<p>
This is a sample welcome text. To configure your own, open the <code>media-server-1.toml</code>
file next to your media-server-1 application and edit the <code>welcome-title</code> and
<code>welcome-content</code> properties.
"#
        .to_string()
}
