use serde::Deserialize;
use crate::helper::{load_config};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app: String,
    pub channels: Channels,
}

#[derive(Debug, Deserialize)]
pub struct Channels {
    pub stable: Channel,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub history: Vec<Release>,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub version: String,
    pub description: String,
    pub released_at: String,
    pub platforms: Platforms,
}

#[derive(Debug, Deserialize)]
pub struct Platforms {
    pub windows: Option<PlatformArch>,
    pub linux: Option<PlatformArch>,
    pub macos: Option<PlatformArch>,
}

#[derive(Debug, Deserialize)]
pub struct PlatformArch {
    pub x86: Option<DownloadInfo>,
    pub arm: Option<DownloadInfo>,
    pub riscv: Option<DownloadInfo>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DownloadInfo {
    pub url: String,
    pub sha256: String,
}

pub fn fetch_config() -> Result<Config, String> {
    let config = load_config();
    let json_config_url = config.json_config_url;

    reqwest::blocking::get(json_config_url)
        .map_err(|e| e.to_string())?
        .json::<Config>()
        .map_err(|e| e.to_string())
}
