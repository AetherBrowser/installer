use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Installasjonsprogram", app_name)),
    ("loading", "Laster...".into()),
    ("license_title", "Lisensavtale".into()),
    ("license_subtitle",
     "Vennligst les følgende viktige informasjon før du fortsetter".into()),
    ("license_body", "Les følgende lisensavtale nøye. Du må godta vilkårene i \
                     denne avtalen før du fortsetter med installasjonen.".into()),
    ("accept", "Godta".into()),
    ("decline", "Avslå".into()),
    ("downloading", "Laster ned...".into()),
    ("verifying", "Verifiserer...".into()),
    ("download_ok", "Nedlasting fullført".into()),
    ("finished_title", "Installasjon fullført".into()),
    ("failed_title", "Installasjon mislyktes".into()),
    ("close", "Lukk".into()),
    ("trying", "Kobler til".into()),
    ("err_checksum", "Kontrollsummefeil".into()),
    ("err_all_failed", "Alle servere mislyktes".into()),
    ("err_no_length", "Ukjent størrelse".into()),
  ])
}
