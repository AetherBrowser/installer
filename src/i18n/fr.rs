use std::collections::HashMap;

pub fn strings(app_name: &str) -> HashMap<&'static str, String> {
  HashMap::from([
    ("app_title", format!("Installateur {}", app_name)),
    ("loading", "Chargement...".into()),
    ("license_title", "Contrat de licence".into()),
    ("license_subtitle",
     "Veuillez lire les informations importantes avant de continuer".into()),
    ("license_body",
     "Veuillez lire attentivement le contrat de licence ci-dessous. Vous devez \
     accepter les termes de cet accord pour continuer l'installation.".into()),
    ("accept", "Accepter".into()),
    ("decline", "Refuser".into()),
    ("downloading", "Téléchargement...".into()),
    ("verifying", "Vérification...".into()), 
    ("download_ok", "Téléchargement terminé".into()),
    ("finished_title", "Installation terminée".into()),
    ("failed_title", "Installation échouée".into()),
    ("close", "Fermer".into()),
    ("trying", "Connexion à".into()),
    ("err_checksum", "Erreur de checksum".into()),
    ("err_all_failed", "Tous les serveurs ont échoué".into()),
    ("err_no_length", "Taille inconnue".into()),
  ])
}
