use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("Programul de instalare {}", app_name)),
    ("loading", "Se încarcă...".into()),
    ("license_title", "Acord de licență".into()),
    ("license_subtitle", "Vă rugăm să citiți următoarele informații importante \
                         înainte de a continua".into()),
    ("license_body",
     "Vă rugăm să citiți cu atenție următorul acord de licență. Trebuie să \
     acceptați termenii acestui acord înainte de a continua instalarea.".into()),
    ("accept", "Accept".into()),
    ("decline", "Refuz".into()),
    ("downloading", "Se descarcă...".into()),
    ("verifying", "Se verifică...".into()),
    ("download_ok", "Descărcare completă".into()),
    ("finished_title", "Instalare finalizată".into()),
    ("failed_title", "Instalare eșuată".into()),
    ("close", "Închide".into()),
    ("trying", "Conectare la".into()),
    ("err_checksum", "Eroare sumă de control".into()),
    ("err_all_failed", "Toate serverele au eșuat".into()),
    ("err_no_length", "Dimensiune necunoscută".into()),
  ])
}
