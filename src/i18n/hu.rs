use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Telepítő", app_name)),
    ("loading", "Betöltés...".into()),
    ("license_title", "Licencszerződés".into()),
    ("license_subtitle",
     "Kérjük, olvassa el a következő fontos információkat a folytatás előtt".into()),
    ("license_body",
     "Kérjük, olvassa el figyelmesen a következő licencszerződést. A telepítés \
     folytatásához el kell fogadnia a szerződés feltételeit.".into()),
    ("accept", "Elfogad".into()),
    ("decline", "Elutasít".into()),
    ("downloading", "Letöltés...".into()),
    ("verifying", "Ellenőrzés...".into()),
    ("download_ok", "Letöltés befejezve".into()),
    ("finished_title", "Telepítés befejezve".into()),
    ("failed_title", "Telepítés sikertelen".into()),
    ("close", "Bezár".into()),
    ("trying", "Csatlakozás".into()),
    ("err_checksum", "Ellenőrzőösszeg hiba".into()),
    ("err_all_failed", "Minden szerver sikertelen".into()),
    ("err_no_length", "Ismeretlen méret".into()),
  ])
}
