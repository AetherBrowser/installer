use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Asennusohjelma", app_name)),
    ("loading", "Ladataan...".into()),
    ("license_title", "Lisenssisopimus".into()),
    ("license_subtitle", "Lue seuraavat tärkeät tiedot ennen jatkamista".into()),
    ("license_body",
     "Lue seuraava lisenssisopimus huolellisesti. Sinun on hyväksyttävä tämän \
     sopimuksen ehdot ennen asennuksen jatkamista.".into()),
    ("accept", "Hyväksy".into()),
    ("decline", "Hylkää".into()),
    ("downloading", "Ladataan...".into()),
    ("verifying", "Tarkistetaan...".into()),
    ("download_ok", "Lataus valmis".into()),
    ("finished_title", "Asennus valmis".into()),
    ("failed_title", "Asennus epäonnistui".into()),
    ("close", "Sulje".into()),
    ("trying", "Yhdistetään".into()),
    ("err_checksum", "Tarkistussummavirhe".into()),
    ("err_all_failed", "Kaikki palvelimet epäonnistuivat".into()),
    ("err_no_length", "Tuntematon koko".into()),
  ])
}
