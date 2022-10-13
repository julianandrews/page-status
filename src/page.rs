use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};

use crate::config::PageConfig;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct Page {
    request_builder: reqwest::RequestBuilder,
    poll_interval: Duration,
    cache_file: PathBuf,
}

impl Page {
    pub fn from_config(conf: PageConfig, cache_file: PathBuf) -> Self {
        let client = reqwest::Client::new();
        let mut request_builder = client.request(conf.method.reqwest_method(), &conf.url);
        request_builder = request_builder.header("user-agent", USER_AGENT);
        for (key, value) in &conf.headers {
            request_builder = request_builder.header(key, value);
        }
        if let Some(timeout) = conf.timeout {
            request_builder = request_builder.timeout(Duration::from_secs(timeout));
        }
        let poll_interval = std::time::Duration::from_secs(conf.poll_interval);

        Self {
            request_builder,
            poll_interval,
            cache_file,
        }
    }

    pub async fn poll(&self) {
        let mut interval = tokio::time::interval(self.poll_interval);
        loop {
            interval.tick().await;
            if let Err(error) = self.check().await {
                let name = self
                    .cache_file
                    .file_name()
                    .expect("cache file should have name");
                eprintln!("Error checking {:?}: {:?}", name, error);
            }
        }
    }

    async fn check(&self) -> Result<()> {
        let response = self
            .request_builder
            .try_clone()
            .expect("unused request should be cloneable")
            .send()
            .await
            .context("request failed");

        match response {
            Ok(response) => {
                std::fs::write(&self.cache_file, response.status().as_str())
                    .context("failed to write to cache")?;
                Ok(())
            }
            Err(error) => {
                std::fs::write(&self.cache_file, format!("{:?}", error))
                    .context("failed to write to cache")?;
                Err(error)
            }
        }
    }
}
