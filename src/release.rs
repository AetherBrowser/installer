use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config_download::{Config, DownloadInfo, Release};

#[derive(Clone)]
pub struct ReleaseRecap {
    pub app: String,
    pub version: String,
    pub description: String,
    pub released_at: String,
    pub install_dir: PathBuf,
    pub url: String,
    pub sha256: String,
}

pub fn default_install_dir() -> Result<PathBuf, String> {
    let data_dir = dirs::data_local_dir().ok_or("Cannot find data dir")?;
    Ok(data_dir.join("appxxxx"))
}

pub fn from_config(config: &Config) -> Result<ReleaseRecap, String> {
    let latest = config
        .channels
        .stable
        .history
        .first()
        .ok_or("No releases")?;

    let download = resolve_download(latest)?;

    Ok(ReleaseRecap {
        app: config.app.clone(),
        version: latest.version.clone(),
        description: latest.description.clone(),
        released_at: latest.released_at.clone(),
        install_dir: default_install_dir()?,
        url: download.url.clone(),
        sha256: download.sha256.clone(),
    })
}

fn resolve_download(release: &Release) -> Result<DownloadInfo, String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let platform = match os {
        "windows" => release.platforms.windows.as_ref(),
        "linux" => release.platforms.linux.as_ref(),
        "macos" => release.platforms.macos.as_ref(),
        _ => None,
    }
    .ok_or("Unsupported OS")?;

    match arch {
        "x86" | "x86_64" => platform.x86.clone(),
        "arm" | "aarch64" => platform.arm.clone(),
        "riscv64" => platform.riscv.clone(),
        _ => None,
    }
    .ok_or_else(|| "Unsupported architecture".to_string())
}

pub fn launch_app(install_dir: &Path) -> Result<(), String> {
    let executable = find_executable(install_dir)?;
    Command::new(&executable)
        .current_dir(install_dir)
        .spawn()
        .map_err(|e| format!("Failed to launch {}: {}", executable.display(), e))?;
    Ok(())
}

fn find_executable(install_dir: &Path) -> Result<PathBuf, String> {
    let names: &[&str] = if cfg!(windows) {
        &["app.exe", "app"]
    } else {
        &["app"]
    };

    for name in names {
        let path = install_dir.join(name);
        if path.is_file() {
            return Ok(path);
        }
    }

    Err(format!(
        "No executable found in {}",
        install_dir.display()
    ))
}
