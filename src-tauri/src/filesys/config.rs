use crate::filesys::get_app_path;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_default_config() -> Value {
    json!({
        "version": "1.1.3",
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

pub fn check_app_version() {
    let mut _json_content = read_config_file();

    let mut parsed_content: Value =
        serde_json::from_str(&_json_content).expect("Failed to parse JSON content");


    if let Value::Object(obj) = &mut parsed_content {
        obj.insert("version".to_string(), json!(env!("CARGO_PKG_VERSION")));
    }

    println!("Changing version in config to: {}", env!("CARGO_PKG_VERSION"));

    let modified_content = serde_json::to_string_pretty(&parsed_content)
        .expect("Failed to serialize modified JSON content");

    let _ee = write_config_file(&modified_content);
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
