use std::collections::HashMap;

pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Installer", app_name)),
    ("loading", "Loading...".into()),
    ("license_title", "License Agreement".into()),
    ("license_subtitle",
     "Please read the following important information before continuing".into()),
    ("license_body",
     "Please read the following license agreement carefully. You must accept \
     the terms of this agreement before continuing with the installation.".into()),
    ("accept", "Accept".into()),
    ("decline", "Decline".into()),
    ("downloading", "Downloading...".into()),
    ("verifying", "Verifying...".into()),
    ("download_ok", "Download complete".into()),
    ("finished_title", "Installation Finished".into()),
    ("failed_title", "Installation Failed".into()),
    ("close", "Close".into()),
    ("trying", "Trying".into()),
    ("err_checksum", "Checksum mismatch".into()),
    ("err_all_failed", "All servers failed".into()),
    ("err_no_length", "No content length".into()),
  ])
}
