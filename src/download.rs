use reqwest::blocking::Response;
use sha2::{Digest, Sha256};
use zip::ZipArchive;

use std::{
    fs,
    io::{Read, Write},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use crate::release::ReleaseRecap;

const PROGRESS_DOWNLOAD: f32 = 0.65;
const PROGRESS_VERIFY: f32 = 0.85;

pub fn download_and_install(
    recap: &ReleaseRecap,
    progress: Arc<Mutex<f32>>,
    status: Arc<Mutex<String>>,
    cancelled: Arc<AtomicBool>,
    str_trying: &str,
    str_verifying: &str,
    str_installing: &str,
    err_cancelled: &str,
) -> Result<(), String> {
    let path = std::env::temp_dir().join("aether_installer");

    check_cancelled(&cancelled, err_cancelled)?;

    *status.lock().unwrap() = format!("{} {}", str_trying, recap.url);

    download_file(&recap.url, &path, progress.clone(), cancelled.clone(), err_cancelled)?;

    check_cancelled(&cancelled, err_cancelled)?;

    *status.lock().unwrap() = str_verifying.into();
    *progress.lock().unwrap() = PROGRESS_DOWNLOAD;

    verify_sha256(&path, &recap.sha256, cancelled.clone(), err_cancelled)?
        .then_some(())
        .ok_or("Checksum mismatch")?;

    check_cancelled(&cancelled, err_cancelled)?;

    *progress.lock().unwrap() = PROGRESS_VERIFY;
    *status.lock().unwrap() = str_installing.into();

    extract_zip(
        &path,
        recap.install_dir.as_path(),
        cancelled.clone(),
        err_cancelled,
    )?;

    let _ = fs::remove_file(&path);

    Ok(())
}

fn check_cancelled(cancelled: &AtomicBool, err_cancelled: &str) -> Result<(), String> {
    if cancelled.load(Ordering::Relaxed) {
        Err(err_cancelled.to_string())
    } else {
        Ok(())
    }
}

fn download_file(
    url: &str,
    path: &Path,
    progress: Arc<Mutex<f32>>,
    cancelled: Arc<AtomicBool>,
    err_cancelled: &str,
) -> Result<(), String> {
    let mut resp: Response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;

    let total_size = resp.content_length().ok_or("No content length")?;

    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;

    let mut downloaded = 0u64;
    let mut buffer = [0u8; 8192];

    loop {
        check_cancelled(&cancelled, err_cancelled)?;

        let n = resp.read(&mut buffer).map_err(|e| e.to_string())?;

        if n == 0 {
            break;
        }

        file.write_all(&buffer[..n]).map_err(|e| e.to_string())?;

        downloaded += n as u64;

        *progress.lock().unwrap() =
            (downloaded as f32 / total_size as f32) * PROGRESS_DOWNLOAD;
    }

    Ok(())
}

fn verify_sha256(
    path: &Path,
    expected: &str,
    cancelled: Arc<AtomicBool>,
    err_cancelled: &str,
) -> Result<bool, String> {
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        check_cancelled(&cancelled, err_cancelled)?;

        let n = file.read(&mut buffer).map_err(|e| e.to_string())?;

        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    let actual = format!("{:x}", hasher.finalize());
    println!("actual: {}", actual);
    println!("expected: {}", expected.trim());
    Ok(actual.eq_ignore_ascii_case(expected.trim()))
}

fn extract_zip(
    zip: &Path,
    output_dir: &Path,
    cancelled: Arc<AtomicBool>,
    err_cancelled: &str,
) -> Result<(), String> {
    let file = fs::File::open(zip).map_err(|e| e.to_string())?;

    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        check_cancelled(&cancelled, err_cancelled)?;

        let mut zipped = archive.by_index(i).map_err(|e| e.to_string())?;

        let outpath = Path::new(output_dir).join(zipped.name());

        if zipped.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = outpath.parent()
                && !parent.exists() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }

            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;

            std::io::copy(&mut zipped, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
