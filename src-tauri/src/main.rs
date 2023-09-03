#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod filesys;
mod request;
mod utils;

use filesys::config::open_config_in_editor;
use utils::log::{LogLevel, LOG};

use crate::filesys::config::{create_config, get_config_path, read_config_file, write_config_file};
use crate::request::http::{self, make_http_request};
use crate::utils::on_start;

#[tauri::command]
fn get_config_values() -> String {
    if get_config_path().exists() {
        read_config_file()
    } else {
        create_config();
        read_config_file()
    }
}

#[tauri::command]
async fn app_loaded() -> Result<(), String> {
    LOG(LogLevel::Debug, "app_loaded", "App loaded!".to_string());
    Ok(())
}

#[tauri::command]
fn save_config(args: &str) -> Result<(), String> {
    Ok(write_config_file(args).unwrap())
}
#[tauri::command]
async fn send_request(
    method: &str,
    url: &str,
    data: Option<&str>,
    headers: Option<Vec<(&str, &str)>>,
) -> Result<String, String> {
    make_http_request(method, url, data, headers)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn send_netio_req(
    protocol: String,
    host: String,
    port: u16,
    data: String,
) -> Result<String, String> {
    let options = crate::request::netio::RequestOptions {
        protocol: protocol,
        host: host.clone(),
        port: port,
        data: data.as_bytes().to_vec(),
    };

    let result = tokio::spawn(async move { crate::request::netio::create_request(options).await })
        .await
        .map_err(|e| e.to_string())?;

    match result {
        Ok(result) => Ok(result),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
fn read_http_history() -> Result<String, String> {
    Ok(http::read_history_file().unwrap())
}

#[tauri::command]
fn clear_http_history() -> Result<(), String> {
    Ok(http::clear_history_file().unwrap())
}

#[tauri::command]
fn open_config() -> Result<(), String> {
    open_config_in_editor();
    Ok(())
}

fn main() {
    on_start();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_loaded,
            send_request,
            send_netio_req,
            get_config_values,
            save_config,
            read_http_history,
            clear_http_history,
            open_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
