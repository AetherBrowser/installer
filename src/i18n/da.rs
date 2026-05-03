use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Installationsprogram", app_name)),
    ("loading", "Indlæser...".into()),
    ("license_title", "Licensaftale".into()),
    ("license_subtitle",
     "Læs venligst følgende vigtige oplysninger inden du fortsætter".into()),
    ("license_body",
     "Læs venligst følgende licensaftale omhyggeligt. Du skal acceptere \
     vilkårene i denne aftale, før du fortsætter med installationen.".into()),
    ("accept", "Acceptér".into()),
    ("decline", "Afvis".into()),
    ("downloading", "Downloader...".into()),
    ("verifying", "Verificerer...".into()),
    ("download_ok", "Download fuldført".into()),
    ("finished_title", "Installation fuldført".into()),
    ("failed_title", "Installation mislykkedes".into()),
    ("close", "Luk".into()),
    ("trying", "Opretter forbindelse til".into()),
    ("err_checksum", "Kontrolsummefejl".into()),
    ("err_all_failed", "Alle servere mislykkedes".into()),
    ("err_no_length", "Ukendt størrelse".into()),
  ])
}
