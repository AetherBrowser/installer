use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("Instalátor {}", app_name)),
    ("loading", "Načítání...".into()),
    ("license_title", "Licenční smlouva".into()),
    ("license_subtitle",
     "Před pokračováním si přečtěte následující důležité informace".into()),
    ("license_body",
     "Pečlivě si přečtěte následující licenční smlouvu. Před pokračováním v \
     instalaci musíte přijmout podmínky této smlouvy.".into()),
    ("accept", "Přijmout".into()),
    ("decline", "Odmítnout".into()),
    ("downloading", "Stahování...".into()),
    ("verifying", "Ověřování...".into()),
    ("download_ok", "Stahování dokončeno".into()),
    ("finished_title", "Instalace dokončena".into()),
    ("failed_title", "Instalace selhala".into()),
    ("close", "Zavřít".into()),
    ("trying", "Připojování k".into()),
    ("err_checksum", "Chyba kontrolního součtu".into()),
    ("err_all_failed", "Všechny servery selhaly".into()),
    ("err_no_length", "Neznámá velikost".into()),
  ])
}
