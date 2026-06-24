use crate::filesys::get_app_path;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method};
use serde::{Deserialize, Deserializer, Serialize};
use std::fs;
use std::time::Instant;
use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};

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

#[derive(Serialize, Clone)]
pub struct ResponseInfo {
    bytes: String,
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
) -> Result<RequestResponse, String> {
    let client = Client::new();
    let mut request_builder = match method {
        "GET"     => client.get(url),
        "PUT"     => client.put(url),
        "OPTIONS" => client.request(Method::OPTIONS, url),
        "HEAD"    => client.request(Method::HEAD, url),
        "PATCH"   => client.patch(url),
        "POST"    => client.post(url),
        "DELETE"  => client.delete(url),
        "TRACE"   => client.request(Method::TRACE, url),
        _ => panic!("Invalid HTTP method"),
    };

    let data_str = data.map(|d| d.to_owned());

    if let Some(data) = data {
        request_builder = request_builder.body(data.to_owned());
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

    // TIMER START
    let start_time = Instant::now();
    let response = request_builder.send().await.map_err(|e| e.to_string())?;

    let status_code = response.status().as_u16();
    let response_headers = response.headers().clone();
    let response_bytes = response.bytes().await.map_err(|e| e.to_string())?;

    // TIMER END
    let end_time = Instant::now();
    let elapsed_time_ms = end_time.duration_since(start_time).as_millis();

    let request_info = RequestInfo {
        method: method.to_string(),
        url: url.to_string(),
        data: data_str,
        headers: headers_info,
    };

    let response_info = ResponseInfo {
        bytes: BASE64_STANDARD.encode(response_bytes),
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

    if let Err(err) = append_to_history(request_response.clone()) {
        eprintln!("Error while saving HTTP history {:?}", err);
    }

    Ok(request_response)
}

const MAX_HISTORY_ENTRIES: usize = 100;

pub fn append_to_history(
    data: RequestResponse,
) -> Result<(), crate::filesys::history::HistoryError> {
    let app_path = get_app_path();
    let file_path = app_path.join("histories").join("http.json");

    // Read existing history from the file
    let existing_history_str = fs::read_to_string(&file_path)?;
    let mut existing_history: Vec<RequestResponse> = serde_json::from_str(&existing_history_str)?;

    // Remove oldest entries if the history exceeds the maximum size
    if existing_history.len() >= MAX_HISTORY_ENTRIES {
        let num_entries_to_remove = existing_history.len() - MAX_HISTORY_ENTRIES + 1;
        existing_history.drain(0..num_entries_to_remove);
    }

    // Push the new data to the history
    existing_history.push(data);

    // Serialize and write back to the file
    let history_json = serde_json::to_string(&existing_history)?;
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
        serde_json::to_string(&existing_history).map_err(|e| e.to_string())?;

    Ok(updated_history_str)
}

pub fn clear_history_file() -> Result<(), String> {
    let app_path = get_app_path();
    let file_path = app_path.join("histories").join("http.json");

    // Create an empty history
    let empty_history: Vec<RequestResponse> = Vec::new();

    // Serialize the empty history to a JSON string
    let empty_history_str =
        serde_json::to_string(&empty_history).map_err(|e| e.to_string())?;

    // Write the empty history to the file, effectively clearing its content
    fs::write(&file_path, empty_history_str).map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Deserialize)]
struct RawResponseInfo {
    #[serde(default)]
    bytes: Option<String>,
    #[serde(default)]
    result: Option<String>,

    status_code: u16,
    headers: Vec<HeaderItem>,
}

impl<'de> Deserialize<'de> for ResponseInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawResponseInfo::deserialize(deserializer)?;
        Ok(ResponseInfo {
            bytes: raw.bytes.unwrap_or_else(|| {
                BASE64_STANDARD.encode(raw.result.unwrap_or_default().into_bytes())
            }),
            status_code: raw.status_code,
            headers: raw.headers,
        })
    }
}
