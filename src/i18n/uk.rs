use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("Інсталятор {}", app_name)),
    ("loading", "Завантаження...".into()),
    ("license_title", "Ліцензійна угода".into()),
    ("license_subtitle",
     "Будь ласка, прочитайте наступну важливу інформацію перед продовженням".into()),
    ("license_body",
     "Уважно прочитайте наступну ліцензійну угоду. Ви повинні прийняти умови \
     цієї угоди, перш ніж продовжити встановлення.".into()),
    ("accept", "Прийняти".into()),
    ("decline", "Відхилити".into()),
    ("downloading", "Завантаження...".into()),
    ("verifying", "Перевірка...".into()),
    ("download_ok", "Завантаження завершено".into()),
    ("finished_title", "Встановлення завершено".into()),
    ("failed_title", "Встановлення не вдалося".into()),
    ("close", "Закрити".into()),
    ("trying", "Підключення до".into()),
    ("err_checksum", "Помилка контрольної суми".into()),
    ("err_all_failed", "Всі сервери недоступні".into()),
    ("err_no_length", "Невідомий розмір".into()),
  ])
}
