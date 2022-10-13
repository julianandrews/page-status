use std::collections::BTreeMap;

use serde::Deserialize;

pub fn load(config_file: &std::path::Path) -> anyhow::Result<Config> {
    let config = toml::from_str(&std::fs::read_to_string(config_file)?)?;
    Ok(config)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub cache_dir: Option<std::path::PathBuf>,
    pub pages: BTreeMap<String, PageConfig>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PageConfig {
    pub url: String,
    #[serde(default)]
    pub method: Method,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    #[serde(default = "default_poll_interval")]
    pub poll_interval: u64,
    pub timeout: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Method {
    // TODO: Add more methods.
    Get,
    Head,
}

impl Method {
    pub fn reqwest_method(&self) -> reqwest::Method {
        match self {
            Method::Get => reqwest::Method::GET,
            Method::Head => reqwest::Method::HEAD,
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Self::Get
    }
}

fn default_poll_interval() -> u64 {
    60
}
