use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Installatieprogramma", app_name)),
    ("loading", "Laden...".into()),
    ("license_title", "Licentieovereenkomst".into()),
    ("license_subtitle",
     "Lees de volgende belangrijke informatie voordat u doorgaat".into()),
    ("license_body",
     "Lees de volgende licentieovereenkomst zorgvuldig. U moet akkoord gaan \
     met de voorwaarden voordat u de installatie kunt voortzetten.".into()),
    ("accept", "Accepteren".into()),
    ("decline", "Weigeren".into()),
    ("downloading", "Downloaden...".into()),
    ("verifying", "Verifiëren...".into()),
    ("download_ok", "Download voltooid".into()),
    ("finished_title", "Installatie voltooid".into()),
    ("failed_title", "Installatie mislukt".into()),
    ("close", "Sluiten".into()),
    ("trying", "Verbinden met".into()),
    ("err_checksum", "Checksum fout".into()),
    ("err_all_failed", "Alle servers mislukt".into()),
    ("err_no_length", "Onbekende grootte".into()),
  ])
}
