use std::{path::PathBuf, fs};
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
        let mut path = PathBuf::from("/etc");
        path.push("oxy-requester");
        path
    }
}

pub fn check_app_path() -> () {
    let app_path = get_app_path();
    if !app_path.exists() {
        let _ = fs::create_dir_all(app_path);
    }
}
