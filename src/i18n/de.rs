use std::collections::HashMap;

pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Installer", app_name)),
    ("loading", "Laden...".into()),
    ("license_title", "Lizenzvereinbarung".into()),
    ("license_subtitle",
     "Bitte lesen Sie die folgenden wichtigen Informationen".into()),
    ("license_body",
     "Bitte lesen Sie die folgende Lizenzvereinbarung sorgfältig. Sie müssen \
     den Bedingungen zustimmen, um die Installation fortzusetzen.".into()),
    ("accept", "Akzeptieren".into()),
    ("decline", "Ablehnen".into()),
    ("downloading", "Herunterladen...".into()),
    ("verifying", "Überprüfen...".into()),
    ("download_ok", "Download abgeschlossen".into()),
    ("finished_title", "Installation abgeschlossen".into()),
    ("failed_title", "Installation fehlgeschlagen".into()),
    ("close", "Schließen".into()),
    ("trying", "Verbinde mit".into()),
    ("err_checksum", "Prüfsummenfehler".into()),
    ("err_all_failed", "Alle Server fehlgeschlagen".into()),
    ("err_no_length", "Unbekannte Größe".into()),
  ])
}
