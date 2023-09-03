use crate::{
    filesys::{config::get_config_values, get_app_path},
    utils::log::{LogLevel, LOG},
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Serialize, Deserialize, Clone)]
pub struct RequestResponse {
    request: RequestInfo,
    response: ResponseInfo,
    time: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestInfo {
    method: String,
    url: String,
    data: Option<String>,
    headers: Option<Vec<HeaderItem>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseInfo {
    result: String,
    status_code: u16,
    headers: Vec<HeaderItem>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HeaderItem {
    name: String,
    value: String,
}

pub async fn make_http_request(
    method: &str,
    url: &str,
    data: Option<&str>,
    headers: Option<Vec<(&str, &str)>>,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut request_builder = match method {
        "GET" => client.get(url),
        "PUT" => client.put(url),
        "OPTIONS" => client.request(reqwest::Method::OPTIONS, url),
        "HEAD" => client.request(reqwest::Method::HEAD, url),
        "PATCH" => client.patch(url),
        "POST" => client.post(url),
        "DELETE" => client.delete(url),
        "TRACE" => client.request(reqwest::Method::TRACE, url),
        _ => panic!("Invalid HTTP method"),
    };

    let data_str = data.map(|d| d.to_owned());

    if let Some(data) = data {
        if let Ok(json_value) = serde_json::from_str::<&str>(data) {
            let body_string = json_value.to_string();
            request_builder = request_builder
                .header(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .body(body_string);
        } else {
            request_builder = request_builder.body(data.to_owned());
        }
    }

    let mut headers_info = None;
    if let Some(headers) = headers {
        let mut header_map = HeaderMap::new();
        for (header_name, header_value) in &headers {
            header_map.insert(
                HeaderName::from_lowercase(header_name.to_lowercase().as_bytes()).unwrap(),
                HeaderValue::from_str(header_value).unwrap(),
            );
        }
        request_builder = request_builder.headers(header_map);
        headers_info = Some(
            headers
                .iter()
                .map(|(name, value)| HeaderItem {
                    name: name.to_string(),
                    value: value.to_string(),
                })
                .collect(),
        );
    }
    LOG(
        LogLevel::Info,
        "make_http_request",
        format!("Sending HTTP request: Method:'{}', Url:'{}'", method, url),
    );
    let start_time = std::time::Instant::now();
    let response = request_builder.send().await.map_err(|e| e.to_string())?;
    let end_time = std::time::Instant::now();
    let elapsed_time_ms = end_time.duration_since(start_time).as_millis();

    let status_code = response.status().as_u16();
    let response_headers = response.headers().clone();
    let response_text = response.text().await.map_err(|e| e.to_string())?;

    let request_info = RequestInfo {
        method: method.to_string(),
        url: url.to_string(),
        data: data_str,
        headers: headers_info,
    };

    let response_info = ResponseInfo {
        result: response_text,
        status_code,
        headers: response_headers
            .iter()
            .map(|(name, value)| HeaderItem {
                name: name.as_str().to_string(),
                value: value.to_str().unwrap().to_string(),
            })
            .collect(),
    };

    let request_response = RequestResponse {
        request: request_info,
        response: response_info,
        time: elapsed_time_ms,
    };

    let _ = append_to_history(request_response.clone());

    let json_result = serde_json::to_string(&request_response).map_err(|e| e.to_string())?;

    Ok(json_result)
}

// const MAX_HISTORY_ENTRIES: usize = 100;

pub fn append_to_history(
    data: RequestResponse,
) -> Result<(), crate::filesys::history::HistoryError> {
    let app_path = get_app_path();
    let file_path = app_path.join("histories").join("http.json");

    // Read existing history from the file
    let existing_history_str = fs::read_to_string(&file_path)?;
    let mut existing_history: Vec<RequestResponse> = serde_json::from_str(&existing_history_str)?;

    // Remove oldest entries if the history exceeds the maximum size
    let config_values = get_config_values();
    let max_history_entries: usize;
    if let Some(save_amount) = config_values["http"]["history"]["save_amount"].as_u64() {
        if save_amount <= usize::MAX as u64 {
            max_history_entries = save_amount as usize;
        } else {
            max_history_entries = 100;
        }
    } else {
        max_history_entries = 100;
    }

    if existing_history.len() >= max_history_entries {
        let num_entries_to_remove = existing_history.len() - max_history_entries + 1;
        existing_history.drain(0..num_entries_to_remove);
    }

    // Push the new data to the history
    existing_history.push(data);

    // Serialize and write back to the file
    let history_json = serde_json::to_string_pretty(&existing_history)?;
    fs::write(&file_path, history_json)?;

    Ok(())
}

pub fn read_history_file() -> Result<String, String> {
    let app_path = get_app_path();
    let file_path = app_path.join("histories").join("http.json");

    if !file_path.exists() {
        crate::filesys::history::create_histories();
    }

    // Read existing history from the file
    let existing_history_str = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let mut existing_history: Vec<RequestResponse> =
        serde_json::from_str(&existing_history_str).map_err(|e| e.to_string())?;

    // Reverse the history vector to have newer requests at the top
    existing_history.reverse();

    // Serialize the updated history back to a JSON string
    let updated_history_str =
        serde_json::to_string_pretty(&existing_history).map_err(|e| e.to_string())?;

    Ok(updated_history_str)
}

pub fn clear_history_file() -> Result<(), String> {
    let app_path = get_app_path();
    let file_path = app_path.join("histories").join("http.json");

    // Create an empty history
    let empty_history: Vec<RequestResponse> = Vec::new();

    // Serialize the empty history to a JSON string
    let empty_history_str =
        serde_json::to_string_pretty(&empty_history).map_err(|e| e.to_string())?;

    // Write the empty history to the file, effectively clearing its content
    fs::write(&file_path, empty_history_str).map_err(|e| e.to_string())?;

    Ok(())
}
