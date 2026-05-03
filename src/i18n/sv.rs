use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Installationsprogram", app_name)),
    ("loading", "Laddar...".into()),
    ("license_title", "Licensavtal".into()),
    ("license_subtitle",
     "Läs följande viktiga information innan du fortsätter".into()),
    ("license_body",
     "Läs följande licensavtal noga. Du måste acceptera villkoren i detta \
     avtal innan du fortsätter med installationen.".into()),
    ("accept", "Acceptera".into()),
    ("decline", "Avböj".into()),
    ("downloading", "Laddar ner...".into()),
    ("verifying", "Verifierar...".into()),
    ("download_ok", "Nedladdning klar".into()),
    ("finished_title", "Installation klar".into()),
    ("failed_title", "Installation misslyckades".into()),
    ("close", "Stäng".into()),
    ("trying", "Ansluter till".into()),
    ("err_checksum", "Kontrollsummafel".into()),
    ("err_all_failed", "Alla servrar misslyckades".into()),
    ("err_no_length", "Okänd storlek".into()),
  ])
}
