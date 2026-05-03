use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("Instalador {}", app_name)),
    ("loading", "Cargando...".into()),
    ("license_title", "Acuerdo de licencia".into()),
    ("license_subtitle",
     "Lea la siguiente información importante antes de continuar".into()),
    ("license_body",
     "Lea atentamente el siguiente acuerdo de licencia. Debe aceptar los \
     términos de este acuerdo antes de continuar con la instalación.".into()),
    ("accept", "Aceptar".into()),
    ("decline", "Rechazar".into()),
    ("downloading", "Descargando...".into()),
    ("verifying", "Verificando...".into()),
    ("download_ok", "Descarga completa".into()),
    ("finished_title", "Instalación completada".into()),
    ("failed_title", "Instalación fallida".into()),
    ("close", "Cerrar".into()),
    ("trying", "Conectando a".into()),
    ("err_checksum", "Error de suma de verificación".into()),
    ("err_all_failed", "Todos los servidores fallaron".into()),
    ("err_no_length", "Tamaño desconocido".into()),
  ])
}
