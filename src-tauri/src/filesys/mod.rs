use std::{fs, path::PathBuf};
pub mod config;
pub mod history;

pub fn get_app_path() -> PathBuf {
    if cfg!(windows) {
        match std::env::var("APPDATA") {
            Ok(appdata) => {
                let mut path = PathBuf::from(appdata);
                path.push("oxy-requester");
                path
            }
            Err(_) => panic!("Failed to get the %APPDATA% environment variable."),
        }
    } else {
        let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
        path.push(".local");
        path.push("oxy-requester");
        path
    }
}

pub fn check_app_path() -> () {
    let app_path = get_app_path();
    if !app_path.exists() {
        println!("Creating app path at {:?}", app_path);
        if let Err(e) = fs::create_dir_all(app_path) {
            println!("Failed to create app path: {}", e);
        }
    }
}
