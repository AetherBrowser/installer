use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("{} Yükleyici", app_name)),
    ("loading", "Yükleniyor...".into()),
    ("license_title", "Lisans Sözleşmesi".into()),
    ("license_subtitle",
     "Devam etmeden önce lütfen aşağıdaki önemli bilgileri okuyun".into()),
    ("license_body",
     "Lütfen aşağıdaki lisans sözleşmesini dikkatlice okuyun. Kuruluma devam \
     etmeden önce bu sözleşmenin koşullarını kabul etmeniz gerekmektedir.".into()),
    ("accept", "Kabul Et".into()),
    ("decline", "Reddet".into()),
    ("downloading", "İndiriliyor...".into()),
    ("verifying", "Doğrulanıyor...".into()),
    ("download_ok", "İndirme tamamlandı".into()),
    ("finished_title", "Kurulum tamamlandı".into()),
    ("failed_title", "Kurulum başarısız".into()),
    ("close", "Kapat".into()),
    ("trying", "Bağlanılıyor".into()),
    ("err_checksum", "Sağlama toplamı hatası".into()),
    ("err_all_failed", "Tüm sunucular başarısız".into()),
    ("err_no_length", "Bilinmeyen boyut".into()),
  ])
}
