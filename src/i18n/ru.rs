use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("Установщик {}", app_name)),
    ("loading", "Загрузка...".into()),
    ("license_title", "Лицензионное соглашение".into()),
    ("license_subtitle",
     "Пожалуйста, прочитайте следующую важную информацию перед продолжением".into()),
    ("license_body",
     "Внимательно прочитайте следующее лицензионное соглашение. Вы должны \
     принять условия этого соглашения, прежде чем продолжить установку.".into()),
    ("accept", "Принять".into()),
    ("decline", "Отклонить".into()),
    ("downloading", "Загрузка...".into()),
    ("verifying", "Проверка...".into()),
    ("download_ok", "Загрузка завершена".into()),
    ("finished_title", "Установка завершена".into()),
    ("failed_title", "Установка не удалась".into()),
    ("close", "Закрыть".into()),
    ("trying", "Подключение к".into()),
    ("err_checksum", "Ошибка контрольной суммы".into()),
    ("err_all_failed", "Все серверы недоступны".into()),
    ("err_no_length", "Неизвестный размер".into()),
  ])
}
