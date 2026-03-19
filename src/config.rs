use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub db_path: String,
}

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("No home dir");
    path.push(".bill_config.json");
    path
}

pub fn save_config(config: &Config) {
    let path = get_config_path();
    let data = serde_json::to_string_pretty(config).unwrap();
    fs::write(path, data).unwrap();
}

pub fn load_config() -> Option<Config> {
    let path = get_config_path();
    if !path.exists() {
        return None;
    }

    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}
