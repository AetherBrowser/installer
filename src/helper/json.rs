use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub app_name: String,
    pub json_config_url: String,
}

pub fn load_config() -> Config {
    let data = include_str!("../../config.json");
    serde_json::from_str(data).expect("Invalid config.json")
}
