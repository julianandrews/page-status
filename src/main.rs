mod config;
mod page;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

use page::Page;

static DEFAULT_CONFIG_FILE: &str = concat!("/etc/", env!("CARGO_BIN_NAME"), "/config.toml");
static DEFAULT_CACHE_DIR: &str = concat!("/var/cache/", env!("CARGO_BIN_NAME"));

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config_file = args
        .config_file
        .unwrap_or_else(|| PathBuf::from(&DEFAULT_CONFIG_FILE));
    let config = config::load(&config_file).context("Failed to load config")?;
    let cache_dir = args
        .cache_dir
        .or(config.cache_dir)
        .unwrap_or_else(|| PathBuf::from(&DEFAULT_CACHE_DIR));
    std::fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;
    let pages: Vec<_> = config
        .pages
        .into_iter()
        .map(|(name, conf)| Page::from_config(conf, cache_dir.join(name)))
        .collect();

    futures::future::join_all(pages.iter().map(|page| page.poll())).await;
    Ok(())
}

#[derive(Parser, Debug, Clone)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Path to cache directory
    #[clap(long, global(true))]
    pub cache_dir: Option<std::path::PathBuf>,

    /// Path to config file
    #[clap(long)]
    pub config_file: Option<std::path::PathBuf>,
}
