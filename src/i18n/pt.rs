use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("Instalador {}", app_name)),
    ("loading", "Carregando...".into()),
    ("license_title", "Acordo de licença".into()),
    ("license_subtitle",
     "Por favor, leia as informações importantes antes de continuar".into()),
    ("license_body",
     "Leia atentamente o seguinte acordo de licença. Você deve aceitar os \
     termos deste acordo antes de continuar com a instalação.".into()),
    ("accept", "Aceitar".into()),
    ("decline", "Recusar".into()),
    ("downloading", "Baixando...".into()),
    ("verifying", "Verificando...".into()),
    ("download_ok", "Download completo".into()),
    ("finished_title", "Instalação concluída".into()),
    ("failed_title", "Instalação falhou".into()),
    ("close", "Fechar".into()),
    ("trying", "Conectando a".into()),
    ("err_checksum", "Erro de checksum".into()),
    ("err_all_failed", "Todos os servidores falharam".into()),
    ("err_no_length", "Tamanho desconhecido".into()),
  ])
}
