use crate::filesys::get_app_path;
use crate::request::http::RequestResponse;
use serde::{Deserialize, Serialize};
use serde_json::Error as JSONError;
use std::io::Error as IOError;
use std::{fs, path::PathBuf};

// Define an error type for your module
#[derive(Debug)]
pub enum HistoryError {
    IO(IOError),
    JSON(JSONError),
}

impl From<IOError> for HistoryError {
    fn from(error: IOError) -> Self {
        HistoryError::IO(error)
    }
}

impl From<JSONError> for HistoryError {
    fn from(error: JSONError) -> Self {
        HistoryError::JSON(error)
    }
}

#[derive(Serialize, Deserialize)]
struct HistoryData {
    requests: Vec<RequestResponse>,
}

pub fn create_histories() -> () {
    let app_path = get_app_path();
    let folder_path = app_path.join("histories");
    if !folder_path.exists() {
        let _ = fs::create_dir_all(folder_path.clone());
    }
    let mut paths: Vec<PathBuf> = Vec::new();
    paths.push(folder_path.join("http.json"));
    paths.push(folder_path.join("netio.json"));

    for path in paths {
        if !path.exists() {
            let empty_history: Vec<RequestResponse> = Vec::new();
            let history_json = serde_json::to_string_pretty(&empty_history).map_err(|e| e);
            let _ = fs::write(&path, history_json.unwrap());
        }
    }
}


