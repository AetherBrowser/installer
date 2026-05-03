use std::collections::HashMap;
pub fn strings(app_name: &str) -> HashMap<&'static str, String> {

  HashMap::from([
    ("app_title", format!("مثبت {}", app_name)),
    ("loading", "جارٍ التحميل...".into()),
    ("license_title", "اتفاقية الترخيص".into()),
    ("license_subtitle", "يرجى قراءة المعلومات المهمة التالية قبل المتابعة".into()),
    ("license_body", "يرجى قراءة اتفاقية الترخيص التالية بعناية. يجب عليك قبول \
                     شروط هذه الاتفاقية قبل متابعة التثبيت.".into()),
    ("accept", "قبول".into()),
    ("decline", "رفض".into()),
    ("downloading", "جارٍ التنزيل...".into()),
    ("verifying", "جارٍ التحقق...".into()),
    ("download_ok", "اكتمل التنزيل".into()),
    ("finished_title", "اكتمل التثبيت".into()),
    ("failed_title", "فشل التثبيت".into()),
    ("close", "إغلاق".into()),
    ("trying", "الاتصال بـ".into()),
    ("err_checksum", "خطأ في المجموع الاختباري".into()),
    ("err_all_failed", "فشلت جميع الخوادم".into()),
    ("err_no_length", "حجم غير معروف".into()),
  ])
}
