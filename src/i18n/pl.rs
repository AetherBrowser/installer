use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("Instalator {}", app_name)),
    ("loading", "Ładowanie...".into()),
    ("license_title", "Umowa licencyjna".into()),
    ("license_subtitle",
     "Przeczytaj poniższe ważne informacje przed kontynuowaniem".into()),
    ("license_body",
     "Uważnie przeczytaj poniższą umowę licencyjną. Musisz zaakceptować \
     warunki tej umowy przed kontynuowaniem instalacji.".into()),
    ("accept", "Akceptuj".into()),
    ("decline", "Odrzuć".into()),
    ("downloading", "Pobieranie...".into()),
    ("verifying", "Weryfikacja...".into()),
    ("download_ok", "Pobieranie zakończone".into()),
    ("finished_title", "Instalacja zakończona".into()),
    ("failed_title", "Instalacja nie powiodła się".into()),
    ("close", "Zamknij".into()),
    ("trying", "Łączenie z".into()),
    ("err_checksum", "Błąd sumy kontrolnej".into()),
    ("err_all_failed", "Wszystkie serwery zawiodły".into()),
    ("err_no_length", "Nieznany rozmiar".into()),
  ])
}
