use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("Programma di installazione {}", app_name)),
    ("loading", "Caricamento...".into()),
    ("license_title", "Contratto di licenza".into()),
    ("license_subtitle", "Si prega di leggere le seguenti informazioni \
                         importanti prima di continuare".into()),
    ("license_body", "Si prega di leggere attentamente il seguente contratto \
                     di licenza. È necessario accettare i termini di questo \
                     contratto prima di procedere con l'installazione.".into()),
    ("accept", "Accetta".into()),
    ("decline", "Rifiuta".into()),
    ("downloading", "Download in corso...".into()),
    ("verifying", "Verifica in corso...".into()),
    ("download_ok", "Download completato".into()),
    ("finished_title", "Installazione completata".into()),
    ("failed_title", "Installazione non riuscita".into()),
    ("close", "Chiudi".into()),
    ("trying", "Connessione a".into()),
    ("err_checksum", "Errore checksum".into()),
    ("err_all_failed", "Tutti i server hanno fallito".into()),
    ("err_no_length", "Dimensione sconosciuta".into()),
  ])
}
