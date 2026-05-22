mod en;
mod fr;
mod de;
mod es;
mod pt;
mod it;
mod nl;
mod pl;
mod ru;
mod uk;
mod tr;
mod ar;
mod zh;
mod ja;
mod ko;
mod cs;
mod sv;
mod da;
mod fi;
mod nb;
mod hu;
mod ro;

use std::collections::HashMap;
use crate::helper::{load_config};

pub struct I18n {
  pub strings : HashMap<&'static str, String>,
}

impl I18n {
  pub fn new (lang : &str)->Self {
    let config = load_config();
    let app_name = config.app_name;

    let lang = lang.split('-').next().unwrap_or("en");

    let strings = match lang{
        "fr" => fr::strings(&app_name),
        "de" => de::strings(&app_name),
        "es" => es::strings(&app_name),
        "pt" => pt::strings(&app_name),
        "it" => it::strings(&app_name),
        "nl" => nl::strings(&app_name),
        "pl" => pl::strings(&app_name),
        "ru" => ru::strings(&app_name),
        "uk" => uk::strings(&app_name),
        "tr" => tr::strings(&app_name),
        "ar" => ar::strings(&app_name),
        "zh" => zh::strings(&app_name),
        "ja" => ja::strings(&app_name),
        "ko" => ko::strings(&app_name),
        "cs" => cs::strings(&app_name),
        "sv" => sv::strings(&app_name),
        "da" => da::strings(&app_name),
        "fi" => fi::strings(&app_name),
        "nb" | "nn" => nb::strings(&app_name),
        "hu" => hu::strings(&app_name),
        "ro" => ro::strings(&app_name),
        _ => en::strings(&app_name),
    };

    Self { strings }
  }

  pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
      self.strings.get(key).map(|s| s.as_str()).unwrap_or(key)
  }
}
