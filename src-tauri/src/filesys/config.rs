use crate::filesys::get_app_path;
use crate::utils::log::LogLevel;
use crate::utils::log::LOG;
use crate::utils::open_file;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_default_config() -> Value {
    const DEFAULT_CONFIG: &str = r#"
    {
        "version": "1.1.4",
        "http": {
          "defaults": {
            "url": "",
            "method": "GET",
            "queries": [],
            "headers": [
              {
                "name": "user-agent",
                "value": "Mozilla/5.0 (Android 13; Mobile; rv:109.0) Gecko/113.0 Firefox/113.0"
              }
            ],
            "body": ""
          },
          "history": {
            "save_amount": 100
          }
        },
        "pages": {
          "http": "Create http request",
          "netio": "Create Tcp/Udp request"
        },
        "css": ""
    }
    "#;

    serde_json::from_str(DEFAULT_CONFIG).expect("Failed to parse default config")
}

pub fn check_app_version() {
    let mut _json_content = read_config_file();

    let mut parsed_content: Value = serde_json::from_str(&_json_content)
        .expect("[check_app_version] FATAL: Failed to parse JSON content");

    let default_config = get_default_config();
    merge_json_values(&mut parsed_content, &default_config);

    if let Value::Object(obj) = &mut parsed_content {
        obj.insert("version".to_string(), json!(env!("CARGO_PKG_VERSION")));
    }

    LOG(
        LogLevel::Debug,
        "check_app_version",
        format!(
            "Changing version in config to: {}",
            env!("CARGO_PKG_VERSION")
        ),
    );

    let modified_content = serde_json::to_string_pretty(&parsed_content)
        .expect("[check_app_version] FATAL: Failed to serialize modified JSON content");

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
        LOG(
            LogLevel::Debug,
            "create_config",
            format!("Default config file created at {:?}", config_path),
        );
    } else {
        LOG(
            LogLevel::Debug,
            "create_config",
            format!("Config file already exists at {:?}", config_path),
        );
    }
}

pub fn get_config_values() -> Value {
    let str_config = read_config_file();
    serde_json::from_str(&str_config).expect("Failed to get config values")
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

fn merge_json_values(target: &mut Value, defaults: &Value) {
    match (target, defaults) {
        (Value::Object(target_obj), Value::Object(default_obj)) => {
            for (key, default_value) in default_obj.iter() {
                if !target_obj.contains_key(key) {
                    target_obj.insert(key.clone(), default_value.clone());
                } else {
                    let target_value = target_obj.get_mut(key).unwrap();
                    merge_json_values(target_value, default_value);
                }
            }
        }
        (target_array @ Value::Array(_), defaults_array @ Value::Array(_)) => {
            if target_array.as_array().unwrap().is_empty() {
                *target_array = defaults_array.clone();
            } else {
                for (index, default_value) in defaults_array.as_array().unwrap().iter().enumerate()
                {
                    if index >= target_array.as_array().unwrap().len() {
                        target_array
                            .as_array_mut()
                            .unwrap()
                            .push(default_value.clone());
                    } else {
                        let target_value = &mut target_array.as_array_mut().unwrap()[index];
                        merge_json_values(target_value, default_value);
                    }
                }
            }
        }
        _ => {
            // Keep target value as is
        }
    }
}

pub fn open_config_in_editor() {
    open_file(get_config_path().to_str().unwrap());
}
