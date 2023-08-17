use crate::filesys::get_app_path;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

fn get_default_config() -> Value {
    json!({
        "version": "1.1.2",
        "defaults": {
            "queries": [],
            "headers": [
                {
                    "name": "user-agent",
                    "value": "Mozilla/5.0 (Android 13; Mobile; rv:109.0) Gecko/113.0 Firefox/113.0"
                }
            ],
            "body": "",
        },
        "pages": {
            "http": "Create http request",
            "netio": "Create Tcp/Udp request"
        },
        "css": ""
    })
}

pub fn get_config_path() -> PathBuf {
    get_app_path().join("config.json")
}

pub fn create_config() {
    let config_path = get_config_path();
    if !config_path.exists() {
        let default_config = get_default_config();
        let config_content = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize default config");
        fs::write(&config_path, config_content).expect("Failed to write config file");
        println!("Default config file created at {:?}", config_path);
    } else {
        println!("Config file already exists at {:?}", config_path);
    }
}

pub fn read_config_file() -> String {
    let config_path = get_config_path();
    fs::read_to_string(&config_path).expect("Failed to read config file")
}
pub fn write_config_file(config: &str) -> Result<(), String> {
    let config_path = get_config_path();
    fs::write(&config_path, config).map_err(|e| e.to_string())?;
    Ok(())
}
