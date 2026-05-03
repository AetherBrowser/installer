use reqwest::blocking::Response;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{Read, Write},
    sync::{Arc, Mutex},
};

pub fn download_with_fallback(
    servers: Vec<&str>,
    path: &str,
    expected_hash: &str,
    progress: Arc<Mutex<f32>>,
    status: Arc<Mutex<String>>,
    str_trying: &str,
    str_verifying: &str,
    str_checksum: &str,
    str_all_fail: &str,
) -> Result<(), String> {
    for url in servers {
        *status.lock().unwrap() = format!("{} {}", str_trying, url);

        match download_file(url, path, progress.clone()) {
            Ok(_) => {
                *status.lock().unwrap() = str_verifying.into();

                if verify_sha256(path, expected_hash)? {
                    return Ok(());
                } else {
                    return Err(str_checksum.into());
                }
            }
            Err(e) => {
                *status.lock().unwrap() = format!("Failed: {}", e);
                continue;
            }
        }
    }

    Err(str_all_fail.into())
}

fn download_file(
    url: &str,
    path: &str,
    progress: Arc<Mutex<f32>>,
) -> Result<(), String> {
    let mut resp: Response = reqwest::blocking::get(url)
        .map_err(|e| e.to_string())?;

    let total_size = resp
        .content_length()
        .ok_or("No content length")?;

    let mut file = File::create(path).map_err(|e| e.to_string())?;

    let mut downloaded = 0u64;
    let mut buffer = [0u8; 8192];

    loop {
        let n = resp.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }

        file.write_all(&buffer[..n]).map_err(|e| e.to_string())?;
        downloaded += n as u64;

        *progress.lock().unwrap() = downloaded as f32 / total_size as f32;
    }

    Ok(())
}

fn verify_sha256(path: &str, expected: &str) -> Result<bool, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()) == expected)
}
